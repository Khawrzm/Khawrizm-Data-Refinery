#!/usr/bin/env python3
"""
Ring-0 Extractor v1.2
Zero-dependency stdlib text extraction engine for sovereign data refinery.
Handles PDF (zlib+re), DOCX/XLSX/PPTX (zip+xml), HTML, logs, code, junk.
Aggressive cleaning: terminal noise, PDF kerning, Arabic RTL reversal.
v1.2: Added ProcessPoolExecutor parallel directory processing for ARM64/RISC-V multi-core throughput.
"""

import sys
import os
import re
import zipfile
import xml.etree.ElementTree as ET
import zlib
import html.parser
import concurrent.futures

def clean_terminal_noise(text):
    text = re.sub(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])', '', text)
    text = re.sub(r'(?m)^\s*(?:kali㎟\S*\s*|PS .*?>\s*|C:\\.*?>|\$\s*|#\s*)\s*', '', text)
    text = re.sub(r'\[\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}(:\d{2})?\]', '', text)
    text = re.sub(r'[ \t]+', ' ', text)
    text = re.sub(r'\n{3,}', '\n\n', text)
    return text.strip()

def fix_pdf_kerning(text):
    for _ in range(6):
        text = re.sub(r'(\b[A-Za-z0-9])\s+([A-Za-z0-9]\b)', r'\1\2', text)
    def join_spaced_letters(m):
        return ''.join(m.group(0).split())
    text = re.sub(r'(?<=[\W^])([A-Za-z0-9]\s+){2,}[A-Za-z0-9](?=[\W$])', join_spaced_letters, text)
    return text

def fix_arabic_reversal(text):
    arabic_pattern = re.compile(r'[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]+')
    def reverse_arabic(m):
        return m.group(0)[::-1]
    return arabic_pattern.sub(reverse_arabic, text)

def extract_text_from_pdf(filepath):
    try:
        with open(filepath, 'rb') as f:
            data = f.read()
        text_parts = []
        stream_pattern = re.compile(rb'stream\s*\r?\n(.*?)\r?\nendstream', re.DOTALL)
        for match in stream_pattern.finditer(data):
            stream_data = match.group(1)
            try:
                decompressed = zlib.decompress(stream_data)
                tj_texts = re.findall(rb'\(([^)]*)\)\s*Tj', decompressed)
                for t in tj_texts:
                    text_parts.append(t.decode('latin-1', errors='ignore'))
                tj_arrays = re.findall(rb'\[([^\]]*)\]\s*TJ', decompressed)
                for arr in tj_arrays:
                    inner = re.findall(rb'\(([^)]*)\)', arr)
                    for t in inner:
                        text_parts.append(t.decode('latin-1', errors='ignore'))
            except zlib.error:
                try:
                    tj_texts = re.findall(rb'\(([^)]*)\)\s*Tj', stream_data)
                    for t in tj_texts:
                        text_parts.append(t.decode('latin-1', errors='ignore'))
                except:
                    pass
        if not text_parts:
            all_tj = re.findall(rb'\(([^)]{3,})\)\s*Tj', data)
            for t in all_tj:
                text_parts.append(t.decode('latin-1', errors='ignore'))
        raw_text = ' '.join(text_parts)
        raw_text = fix_pdf_kerning(raw_text)
        raw_text = fix_arabic_reversal(raw_text)
        return raw_text
    except Exception as e:
        return f"[PDF EXTRACTION ERROR: {str(e)}]"

def extract_text_from_docx(filepath):
    try:
        with zipfile.ZipFile(filepath) as z:
            with z.open('word/document.xml') as xml_file:
                tree = ET.parse(xml_file)
                root = tree.getroot()
                ns = {'w': 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'}
                texts = [t.text for t in root.findall('.//w:t', ns) if t.text]
                return ' '.join(texts)
    except Exception as e:
        return f"[DOCX EXTRACTION ERROR: {str(e)}]"

def extract_text_from_xlsx(filepath):
    try:
        with zipfile.ZipFile(filepath) as z:
            shared_strings = []
            if 'xl/sharedStrings.xml' in z.namelist():
                with z.open('xl/sharedStrings.xml') as f:
                    tree = ET.parse(f)
                    root = tree.getroot()
                    ns = {'main': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main'}
                    for si in root.findall('.//main:si', ns):
                        t = si.find('.//main:t', ns)
                        if t is not None and t.text:
                            shared_strings.append(t.text)
            texts = []
            sheet_files = [name for name in z.namelist() if name.startswith('xl/worksheets/sheet') and name.endswith('.xml')]
            for sheet_file in sheet_files:
                with z.open(sheet_file) as f:
                    tree = ET.parse(f)
                    root = tree.getroot()
                    ns = {'main': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main'}
                    for row in root.findall('.//main:row', ns):
                        row_texts = []
                        for cell in row.findall('.//main:c', ns):
                            cell_type = cell.get('t')
                            val = cell.find('.//main:v', ns)
                            if val is not None and val.text:
                                if cell_type == 's' and shared_strings:
                                    try:
                                        idx = int(val.text)
                                        if idx < len(shared_strings):
                                            row_texts.append(shared_strings[idx])
                                    except:
                                        pass
                                else:
                                    row_texts.append(val.text)
                        if row_texts:
                            texts.append(' | '.join(row_texts))
            return '\n'.join(texts)
    except Exception as e:
        return f"[XLSX EXTRACTION ERROR: {str(e)}]"

def extract_text_from_pptx(filepath):
    try:
        with zipfile.ZipFile(filepath) as z:
            texts = []
            slide_files = [name for name in z.namelist() if 'ppt/slides/slide' in name and name.endswith('.xml')]
            ns = {
                'a': 'http://schemas.openxmlformats.org/drawingml/2006/main',
                'p': 'http://schemas.openxmlformats.org/presentationml/2006/main'
            }
            for slide in slide_files:
                with z.open(slide) as f:
                    tree = ET.parse(f)
                    root = tree.getroot()
                    for t in root.findall('.//a:t', ns):
                        if t.text:
                            texts.append(t.text)
            return ' '.join(texts)
    except Exception as e:
        return f"[PPTX EXTRACTION ERROR: {str(e)}]"

def extract_text_from_html(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        class TextExtractor(html.parser.HTMLParser):
            def __init__(self):
                super().__init__()
                self.texts = []
            def handle_data(self, data):
                self.texts.append(data.strip())
        parser = TextExtractor()
        parser.feed(content)
        return ' '.join([t for t in parser.texts if t])
    except Exception as e:
        return f"[HTML EXTRACTION ERROR: {str(e)}]"

def extract_text_from_plain(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            return f.read()
    except Exception as e:
        return f"[PLAIN EXTRACTION ERROR: {str(e)}]"

def process_file(filepath):
    ext = os.path.splitext(filepath)[1].lower()
    if ext == '.pdf':
        text = extract_text_from_pdf(filepath)
    elif ext in ['.docx', '.doc']:
        text = extract_text_from_docx(filepath)
    elif ext in ['.xlsx', '.xls']:
        text = extract_text_from_xlsx(filepath)
    elif ext == '.pptx':
        text = extract_text_from_pptx(filepath)
    elif ext in ['.html', '.htm']:
        text = extract_text_from_html(filepath)
    else:
        text = extract_text_from_plain(filepath)
    text = clean_terminal_noise(text)
    text = fix_pdf_kerning(text)
    text = fix_arabic_reversal(text)
    return text

def process_directory_parallel(directory):
    import glob
    files = []
    patterns = ('**/*.pdf', '**/*.docx', '**/*.doc', '**/*.xlsx', '**/*.xls', '**/*.pptx',
                '**/*.html', '**/*.htm', '**/*.txt', '**/*.log', '**/*.md', '**/*.py',
                '**/*.sh', '**/*.json', '**/*.xml', '**/*.csv')
    for pat in patterns:
        files.extend(glob.glob(os.path.join(directory, pat), recursive=True))
    files = sorted(set(f for f in files if os.path.isfile(f)))
    print(f"[Ring-0] Parallel extraction of {len(files)} files using ProcessPoolExecutor on {os.cpu_count()} cores", file=sys.stderr)
    results = []
    with concurrent.futures.ProcessPoolExecutor(max_workers=min(32, os.cpu_count() or 4)) as executor:
        future_to_file = {executor.submit(process_file, f): f for f in files}
        for future in concurrent.futures.as_completed(future_to_file):
            f = future_to_file[future]
            try:
                res = future.result()
                results.append((f, res))
            except Exception as exc:
                results.append((f, f"[PARALLEL ERROR: {exc}]"))
    results.sort(key=lambda x: x[0])
    for f, res in results:
        print(f"=== BEGIN {f} ===")
        print(res)
        print("=== END ===\n")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 ring0_extractor.py <filepath|directory>")
        sys.exit(1)
    path = sys.argv[1]
    if os.path.isdir(path):
        process_directory_parallel(path)
    elif os.path.isfile(path):
        result = process_file(path)
        print(result)
    else:
        print(f"Error: {path} not found")
        sys.exit(1)
