
####  **Chat History** 

** Location:** `~/.lmstudio/conversations/<session_id>.json` (macOS / Linux) and `%USERPROFILE%\.lmstudio\conversations\<session_id>.json` (Windows). 

- **Format:** Structured JSON. 

- **Forensic Value:** This is the crown jewel artifact. Each JSON file represents a single chat session and contains a complete, timestamped record of the conversation, including user prompts, AI responses, the model used, and configuration presets applied[64, 65, 66]. Analysis of these files allows for a near-perfect reconstruction of the users interactions. The format appears to be an internal data structure but is human-readable and programmatically parsable[64]. Importantly, the `<session_id>` component of the filename is a **Unix epoch timestamp encoded in milliseconds** , providing a precise creation timestamp for each session that can be correlated with external artefacts such as proxy logs, browser history, or Windows Event Logs during a DFIR investigation[64]. 

- **Volatility:** Persistent. These files remain until manually deleted. Due to their structured nature, they are highly recoverable from unallocated space. Note that LM Studio does not delete model folder 

13 

_6.2 LMStudio Artifacts_ 

remnants from the filesystem when a model is removed via the UI, so orphaned session directories may persist even after apparent user clean-up[36]. 

- **Model Cache** 

   - **Location:** The correct dual-path structure (Windows example) is: 

      - (or a user-configured alternative path) 

The equivalent on macOS / Linux replaces `%USERPROFILE%\` with `~/` . (Path varies slightly by OS, see Section 4.2.) 

   - **Format:** GGUF model weight files within a nested `<publisher>/<repo-name>` directory structure; the hub sub-tree contains small JSON manifests and configuration shards alongside the weight files. 

   - **Forensic Value:** The directory path itself ( `<publisher>/<repo-name>` ) provides valuable metadata about the models origin from the Hugging Face Hub, even if the user renames the `.gguf` file[54]. The split introduced in v0.3.16 means an investigator must examine _both_ sub-trees; hub metadata directories may persist on disk even after the GGUF weight file has been deleted[36]. 

   - **Volatility:** Persistent. 

- **Configuration Presets** 

   - **Location:** `~/.lmstudio/hub/presets/` (synced / community presets) or `~/.lmstudio/config-presets/` (user-defined presets). A drafts sub-directory at `~/.lmstudio/.internal/config-presets-drafts/` stores in-progress or unsaved preset configurations. 

   - **Format:** JSON ( `.preset.json` ). 

** Forensic Value:** These files store user-defined or downloaded model configurations, such as the system prompt, temperature, context length, and GPU offload settings[52]. They reveal how a user tailored a models behaviour for specific tasks, which can be indicative of intent. The presence of drafts in `config-presets-drafts/` may expose experimental or discarded configurations not visible in the main UI. 

   - **Volatility:** Persistent. 

- **Application and Server Logs** 

   - **Location:** Two distinct log sources exist: 

      1. **Persistent server logs:** `~/.lmstudio/server-logs/YYYY-MM/` : automatically written per calendar month. 

14 

_6.2 LMStudio Artifacts_ 

      2. **Live inference stream:** accessible via the CLI command `lms log stream` [30]: volatile unless redirected to a file. 

   - **Format:** Persistent logs are timestamped, structured text files; the live stream is a real-time text output[30]. 

   - **Forensic Value:** The persistent server logs record every API request served by LM Studios local HTTP server, including endpoint calls ( `/v1/chat/completions` , `/v1/models` , etc.), timestamps, model load/unload events, and client IP addresses if the server was accessed from other devices. The month-partitioned directory structure provides a direct timeline of application activity. The live inference stream reveals the _exact_ , fully formatted prompt sent to the inference engine _after_ prompt templating has been applied, which may differ from the raw user input recorded in the chat history  this distinction is significant for forensic reconstruction of model instructions[30]. 

   - **Volatility:** Server logs are **persistent** and month-partitioned. The live stream is highly volatile and is not persisted unless the user explicitly redirects output to a file. 

- **RAG Pipeline Cache** 

#### ** Location:** 

      -  `~/.lmstudio/.internal/retrieval-sessions/` : active RAG session state 

      -  `~/.lmstudio/.internal/cached-rag-pipeline-chunks/` : chunked and vectorised document representations 

      -  `~/.lmstudio/.internal/parsed-documents-cache/` : raw text extracted from uploaded files (PDF, DOCX, etc.) 

   - **Format:** Internal binary or serialised vector format (chunks); parsed document cache may contain extractable plain text. 

   - **Forensic Value:** When a user employs the Chat with Documents feature (Retrieval Augmented Generation, RAG [74]), LM Studio processes and caches documents across these three directories[55]. Analysis can reveal fragments or full copies of external documents the user was interacting with, even if the original documents have been deleted. The `parsed-documents-cache/` sub-directory is particularly valuable as it may contain human-readable extracted text. The bundled embedding model used for vectorisation ( `nomic-embed-text-v1.5-GGUF` , stored at `.internal/bundled-models/nomic-ai/` ) provides context for interpreting the chunk format. 

   - **Volatility:** Semi-persistent. It is a cache that can be cleared, but often persists across sessions and can grow to significant size with heavy document use. 

- **API Prediction History** 

15 

_6.2 LMStudio Artifacts_ 

- **Location:** `~/.lmstudio/.internal/api-prediction-history/packs/` 

- **Format:** Binary pack files; individual pack files can exceed 500 MB under heavy usage[10]. 

- **Forensic Value:** This directory captures every inference request processed by LM Studio, including _programmatic_ API calls made by external scripts, `curl` commands, or Python SDKs  artefacts that are _not_ recorded in the chat UIs conversation history. This makes it an indispensable source for detecting automated or scripted model usage beyond the graphical interface. 

- **Volatility:** Persistent. Pack files accumulate over time. A corrupted or oversized pack file is known to cause HTTP 500 errors on the local API server, which itself is a forensic indicator of sustained, high-volume API usage[10]. 

####  **Credentials Store** 

#### ** Location:** `~/.lmstudio/credentials/` and `~/.lmstudio/.internal/lms-key-2` [12] 

** Format:** Key files; may be stored in plaintext or lightly encoded form. The `lms-key-2` file stores the CLI authentication key used for LM Studio Hub access[12]. The `credentials/` directory may additionally contain tokens for integrated external services such as Hugging Face. 

- **Forensic Value:** Highest-sensitivity artefacts in the directory tree. These files should be examined for plaintext or base64-encoded secrets. Their presence confirms the user authenticated with LM Studio Hub or an external model repository. The `lms login` command uses asymmetric key pairs ( `key-id` , `public-key` , `private-key` ) for CI-style authentication, meaning key material may reside here in exportable form[35]. 

- **Volatility:** Persistent. Credentials persist until the user explicitly logs out or deletes the files. 

####  **User-Uploaded Files** 

** Location:** `~/.lmstudio/user-files/` 

- **Format:** Original file formats as uploaded by the user (PDF, TXT, DOCX, source code, etc.). 

- **Forensic Value:** Contains files the user attached to chat sessions or fed into the RAG pipeline. The presence and modification timestamps of files here directly corroborate when external documents were introduced into the models context. These files may persist even after the originating chat session is deleted. 

- **Volatility:** Persistent. 

16 

_6.3 llama.cpp Artifacts_ 

### **6.3 llama.cpp Artifacts** 

By design, llama.cpp is a minimalist tool and consequently, its forensic footprint is the most ephemeral and challenging to analyze. 

- **Model Files** 

** Location:** User defined. Typically in a ./models/ subdirectory relative to the executable[5]. 

** Format:** GGUF. 

- **Forensic Value:** The presence of .gguf files is the primary indicator that LLM activity may have occurred. Analysis of the GGUF file itself is the main source of evidence (see 6.4). 

** Volatility:** Persistent. 

- **Command Line History** 

** Location:** Standard shell history files (~/.bash_history, ~/.zsh_history, PowerShell history). 

   - **Format:** Plain text. 

   - **Forensic Value:** This is the most critical artifact for llama.cpp. The full command line used to launch llama-cli or main contains the path to the model, all generation parameters (temperature, top-p, etc.) and often the initial prompt itself (if passed with the -p flag)[23]. 

   - **Volatility:** Highly volatile. Shell history is often limited in size, can be disabled, or can be easily cleared by a user. 

- **Memory Artifacts** 

   - **Location:** System RAM. 

   - **Format:** Raw memory strings. 

   - **Forensic Value:** During execution, the prompt text, model weights and generated output exist in the processs memory space. A live memory capture or analysis of a pagefile/swap file may be the _only_ way to recover a prompt that was not logged or passed via the command line (e.g., in interactive mode)[67, 68]. 

   - **Volatility:** Extremely volatile. Lost upon process termination or system shutdown. 

### **6.4 Cross Cutting Artifacts (GGUF and SQLite)** 

Two file formats are common across the local LLM ecosystem and warrant special attention. 

- **GGUF File Analysis:** The GGUF format is central to llama.cpp and the models used by Ollama and LM Studio. Using Python libraries like gguf and pygguf, an investigator can parse these binary files[60]. 

17 

   - **Forensic Value:** The GGUF header and metadata section contain a wealth of information[25]. This includes the models architecture (e.g., llama, qwen), parameter count, quantization level (e.g., Q4_K_M), context length, embedding length and the full tokenizer vocabulary. This data can be used to precisely fingerprint a model and understand its capabilities, which is crucial for verifying if a given output could have been produced by a specific model file found on a system. 

- **SQLite Database Analysis:** While not used by the core applications in their default state, many popular front ends and related tools in the ecosystem use SQLite databases for storing user settings, chat histories, or vector embeddings for RAG[73]. 

   - **Forensic Value:** Standard forensic techniques for SQLite are highly applicable [76]. Tools can analyze the main database file, but also the rollback journal (-journal) and Write-Ahead Log (-wal) files to recover transient data. Furthermore, carving for deleted records within the database files freelists and unallocated space can recover previously deleted chat messages or configuration settings, providing evidence a user thought they had removed[59]. 

The architectural design of each tool client server for Ollama, monolithic Electron for LM Studio and minimalist CLI for llama.cpp fundamentally dictates the nature and persistence of the evidentiary trail. This creates a clear hierarchy of forensic richness. User friendly applications like LM Studio, designed for convenience, generate more structured and persistent artifacts. In contrast, the ephemeral traces left by a command line tool like llama.cpp make it more difficult to investigate post facto. This suggests that a suspects choice of tool can itself be an indicator of their technical sophistication and potential awareness of forensic countermeasures, a "meta artifact" that can inform the overall investigative strategy. 

## **7 Experiments and Results** 

To validate and quantify the findings from our artifact analysis, we conducted a series of controlled experiments. These experiments were designed to simulate common investigative challenges: reconstructing user interactions and determining the persistence of evidence. The results provide objective metrics on the forensic utility of the identified artifacts. 

### **User Interaction Reconstruction** 

This experiment aimed to determine the extent to which a users conversation with an LLM could be recovered from disk based artifacts after a normal system shutdown. 

18 

- **Experimental Setup:** On each of the six test environments (3 tools x 2 OSes), we executed a standardized script of 10 prompts. The prompts ranged from simple questions to code generation requests and contained unique keywords. After the session, the VM was shut down cleanly. 

- **Procedure:** The "after" disk image of each VM was mounted and analyzed. We searched for the primary prompt/chat history artifacts identified in Section 6: ~/.ollama/history for Ollama, ~/.lmstudio/conversations/*.json for LM Studio and shell history files for llama.cpp. 

- **Results:** 

- ** LM Studio:** For both the operating systems, 100% of the 10 prompts and their corresponding model responses were recovered with full fidelity from the JSON chat logs. Timestamps, model identifiers and session configurations were also fully intact. 

   - **Ollama:** On both the OSes, 100% of the prompts entered via the ollama run CLI were recovered from the ~/.ollama/history file. However, this file only contains the users input, not the models output. Prompts sent via an API client were not logged in this file. 

   - **llama.cpp:** Recovery was dependent on the shell. On Linux 100% of the commands, including prompts passed with the -p flag, were recovered from .bash_history or .zsh_history. On Windows, PowerShell history successfully captured the commands. In interactive mode (-i), no prompts were logged to the shell history, resulting in 0% recovery from disk. This highlights the critical importance of memory forensics for llama.cpp. 

deletion, simulating a non expert users attempt to cover their tracks. 

### **7.1 Summary of Forensic Artifacts** 

The following table synthesizes the findings from our analysis and experiments, providing a comprehensive, at a glance reference for forensic practitioners. It ranks artifacts based on their forensic value and persistence, helping investigators prioritize their efforts during an examination. 
