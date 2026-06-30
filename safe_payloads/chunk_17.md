
Digital artefacts related with an Ollama installation are as follows: 

7 

_4.2 LM Studio_ 

- Since Ollama uses a client server model the language model is exposed as an API on the local network port (default 11434)[27]. All the interaction with the model happens via this port only[38]. 

- **Installation Footprint:** 

- ** Linux:** The official installation script (install.sh) makes a separate user named ollama (a system user) and a corresponding systemd service. The main Ollama executable binary is in /usr/local/bin, the model data is kept at /usr/share/ollama[37] and is available system wide. User specific data including the downloaded models and logs are stored by default in the users home directory at ~/.ollama [40]. 

   - **Windows:** The installer binary puts the application files in the current users local application data folder located at C:\Users\<username>\AppData\Local\ollama [43, 46, 47]. The models and logs are kept at C:\Users\<username>\.ollama [39, 40]. The user can also install the application at a custom location by using the command line flag (/DIR=) to give the installation directory [41, 42, 44, 45, 48]. 

### **4.2 LM Studio** 

LM Studio is another alternative desktop application built upon the Electron framework[27]. Electron bundles a web based user interface typically made using HTML, CSS, JavaScript along with a backend process into a single executable. In the case of LM Studio a llama.cpp inference engine is included. LM Studio provides GUI based model discovering, downloading and chatting. It also provides an OpenAI compatible API server for programmatic access[28, 56, 57]. LM Studio also supports inferencing via GGUF model files. 

- **Installation Footprint:** 

- ** Linux:** The software is distributed as an .AppImage file. An AppImage file is a self sufficient way to distribute software. It includes all the required files and libraries that are mounted when the application is executed. When executed, there is no traditional installation. The required data directories are created within the users home folder. Major forensic artifacts are present in ~/.lmstudio/ and ~/.config/LM Studio[49, 50, 51, 53]. 

   - **Windows:** Like typical softwares, a standard .exe installer is provided to install. It has the vast majority of its operational data, including the crucial model cache and conversation logs, under the users profile at %USERPROFILE%\.cache\LM Studio\ and %USERPROFILE%\.lmstudio\[54]. 

### **4.3 llama.cpp** 

llama.cpp is a community driven open source C++ software library built for extremely efficient, local inference of large language models (originally built 

8 

for LLaMA but now supports almost every model. It exploits the GGML tensor library (also open source) that performs highly optimized computation based on the hardware (including CPUs and GPUs). The framework supports quantization levels (from 1.5 bit to 8 bit) enabling large models to be deployed on systems with a not so high configuration (often as low as 6GB RAM). llama.cpp performs text tokenization, inferencing using next token sampling and finally detokenization via the language model (in GGUF file format [70, 71, 72]), which bundles weights, tokenizer and metadata for quick loading and deployment. Llama.cpp also features real time token streaming, hybrid CPU+GPU inference, speculative decoding for speed and OpenAI compatible APIs. All these features make them the ideal solution for privacy focused, offline deployment without external dependencies like Python or CUDA frameworks. 

- **Architecture:** Written in C/C++, originally llama.cpp is not intended to be used directly as a user facing application. Instead it is meant to act like a command line based tool that is supposed to have some kind of front end like Ollama or LM Studio. The core design focuses on highly optimized performance LLM text inference with minimal dependencies[5]. Llama.cpp is distributed as source code and meant to be compiled, however compiled binaries are also released[24]. 

- **Installation Footprint:** There is no standard installation path. The tool and all the corresponding files (main, llama-cli) are present wherever the user downloaded and compiled the source code repository[5]. Forensically interesting artifacts are created in the same directory from which the tool is executed. The model files in the GGUF format, are usually placed by the user in a manually created models subdirectory within the project folder[5]. The absence of a standardized footprint causes difficulties for investigators. 

## **5 Methodology** 

We used a detailed, rigorous and forensically sound procedure to identify, generate and then analyze digital artifacts from the selected softwares. The process is designed to be reproducible so that the findings can be reproduced and verified which is very important for legal admissibility[9]. 

### **5.1 Forensic Acquisition** 

To establish a clean baseline environment and ensure proper and clear artifacts attribution, a differential analysis approach was used to compare before and after states. 

- **Disk Imaging:** For both the operating systems (Windows 11, Ubuntu 24.04) we made a base virtual machine (VM). A complete bit by bit disk image of this base state was made using dd for Linux and FTK 

9 

_5.2 Instrumentation and Data Generation_ 

Imager for Windows. Then the target software was installed and specific models were downloaded. After this a second disk image was captured. These comparative images allowed for precise identification of all files and identifying the system changes introduced by the software. 

- **Memory Acquisition:** Volatile memory or the RAM (Random Access Memory) often has artifacts that will typically not be written back to the disk. Examples include the user given in memory prompts or transient configuration data. While chatting with the LLM sessions, live memory captures were simultaneously performed using industry standard tools: Linux Memory Extractor (LiME) for Linux and WinPmem for Windows. 

- **OS Artifact Collection:** Standard host based artifacts were acquired to correlate the findings from the application specific data. This included shell history files (.bash_history, .zsh_history), PowerShell console history, Windows Prefetch files (.pf) and the Application Compatibility Cache (Shimcache). 

### **5.2 Instrumentation and Data Generation** 

To analyze and map the behavior of each application and to ensure that a consistent set of evidentiary data is generated across the experiments, we used a combination of system monitoring and scripted interactions. 

- **Process Monitoring:** System level tracing tools were used to monitor file system, registry (on Windows) and process activity during installation, model downloads and chat sessions. Process Monitor (ProcMon) from Sysinternals was used on Windows, while strace was used on Linux. These tools gave a real time log of every file read, written and modified by the applications. 

- **Network Capture:** For network traffic monitoring we used Wireshark to capture all the passing network traffic from the test VMs. This helped us to analyze and know telemetry, update checks, model download communications or any other network activity of forensic interest[38]. 

- **Controlled Data Generation:** A predecided, fixed script of user interactions was performed on each platform and tool. This included downloading specific models, running a series of ten distinct prompts and then deleting some of the generated artifacts. The prompts included benign questions ("How to make cake?"), requests for code generation ("Write a Python script to list files in a directory") and the inclusion of unique keywords (e.g., "FORENSIC_KEYWORD_12345") to facilitate later searching and data carving. All actions were properly recorded in a timestamped experiment log. 

10 

_5.3 Analysis Procedures_ 

### **5.3 Analysis Procedures** 

The collected data was then analyzed using a combination of tools. 

- **Integrity Verification:** All acquired disk images and key evidence files were hashed using the SHA 256 algorithm. These hashes were verified throughout the analysis process to ensure data integrity and maintain a valid chain of custody. 

- **Forensic Tooling:** Analysis of the disk images was conducted using leading open source digital forensic platform Autopsy. These tools were used for file system navigation, keyword searching and carving for deleted files. 

- **Specialized Analysis:** For application related artifacts case to case based specialized tools were used. SQLite databases were examined using the Foxton SQLite Examiner which can parse freelists and Write Ahead Logs (WAL) to recover deleted or uncommitted records[58]. Custom Python scripts were developed using the GGUF library to parse the metadata and structure of GGUF model files[60]. JSON formatted chat logs were parsed programmatically to extract conversations and metadata. 

- **Chain of Custody:** A formal chain of custody log was maintained for all evidentiary items. This document recorded every individual who handled the evidence, the date and time of transfer and the actions performed, adhering to best practices to ensure the evidences admissibility[31]. 

## **6 Artifact Analysis** 

This section presents the core findings. A detailed breakdown of the forensic artifacts created by Ollama, LM Studio and llama.cpp is given with the locations. The architectural differences between these tools result in distinct evidentiary footprints, with varying levels of richness, persistence and ease of recovery. For each artifact, we detail its location, format, forensic value and volatility. 

### **6.1 Ollama Artifacts** 

Ollamas client server architecture creates a centralized set of artifacts inside a hidden _.ollama_ directory in the users profile. Ollama is also distributed as a dockerized container for rapid deployment. In case of a container based deployment the _.ollama_ directory of the container is stored in the _/root_ directory. It is mapped to the _/var/lib/docker/volumes/ollama/_data_ location on the host system. [6] 

- **Model Manifests** 

** Location:** /.ollama/models/manifests/registry.ollama.ai/library/<model>/<tag> 

** Format:** JSON. 

11 

_6.1 Ollama Artifacts_ 

   - **Forensic Value:** These files are critical for proving which specific models and versions a user has downloaded. Each manifest contains metadata about the model, including a list of SHA 256 hashes corresponding to the models layers (blobs). This allows an investigator to confirm the exact composition of a model on the system[27]. 

   - **Volatility:** Persistent. These files remain on disk until the model is explicitly removed via ollama rm. 

- **Model Blobs (Layers)** 

** Location:** ~/.ollama/models/blobs/sha256-<hash> 

   - **Format:** Binary data. 

   - **Forensic Value:** These are the actual data layers of the LLMs. While their content is not human readable, their presence, verified by matching their SHA 256 hash with a manifest file, proves that a specific model was present on the machine. Hashing these files can be part of a signature based detection strategy[27]. 

   - **Volatility:** Persistent. Blobs are content addressable and may be shared across multiple models. They are only deleted when no manifest references them. 

- **Server Logs** 

   - **Location:** Default: ~/.ollama/logs/server.log. This can be redirected by the user at runtime (e.g., ollama serve > /path/to/logfile.log 2>&1)[61, 62, 63]. 

   - **Format:** Plain text. 

   - **Forensic Value:** Highly valuable for reconstructing a timeline of activity. Logs can contain timestamps for server startup/shutdown, model loading events, API requests from clients and, if verbose logging is enabled (OLLAMA_DEBUG=true), potentially the full text of user prompts and model responses[27]. 

   - **Volatility:** Semi persistent. The log file can be easily deleted by the user. Its location can be changed, making it harder to find. 

- **CLI History** 

** Location:** ~/.ollama/history 

** Format:** Plain text, one entry per line. 

- **Forensic Value:** Provides direct, plaintext evidence of user prompts entered via the ollama run command. This is a crucial artifact for understanding user intent. However, it does not capture interactions made through the API or third party GUI clients. 

- **Volatility:** Persistent, but only captures one mode of interaction and can be deleted. 

12 

_6.2 LMStudio Artifacts_ 

####  **Configuration** 

- **Location:** No single configuration file. Configuration is primarily managed through environment variables (e.g., OLLAMA_MODELS, OLLAMA_HOST) set in shell profiles (.bashrc, .zshrc) or, on Linux, in the systemd service file (/etc/systemd/system/ollama.service)[40]. 

- **Format:** N/A (environment variables, .ini style service files). 

- **Forensic Value:** Critical for identifying non default configurations. An OLLAMA_MODELS variable will point to a custom storage location for models, which an investigator must examine. An OLLAMA_HOST variable might indicate the server was bound to a public network interface. 

- **Volatility:** Persistent. 

### **6.2 LMStudio Artifacts** 

As a feature rich Electron application, LM Studio creates the most structured and comprehensive set of forensic artifacts, making it the most forensically revealing of the tools analyzed. The most important file is ~.lmstudio-homepointer, which is a small text file created by LM Studio application. It stores the absolute path to the applications home data directory which is ~ /.lmstudio or ~ /.cache/lm-studio on Linux/Mac, or ~%USERPROFILE%\.lmstudio on Windows. Inside this folder the following artefacts are stored. 
