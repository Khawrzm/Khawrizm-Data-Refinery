
**Table 2: Summary of Key Forensic Artifacts by Product and Operating System** 

|**Product**|**OS**|**Artifact Category**|**Default Path (User Profl**|
|---|---|---|---|
|**Ollama**|All|Model Manifests|~/.ollama/models/manifests/|
|All|Model Blobs|~/.ollama/models/blobs/|Binary|
|All|CLI History|~/.ollama/history|Plain Text|
|All|Server Logs|~/.ollama/logs/server.log|Plain Text|
|Linux|Confguration|/etc/systemd/system/ollama.service|INI|
|**LM Studio**|All|**Chat History**|~/.lmstudio/conversations/|

19 

|All|Model Cache|~/.lmstudio/models/|GGUF|
|---|---|---|---|
|All|Confg Presets|~/.lmstudio/confg-presets/|JSON|
|All|RAG Cache|~/.lmstudio/.session_cache|Binary|
|All|Application Logs|N/A (Live Stream)|Text Stream|
|**llama.cpp**|All|CLI History<br>|Shell History (.bash_history, etc.)|
|All|Model Files|User defned (e.g., ./models/)|GGUF|
|All|Memory|System RAM / Pagefle|Raw Strings|

Table 3: Summary of Key Forensic Artifacts by Product and Operating System 

**?** __ `tab:forensic-artifacts` __ **? Product OS** 

|**Product **|**OS**|**Artifact**<br>**Category**|**Default Path**<br>**(User Profle**<br>**Relative)**|**Format**|**Persistence**|**Forensic**<br>**Value**<br>**(15)**|**Remarks**|
|---|---|---|---|---|---|---|---|
|**Ollama**|All|Model<br>Manifests|~/.ollama/models/man<br>|ifests/<br>JSON<br>|Persistent|4|Proves which<br>models/versions were<br>downloaded.|
||All|Model Blobs|~/.ollama/models/blob|s/Binary|Persistent|3|Confrms presence of<br>model layers via<br>hashing.|
||All|CLI History|~/.ollama/history|Plain<br>Text|Persistent|5|Plaintext record of<br>`ollama run` prompts.|
||All|Server Logs|~/.ollama/logs/server.l|ogPlain<br>Text|Semi-<br>Persistent|4|Records server activity;<br>can be<br>redirected/deleted.|
||Linux|Confguration|/etc/systemd/system/|ollama.serv<br>INI|ice<br>Persistent|3|Reveals non-default<br>paths or network<br>settings.|
|**LM**<br>**Studio**|All|**Chat**<br>**History**|~/.lmstudio/conversati|ons/<br>**JSON**|**Persistent **|**5**<br>**(Critical)**|**Complete,**<br>**timestamped user/AI**<br>**conversation logs.**|
||All|Model<br>Cache|~/.lmstudio/models/|GGUF|Persistent|4|Stores models; path<br>reveals Hugging Face<br>origin.|
||All|Confg<br>Presets|~/.lmstudio/confg-<br>presets/|JSON|Persistent|4|Shows user-defned<br>model parameters and<br>intent.|
||All|RAG Cache|~/.lmstudio/.session_c|ache<br>Binary|Semi-<br>Persistent|4|Contains fragments of<br>documents used in RAG.|
||All|Application<br>Logs|N/A (Live Stream)|Text<br>Stream|Volatile|5|`lms log stream` shows<br>fnal formatted prompt.|
|**llama.cp**|**p**All|CLI History|Shell History<br>(`.bash_history`, etc.)|Plain<br>Text|Volatile|5<br>(Critical)|Often the only record of<br>prompts and<br>parameters.|
||All|Model Files|User-defned (e.g.,<br>`./models/`)|GGUF|Persistent|3|Proves presence of<br>models; metadata is key.|
||All|Memory|System<br>RAM / Pagefle|Raw<br>Strings|Extremely<br>Volatile|5<br>(Critical)|May be the only source<br>for interactive mode<br>prompts.|

_Note: ~ refers to the users home directory (/home/<user> on Linux and C:\Users\<user> on Windows)._ 

20 

## **8 Discussion** 

Our findings have significant implications for digital forensic investigators, legal professionals and software vendors. The massive adoption of local LLMs has created a new and complex evidentiary landscape that poses multiple challenges to traditional investigative methods while simultaneously offering unprecedented insight into user intent. This section discusses these implications, explores potential anti forensic techniques and countermeasures and provides recommendations for stakeholders. 

### **8.1 Implications for Investigators** 

Our analysis demonstrates that local LLMs are a double edged sword for digital forensics. On one hand, they represent a new vector for malicious activity that can be conducted offline, away from the purview of network monitoring. On the other hand, when artifacts are present, they provide an exceptionally rich source of evidence regarding a users state of mind, intent and actions. 

The primary takeaway is that the forensic strategy must be tailored to the specific tool in use. The architectural differences between Ollama, LM Studio and llama.cpp are not trivial and they create a clear hierarchy of evidentiary persistence. An investigator examining a system with LM Studio can expect to find structured, persistent chat logs that are relatively easy to parse and recover[64]. In contrast, an investigation involving llama.cpp may yield no persistent prompt history on disk, making live memory acquisition and analysis of shell history paramount[23]. Investigators must therefore be trained to first identify the specific runner being used and then apply the appropriate analytical workflow as outlined in this paper. 

### **8.2 Anti Forensic Risks and Countermeasures** 

A forensically literate user can take several steps to obstruct or evade analysis of their local LLM activity. 

- **Anti Forensic Techniques:** 

- ** Path Obfuscation:** A user can set the OLLAMA_MODELS environment variable or use LM Studios settings to store models and data on an external or encrypted volume, evading searches of default directories[40]. 

   - **Ephemeral Execution:** Tools like llama.cpp or Ollama can be run from a temporary directory or a live USB, leaving minimal traces on the host machines primary storage. 

   - **Logging Evasion:** Ollamas server logging can be disabled or redirected to /dev/null on Unix like systems. For LM Studio, a user can simply delete the JSON chat logs from the cache directory. 

21 

_8.3 Vendor Recommendations and Future Work_ 

   - **Tool Selection:** As our findings show, a sophisticated actor would likely choose llama.cpp over LM Studio precisely because it generates fewer persistent artifacts. 

- **Investigative Countermeasures:** 

- ** Signature Based Detection:** The YARA rules and triage commands can be developed to find artifacts regardless of their location. Searching for GGUF file headers (GGUF) across an entire disk image can identify model files even in non standard paths[77, 78, 79]. 

   - **Memory Forensics:** Live memory analysis remains the most effective countermeasure against ephemeral execution and interactive prompts. Searching a memory dump for strings related to model loading or prompt templates can reveal activity that was never written to disk. 

   - **System Level Artifacts:** Even if application level logs are deleted, OS artifacts can provide crucial leads. Windows Prefetch files can show that lm studio.exe or ollama.exe was executed. Shell history can capture the commands used to launch llama.cpp or change environment variables. 

### **8.3 Vendor Recommendations and Future Work** 

To improve the forensic readiness of these tools, especially in corporate or regulated environments, we offer the following recommendations for vendors: 

- **Standardized, Structured Logging:** We recommend that vendors like Ollama and LM Studio implement a robust, standardized logging framework. This should be an opt in feature for enterprise or forensic use, creating a single, secure log file that records all key events: user authentication, model loading, API requests, full prompt text and generated responses, all with reliable timestamps. This would be invaluable for incident response. 

- **Log Integrity:** To prevent tampering, these logs could incorporate cryptographic integrity checks, such as chaining log entries with hashes, similar to a blockchain. 

Our work has several limitations that open avenues for future research. We only partially investigated containerized deployments (Ollama running in Docker), which introduces layers of abstraction that complicate forensic analysis[29, 80]. The forensic traces left by model finetuning and the use of Retrieval Augmented Generation (RAG) with large, external knowledge bases also warrant dedicated study[81, 82]. Finally, the growing ecosystem of third party web front ends for these tools (Open WebUI [75]) creates additional artifacts that need to be cataloged. 

22 

_8.4 Privacy and Admissibility in the Age of Local AI_ 

### **8.4 Privacy and Admissibility in the Age of Local AI** 

The ability to recover a users complete, verbatim interactions with an LLM raises profound legal and ethical questions. These prompt histories can be more revealing than a private diary, capturing a users brainstorming, sensitive queries and unrefined thoughts. This places a heavy burden on the legal system to balance the needs of an investigation with an individuals right to privacy[22]. The specificity of search warrants, as discussed in Section 3.3, becomes non negotiable. 

Furthermore, for evidence derived from these systems to be admissible, it must be presented in a reliable and understandable manner. An investigator cannot simply present a generated text as evidence. It must be prepared to use the artifacts we have identified model manifests, GGUF metadata, configuration presets to explain the process by which that text was generated. This aligns with the principles of the _Daubert_ standard and the proposed FRE 707, which demand that the proponent of machine generated evidence demonstrate the validity of the underlying process[33]. Our research provides the first map to the artifacts needed to build that foundational argument. 

## **9 Conclusion** 

This paper has conducted the first systematic and empirical forensic analysis of the leading local Large Language Model runners: Ollama, LM Studio and llama.cpp. We have demonstrated that while these applications are designed with privacy and offline use in mind, they create a rich and varied trail of digital evidence across Windows and Linux platforms. The architectural choices of their developers have resulted in a clear hierarchy of forensic utility, with user friendly GUI applications like LM Studio producing highly structured and persistent artifacts, while minimalist command line tools like llama.cpp leave more ephemeral traces. 

Our research provides a foundational methodology for this new domain of digital forensics. We have established a comprehensive corpus of artifacts, from model caches and manifests to plaintext chat histories and configuration files. We have validated the forensic value of these artifacts through controlled experiments. 

The rise of local AI represents a paradigm shift and the digital forensics field must adapt accordingly. The techniques and findings presented in this paper are a critical first step, equipping investigators to pull back the curtain on these private AI environments. By integrating these methodologies into standard operating procedures, the forensic community can ensure that evidence from local LLM systems is identified, recovered and presented in a manner that is both scientifically rigorous and legally sound, upholding justice in an increasingly intelligent world. 

23 

_REFERENCES_ 

## **References** 

- `1` [1] LM Studio - Local AI on your computer. https://lmstudio.ai/ (Accessed: January 8, 2026). 

- `1.1` [2] Ollama. https://ollama.com/ (Accessed: January 8, 2026). 

- `1.2` [3] GitHub - ggml-org/llama.cpp: LLM inference in C/C++ (no date). https://github.com/ggml-org/llama.cpp (Accessed: January 8, 2026). 

- `1.3` [4] Mucci, T. (2025, November 17). GGUF versus GGML. IBM. https://www.ibm.com/think/topics/gguf-versus-ggml 

- `2` [5] How to Use llama.cpp to Run LLaMA Models Locally - Codecademy, accessed August 8, 2025, _https://www.codecademy.com/article/llama-cpp_ 

- <u>`3`</u> [6] Where Are Ollama Models Stored on Mac? - BytePlus, accessed August 8, 2025, _https://www.byteplus.com/en/topic/418089_ 

- `4` [7] Yin, Z., Wang, Z., Xu, W., Zhuang, J., Mozumder, P., Smith, A. and Zhang, W., 2025. Digital Forensics in the Age of Large Language Models. arXiv preprint arXiv:2504.02963. 

- <u>`5`</u> [8] Exploring the Potential of Large Language Models for Improving Digital Forensic Investigation Efficiency - ResearchGate, accessed August 8, 2025, 

- <u>`6`</u> [9] ForensicLLM: A local large language model for digital forensics | DFRWS, accessed August 8, 2025, _https://dfrws.org/wpcontent/uploads/2025/03/ForensicLLM-A-local-large-languagemod_2025_Forensic-Science-International-.pdf_ 

- <u>`7`</u> [10] A comprehensive study of Cybercrime and Digital Forensics through Machine Learning and AI | Al-Rafidain Journal of Engineering Sciences, accessed August 8, 2025, _https://rjes.iq/index.php/rjes/article/view/168_ 

- <u>`8`</u> [11] The Use of Machine Learning in Digital Forensics: Review Paper - Atlantis Press, accessed August 8, 2025, _https://www.atlantispress.com/article/125984186.pdf_ 

- <u>`9`</u> [12] AI-Enhanced Digital Forensics: Automated Techniques for Efficient Investigation and Evidence Collection | Journal of Electrical Systems, accessed August 8, 2025, _https://journal.esrgroups.org/jes/article/view/766_ 

- `10` [13] Using Micro-Services and Artificial Intelligence to Analyze Images in Criminal Evidences - DIGITAL FORENSIC RESEARCH CONFERENCE, accessed August 8, 2025, _https://dfrws.org/wpcontent/uploads/2021/09/2021-usa-paper-41-using_micro-_ 

   - _services_and_artificial_intelligence_to_analyze_images_in_criminal_evidences.pdf_ 

24 

_REFERENCES_ 

- `11` [14] Murtuza, Shariq, Robin Verma, Jayaprakash Govindaraj, and Gaurav Gupta. "A tool for extracting static and volatile forensic artifacts of windows 8. x apps." In _IFIP International Conference on Digital Forensics_ , pp. 305-320. Cham: Springer International Publishing, 2015. 

- `12` [15] The Future of Artificial Intelligence (AI) Applications in Forensics - RAIS Conferences, accessed August 8, 2025, _https://rais.education/wpcontent/uploads/2025/05/0523.pdf_ 

- `13` [16] Large Language Models in Modern Forensic Investigations: Harnessing the Power of Generative Artificial Intelligence in Crime Resolution and Suspect Identification - Zenodo, accessed August 8, 2025, _https://zenodo.org/records/14825697_ 

- `14` [17] Leveraging LLMs for Memory Forensics: A Comparative Analysis of Malware Detection, accessed August 8, 2025, 

- `15` [18] Murtuza, Shariq. "Scout: Leveraging Large Language Models for Rapid Digital Evidence Discovery." arXiv preprint arXiv:2507.18478 (2025). 

- `17` [19] Exploring the potential of large language models for author profiling tasks in digital text forensics - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/exploring-the-potential-of-largelanguage-models-for-author-profiling-tasks-in-digital-text-forensics/_ 

- `18` [20] Deep Reasoning and Large Context Windows: Next-Generation AI in Digital Forensic Investigations - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/deep-reasoning-and-large-contextwindows-next-generation-ai-in-digital-forensic-investigations/_ 

- `19` [21] ForensicLLM: A Local Large Language Model for Digital Forensics - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/forensicllm-a-locallarge-language-model-for-digital-forensics/_ 

- `20` [22] The Ethical Implications of AI in Forensic Science, accessed August 8, 2025, _https://forensicscienceacademy.org/blog/f/the-ethical-implications-ofai-in-forensic-science_ 

- `21` [23] llama.cpp - Qwen - Read the Docs, accessed August 8, 2025, _https://qwen.readthedocs.io/en/latest/run_locally/llama.cpp.html_ 

- `22` [24] llama.cpp - Wikipedia, accessed August 8, 2025, _https://en.wikipedia.org/wiki/Llama.cpp_ 

- `23` [25] GPT-Generated Unified Format - NATO, accessed August 8, 2025, _https://nhqc3s.hq.nato.int/apps/DCRA_Report/id29d4122b072148f5aaf4882ecc5d963c/elements/id673b71b3cfc34434a68a55988f9a6354.html_ 

25 

_REFERENCES_ 

- `24` [26] What is GGUF? A Beginners Guide - Shep Bryan, accessed August 8, 2025, _https://www.shepbryan.com/blog/what-is-gguf_ 

- `25` [27] Analysis of Ollama Architecture and Conversation Processing Flow for AI LLM Tool, accessed August 8, 2025, _https://medium.com/@rifewang/analysis-of-ollama-architecture-andconversation-processing-flow-for-ai-llm-tool-ead4b9f40975_ 

- `26` [28] About LM Studio | LM Studio Docs, accessed August 8, 2025, _https://lmstudio.ai/docs_ 

- `28` [29] What is Container Forensics and Incident Response? - Sysdig, accessed August 8, 2025, _https://www.sysdig.com/learn-cloud-native/what-is-containerforensics-and-incident-response_ 

- `29` [30] Best practices for performing forensics on containers. | Google Cloud Blog, accessed August 8, 2025, _https://cloud.google.com/blog/products/containerskubernetes/best-practices-for-performing-forensics-on-containers_ 

- <u>`30`</u> [31] Best Practices for Maintaining Chain of Custody for Digital Evidence - Vidizmo, accessed August 8, 2025, _https://vidizmo.ai/blog/chain-of-custodyfor-digital-evidence_ 

- <u>`31`</u> [32] What is the Chain of Custody in Digital Forensics? - Champlain College Online, accessed August 8, 2025, _https://online.champlain.edu/blog/chaincustody-digital-forensics_ 

- <u>`32`</u> [33] Artificial Intelligence and the Law: Expert Witness Issues in 2025 - Forensis Group, accessed August 8, 2025, _https://www.forensisgroup.com/resources/expert-legal-witnessblog/artificial-intelligence-law_ 

- <u>`33`</u> [34] Jessica Kerbel and Leonard Dietzen, III, New AI Rule, Old Standard: Proposed Federal Rule of Evidence 707 Aims to Apply Daubert Standard to AI-Generated Evidence, _RumbergerKirk_ , June 20, 2025, accessed August 8, 2025, _https://www.rumberger.com/insights/new-ai-rule-old-standardproposed-federal-rule-of-evidence-707-aims-to-apply-daubert-standard-to-aigenerated-evidence/_ 
