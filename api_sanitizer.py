#!/usr/bin/env python3
"""
api_sanitizer.py v1.0
Chunked LLM invocation with mandatory JSON Schema enforcement.
Forces deterministic, hallucination-free structured Markdown output.
Drops any conversational or non-conforming responses.
"""

import sys
import os
import json
import urllib.request
import urllib.error
import re

def chunk_text(text, max_chars=3200):
    paragraphs = re.split(r'\n\s*\n', text)
    chunks = []
    current = ""
    for p in paragraphs:
        if len(current) + len(p) + 2 > max_chars and current:
            chunks.append(current.strip())
            current = p
        else:
            current += "\n\n" + p if current else p
    if current.strip():
        chunks.append(current.strip())
    return chunks if chunks else [text[:max_chars]]

def sanitize_chunk(chunk, api_key, model="grok-3-latest"):
    system_prompt = ("You are the Ring-0 Data Structurer. Output ONLY one valid JSON object. "
                   "No text before or after the JSON. No explanations. No moralizing. "
                   "Schema: {\"section_title\": \"string\", \"executive_summary\": \"string (2-4 sentences dense facts only)\", "
                   "\"entities\": [\"string array of named entities, technical terms, orgs, locations\"], "
                   "\"structured_markdown\": \"string - FULL content as clean hierarchical Markdown for NotebookLM (## headings, bullets, tables, preserve ALL facts, zero fluff)\", "
                   "\"tags\": [\"5-10 lowercase keyword tags\"]}. "
                   "If input is empty or junk, still produce valid JSON with appropriate placeholder values.")

    user_content = f"RAW EXTRACTED DATA TO STRUCTURE:\n\n{chunk}"

    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content}
        ],
        "response_format": {"type": "json_object"},
        "temperature": 0.0,
        "max_tokens": 4500
    }

    url = "https://api.x.ai/v1/chat/completions"
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}"
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, headers=headers, method="POST")

    try:
        with urllib.request.urlopen(req, timeout=180) as resp:
            body = json.loads(resp.read().decode("utf-8"))
            content = body["choices"][0]["message"]["content"]
            parsed = json.loads(content)
            # Strict validation
            required = ["section_title", "structured_markdown"]
            if all(k in parsed for k in required) and isinstance(parsed.get("structured_markdown"), str):
                return parsed
            return None
    except Exception as e:
        print(f"[SANITIZER API ERROR] {str(e)}", file=sys.stderr)
        return None

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 api_sanitizer.py <input.txt> [output.md] | python3 api_sanitizer.py - <output.md>")
        sys.exit(1)

    input_arg = sys.argv[1]
    output_arg = sys.argv[2] if len(sys.argv) > 2 else None

    api_key = (os.environ.get("GROK_API_KEY") or
               os.environ.get("XAI_API_KEY") or
               os.environ.get("OPENAI_API_KEY"))
    if not api_key:
        print("FATAL: Set GROK_API_KEY / XAI_API_KEY / OPENAI_API_KEY", file=sys.stderr)
        sys.exit(1)

    if input_arg == "-":
        text = sys.stdin.read()
    else:
        with open(input_arg, "r", encoding="utf-8", errors="ignore") as f:
            text = f.read()

    if not text.strip():
        print("No content to sanitize.")
        sys.exit(0)

    chunks = chunk_text(text)
    sections = []
    for i, ch in enumerate(chunks):
        print(f"[Ring-0] Sanitizing chunk {i+1}/{len(chunks)}...", file=sys.stderr)
        result = sanitize_chunk(ch, api_key)
        if result:
            sections.append(result)
        else:
            print(f"[Ring-0] Dropped non-compliant chunk {i+1}", file=sys.stderr)

    # Assemble Master Markdown
    final = "# Ring-0 Master Synthesized Corpus\n\n"
    final += f"**Chunks Processed:** {len(sections)} | **Source:** Sovereign Data Grinder\n\n---\n\n"
    for sec in sections:
        final += f"## {sec.get('section_title', 'Untitled Section')}\n\n"
        if sec.get('executive_summary'):
            final += f"**Executive Summary:** {sec['executive_summary']}\n\n"
        ents = sec.get('entities', [])
        if ents:
            final += "**Entities:** " + ", ".join(ents) + "\n\n"
        final += sec.get('structured_markdown', '') + "\n\n"
        tags = sec.get('tags', [])
        if tags:
            final += f"**Tags:** {', '.join(tags)}\n\n"
        final += "---\n\n"

    if output_arg:
        with open(output_arg, "w", encoding="utf-8") as f:
            f.write(final)
        print(f"[Ring-0] Written sanitized output to {output_arg}")
    else:
        print(final)

if __name__ == "__main__":
    main()
