
2 

_2.1 Digital Forensics of AI/ML Systems_ 

### **2.1 Digital Forensics of AI/ML Systems** 

Traditional research in the domain of AI and forensics intersection has typically focussed on the application of machine learning techniques to support and strengthen the investigative capabilities [11, 13]. Prior and current works presents the application of artificial intelligence for the automation of the analysis of vast artifacts, detecting network traffic anomalies, identifying unusual user behaviour or classifying digital evidence such as images or malware[10]. These applications aim to improve the efficiency and effectiveness of forensic examiners who face an ever increasing volume of data[14]. The underlying premise of such research is to apply AI as an analytical tool within established digital forensic frameworks, such as those proposed by the Digital Forensics Research Workshop (DFRWS) or the National Institute of Standards and Technology (NIST)[8]. While quite usefull, this research paradigm treats the AI system as a trusted assistant, not as a source of evidence itself [15, 20, 21]. 

### **2.2 Forensic Artifacts from Agentic and LLM Systems** 

In recent times, with the release of powerful Large Language Models (LLMs), the focus of researchers has been drawn towards their potential to be deployed in the digital forensics domain. Multiple studies has explored and tried to evaluate the potential of LLMs to be used as "investigative assistants"[7, 16, 17, 18]. Such task requires the LLMs to be able to summarize the case files, while analyzing textual evidence. More complex tasks involve the application of a LLM for authorship attribution in order to detect unique traits like age and gender using written text[19]. Other works have deeply focused on creating highly specialized domain specific models, such as ForensicLLM, which are finetuned on digital forensics related datasets to provide much more accurate responses by utilizing their context aware processing capabilities[9]. 

These works rely on trusting a large language model to make sensitive decisions, due to which such tools are often supplemented with human monitoring all the steps manually. Such tools or services may also require the subscription of an online LLM hosting service where the LLM is hosted. The tools then collect the data locally and send it to the cloud based service where the LLM is hosted for obtaining results. This can be a hurdle if the case data is confidential or sensitive and requires a high amount of discretion. Privacy is also a major concern and law enforcement cannot be allowed to upload such data to third party services[9]. 

Even if the language models are hosted locally in an offline setting, the models are highly prone to hallucination where they generate factually incorrect data. Other issues include the presence of inherent biases in the training data of the model, the process of complicated and unexplainable decision making process. Such privacy issues further fuels the adoption of local LLM runners. 

3 

_2.3 The Local LLM Ecosystem and the Forensic Gap_ 

### **2.3 The Local LLM Ecosystem and the Forensic Gap** 

This section discusses the current state of local LLM running tools and their forensic implications. The need for privacy, security and confidentiality often results in individuals selecting offline solutions albeit having lower capabilities over stronger and highly capable but third party hosted online solutions. The open source nature of these tools have further captured general interest resulting in rapid capability updation. With modern desktops and laptops being able to run and deploy language models as capable as GPT 3.5 without any external hardware addition. This has been majorly possible due to the efforts of the most powerful and popular open source C++ based inference engine named llama.cpp[3]. This inference engine is built with high optimizations and has become the defacto standard, with Ollama, LM Studio and almost every other local inference tool using llama.cpp at its core. Llama.cpp was originally designed for deployment on consumer grade CPU in an extremely efficient and resource aware manner but has not expanded to now utilize GPUs also, if available. 

llama.cpp has its own specialized file format called GPT-Generated Unified Format (GGUF) [4, 26, 69], which has now become an industry standard. Language models are typically distributed in a binary format packing model weights, metadata and quantization information into a single, portable file. Applications like Ollama and LM Studio have a user friendly graphical interfaces to facilitate interaction with the language models. Ollama and llama.cpp provide a server based API using client server paradigm to allow any application to interact with the locally hosted model [27]. LM Studio on the other hand is built as a standalone application that supports chatting via a graphical interface[28]. This rapid adoption of local LLM deployment softwares has created a large forensic gap. Extensive work has been already done for using large language models as assistants in digital forensics tasks, but there is a near complete absence of academic literature on the forensic analysis of local LLM runners. These offline first, secure assistants have become an evidentiary blind spot for forensic investigators. This push for local AI applications in forensics, has paradoxically created another new class of applications whose digital traces are unexplored and unidentified. To the best of our knowledge this work is the first to explore this avenue, aiming to lay the foundational analysis for evidence processing of these local first AI environments to ensure that cases involving these softwares can be investigated with the same rigor as other digital activity. 

## **3 Threat and Forensic Models** 

To base this research on ground reality, we create a set of different models to cover the possible local LLMs misuse. The threat scenarios guide our forensic approach which aims to be designed to be compliant with scientifically and legally established standards. 

4 

_3.1 Investigative Scenarios_ _<u>(Use</u> Case Models)_ 

### **3.1 Investigative Scenarios (Use Case Models)** 

The threat models are as follow: 

- **Insider Threat:** This scenario involves an employee using a local LLM on a corporate computer system to process (summarize, analyse or rephrasing etc.) confidential internal documents (source code, trade secretor, financial data etc.) and then exfiltrate it. A locally running LLM was chosen by the employee since it wont leave any network traces and would be nearly impossible to track. 

- **Malicious Content Generation:** In this threat scenario, a malicious actor has used a locally deployed LLM in order to make highly sophisticated phishing emails, create malware, generate fraud documents, or create disinformation for a social engineering campaign etc. In this scenario the forensic investigation aims to attribute the creation of this content to the suspects machine. 

- **Contraband Data Processing:** A suspect under investigation has allegedly used a locally deployed LLM to process illegal data. For example, summarization of stolen documents, trade secrets. Recovering the prompts and outputs is essential evidence. 

- **Attribution and Reconstruction:** Once an investigator discovers questionable documents and needs to further identify and link it with the suspects computer. This investigation shall then focusses detecting the specific software that was used to deploy the LLM locally, specific model used, if possible, then the configuration parameters used to infer from it and finally the most important, the exact sequence of textual prompts that generated the final output. 

### **3.2 Forensic Assumptions and Scope** 

This paper works under the following scope and fixed assumptions: 

- **Scope:** The forensic analysis is confined to the extraction of artifacts found on a host workstation having Windows or Linux operating system. Multi tenant, server based hosts, cloud based environments are out of our scope of work. These areas are highlighted as important areas for future work[29]. 

- **Assumptions:** This work assumes that the forensic investigator is already having the relevant permissions from the legal entities for full physical or logical acquisition of target systems memory and storage. The subject under investigation is not presumed to a highly capable state actor or a sophisticated individual employing advanced anti forensics techniques like full disk encryption, file shredding or using live bootable operating system. The impact of these mentioned techniques are discussed in section 9. Our 

5 

_3.3 Legal and Ethical Framework_ 

primary objective is to recreate the suspect who is under investigation for interactions with the LLM software. 

### **3.3 Legal and Ethical Framework** 

The novel nature of locally deployed language models and corresponding evidence requires deep and careful planning of legal and ethical principles to ensure that integrity and admissibility of the evidence remains unquestionable. 

- **Chain of Custody:** The investigator must keep the chain of custody maintained for each and every digital evidence. The investigator must record all the steps from the very (image acquiring) till the final analysis step of all the collected artifacts. This must be maintained to show that all the evidence is untampered [31, 32]. 

- **Evidence Admissibility:** Artifacts extracted from local LLMs must adhere to the established standards for reliability, such as the _Daubert_ standard in U.S. federal courts[9]. Key _Daubert_ factors involve whether the technique described can be tested/re tested, has prior identified and calculated error rates, has been peer reviewed and is accepted in the scientific community. The steps described in this work are reproducible and have quantitative results. This work itself has peer review nature which helps in satisfying these criteria. The recently proposed Federal Rule of Evidence 707, dealing with machine generated evidences, underscoring the need to demonstrate the strength of the forensic process that produced the output[33, 34]. The analysis of the configuration files and associated model metadata helped in identifying and laying out the "process" that resulted in generating a particular AI response. 

- **Privacy:** The recovery of a particular plaintext prompt is often associated with deep privacy concerns[22]. The chat logs can often have sensitive personal, medical, financial or even proprietary information that was given by the user under the impression of the communication being confidential. The investigator looking for evidence of a particular crime may come across such a scenario. The complication arising from this stems from the disruption of the "plain view" doctrine and puts an ethical obligation on the investigator to ensure extreme care in managing the data. The scope of analysis must remain strictly within the scope of the original obtained legal warrant[36]. 

## **4 Targeted Software and Deployment Scenarios** 

To ensure that the methodology presented here is reproducible and valid, this work uses the below specific versions of the software under study. They were installed on a clean, controlled virtual machine environments. The architectural 

6 

_4.1 Ollama_ 

differences present in these tools result in fundamental differences in the types and locations of forensic artifacts produced. 

Table 1: Experimental Software and Environment Configuration 

|**?**__`tab:software-env`__**?**<br>Software|Version|Operating System|Installation<br>Date|Installation Method|
|---|---|---|---|---|
|Ollama|v0.11.8|Windows 11 Pro<br>(23H2)|2025-08-29|`https:`<br>`//ollama.com/install.sh` | sh|
|Ollama|v0.11.8|Ubuntu 24.04 LTS|2025-08-29|`https:`<br>`//ollama.com/install.sh` | sh|
|LM Studio|0.3.24|Windows 11 Pro<br>(23H2)|2025-08-29|LM-Studio-setup-0.2.22.exe|
|LM Studio|0.3.24|Ubuntu 24.04 LTS|2025-08-29|LM_Studio-0.2.22.AppImage|
|llama.cpp|b6316<br>(Git)|Windows 11 Pro<br>(23H2)|2025-08-29|Download from Releases section|
|llama.cpp|b6316<br>(Git)|Ubuntu 24.04 LTS|2025-08-29|Download from Releases section|

### **4.1 Ollama** 

Ollama [2] is an easy to use opensource framework that enables easy and quick deployment of Large language models (LLMs). It is written in the Go programming language and it hides all complications associated with running a Large Language Model, dependencies and other configuration issues by providing a simple abstract interface to the user to interact with the language model. Central to Ollama is a client server architecture that has a server using the llama.cpp library to deploy a language model in a highly optimized manner on the CPU and GPU (if available). Ollama also provides a Command Line Interface (CLI) that interacts with this server. Multiple Graphical User Interfaces (GUI) are also available that connect with the Ollama server. Each Ollama model is associated with a modelfile whose functionality is to provide users with a way to customize and create newer models by defining or redefining network parameters such as temperature, top_p and system prompts. These models are not the exact complete models, instead they are Quantized versions of the corresponding original models. Quantization is a process in which the numerical precision of a model is reduced to decrease the models size and computational requirements. Different levels of quantization results in different sized models, with a smaller sized quantized variant being lesser capable than a quantization variant with a larger size. With these optimizations sophisticated models are able to run on consumer grade hardware directly. Ollama also provides a full REST API to enable seamless integration with different applications. All the data remains on the system that hosts the server including the models. This design makes it an important and ideal tool of deployment by users for offline purposes. 
