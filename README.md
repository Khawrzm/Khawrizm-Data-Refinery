# KH AWRIZM DATA REFINERY

**Document Classification:** RING-0 SOVEREIGN INFRASTRUCTURE
**Component Designation:** Ring-0 Master Synthesizer
**Version:** 1.1
**Date:** 2026-06-30
**Architect:** Sulaiman Alshammari / KHAWRIZM Forensic Labs

## 1. PURPOSE

This repository implements the zero-telemetry, air-gapped "Sovereign Data Grinder" pipeline. It ingests arbitrary file formats (PDF, DOCX, XLSX, PPTX, HTML, logs, code, junk) and produces a single, highly structured Markdown artifact (`Master_Ring0.md`) optimized for NotebookLM ingestion and local LLM synthesis. v1.1 introduces fully local inference path and cryptographic data provenance.

## 2. ARCHITECTURAL COMPONENTS

### 2.1 ring0_extractor.py
- **Context:** Ring-0 offline extraction engine
- **Dependencies:** Python 3 standard library exclusively (zipfile, xml.etree.ElementTree, zlib, re, html.parser)
- **Capabilities:**
  - PDF: zlib stream decompression + regex Tj/TJ text extraction with fallback
  - Office (DOCX/XLSX/PPTX): native zip+xml parsing of document.xml, sharedStrings, slides
  - HTML: html.parser data extraction
  - Generic: direct text read for logs/code/junk
  - Post-process: aggressive regex removal of terminal/PowerShell/Kali prompts, ANSI codes, timestamps; PDF kerning repair (spaced letters); Arabic RTL run reversal correction

### 2.2 api_sanitizer.py
- **Context:** Deterministic LLM structuring bridge (external)
- **Dependencies:** Python stdlib (json, urllib.request) + GROK_API_KEY / XAI_API_KEY / OPENAI_API_KEY
- **Mechanism:** Text is chunked (~3200 chars). Each chunk submitted to xAI Grok endpoint under system prompt enforcing EXACT JSON schema. response_format=json_object + temperature=0.0. Non-conforming or conversational responses are dropped. Valid sections are aggregated into hierarchical Markdown.
- **Schema Enforced:** section_title, executive_summary, entities[], structured_markdown (NotebookLM-optimized), tags[]

### 2.3 airgapped_sanitizer.py (NEW v1.1)
- **Context:** Local-first, zero-external-network alternative
- **Dependencies:** Python stdlib (json, urllib.request) + running local inference engine (Ollama recommended on 127.0.0.1:11434)
- **Mechanism:** Identical chunking and JSON schema enforcement as api_sanitizer.py. Routes to http://127.0.0.1:11434/api/chat (or OLLAMA_HOST) with "format": "json" and temperature=0.0. No bytes leave the host. Model selectable via OLLAMA_MODEL env var (default: llama3).
- **Schema Enforced:** Identical strict contract (section_title, executive_summary, entities[], structured_markdown, tags[])

### 2.4 grinder_pipeline.sh
- **Context:** Kali Linux recursive orchestration harness
- **Operation:** find(1) recursion over target directory → ring0_extractor.py → pipe → selected sanitizer (stdin) → append to Master_Ring0.md
- **v1.1 Controls:**
  - `--airgap` | `--local` flag: dynamically selects airgapped_sanitizer.py (local Ollama) instead of external API
  - Automatic GPG detached signature generation on completion (`Master_Ring0.md.sig`)
- **Controls:** set -euo pipefail, stderr progress, automatic chmod, size reporting, exclusion of scripts and output file

## 3. OPERATIONAL MANDATE

Extraction phase: fully offline, zero external dependencies or telemetry.
Sanitization phase (airgap mode): machine-enforced JSON contract via local inference engine; zero network egress.
Result: Master_Ring0.md is the canonical, chunkable, NotebookLM-ready sovereign knowledge base. Cryptographic signature provides tamper-evident provenance aligned with OpenSSF secure supply chain practices.

## 4. SECURITY POSTURE
- Zero telemetry in extraction
- Air-gapped core processing (optional full-local inference path)
- Strict JSON schema boundary at LLM interface (local or remote)
- Full local sovereignty; no raw data leaves controlled environment
- Cryptographic signing of final artifact for verifiable chain of custody

## 5. EXECUTION PARAMETERS (v1.1)

```bash
# Standard (external API path)
./grinder_pipeline.sh /path/to/raw_data

# Fully air-gapped local inference (requires Ollama running)
export OLLAMA_MODEL=llama3.1
export OLLAMA_HOST=127.0.0.1:11434
./grinder_pipeline.sh --airgap /path/to/raw_data

# Verify cryptographic provenance after run
 gpg --verify Master_Ring0.md.sig Master_Ring0.md
```

**End of Ring-0 Master Synthesizer Architectural Specification (v1.1)**