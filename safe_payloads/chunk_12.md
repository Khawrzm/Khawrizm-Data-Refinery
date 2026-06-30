
- **Service manipulation:** Instead of manipulating an environment directly, an adversary may compromise utilized services. For instance, by manipulating the NTP service, an attacker can change the system time (Malhotra et al., 2015). Another example would be a compromised update server. 

## _7.2. Q2: Post-Event Period_ 

Post-event one may **manipulate or delete metadata or content** such as altering timestamps, modifying log entries, or deleting critical files (e.g., remote wiping of mobile devices). Logs and other files are often not protected against alternation or deletion (Choi et al., 2021). Active tampering and manipulation of artifacts present some of the most challenging obstacles in event reconstruction and the risk of misinterpretation increases (Casey, 2020) especially when performed from advanced persistent threads. 

## **8. Key findings** 

## **7. Challenges stemming from deliberate interference** 

To complement the previous section, this one outlines challenges stemming from deliberate actions such as backdating, erasing, or wiping, to hide activities (Casey, 2020). While it may not always be the case, for this work we assume that the investigative body and tool vendors are free from insider threats. Therefore, challenges are limited to the _reality_ . 

As already pointed out in Sec. 6, some overlap of challenges is inevitable due to the interconnected nature of these activities. 

## _7.1. Q1: Timeframe of interest_ 

Interference with the environment can be conducted before the event occurs, with the intent to complicate investigations. Such interference often seeks to generate misleading artifacts or prevent their creation altogether, e.g., examples under defence evasion in the MITRE ATT&CK Matrix<sup>4</sup> . 

- **Time manipulation:** An adversary may turn off set time and date automatically and actively manipulates the system time or timezone (Vanini et al., 2024b). Even when detected, distinguishing between accidental misconfigurations and deliberate tampering remains difficult. 

> 4 `https://attack.mitre.org/tactics/TA0005/` 

This section summarizes the key findings identified in the foundational sections 2 to 4, and the challenge identification sections 6 and 7: 

1. The terms event and artifact in digital forensics are defined inconsistently across existing studies and it leads to ambiguity in their usage. 

2. Event reconstruction relies on modeling two critical intervals: the timeframe of interest ( _T_ ) where events occur, and the post-event period () where subsequent changes may overwrite or obscure evidence. 

3. Event reconstruction is highly affected by unintentional challenges such as incorrect system time, insufficient logging, environmental anomalies, and data volatility. 

4. Subsequent events can delete, overwrite, or degrade digital artifacts; so they reduce the availability and reliability of evidence over time. 

5. Timeline generation faces challenges from data heterogeneity, software updates, extraction errors, normalization issues, and tool limitations. 

6. Event reconstruction requires careful hypothesis generation and testing, but faces challenges from data volume, correlation complexity, trust issues, and investigator bias. 

7. Deliberate actions such as time manipulation, antiforensics, and post-event tampering can alter or destroy digital evidence and make event reconstruction even more challenging. 

9 

8. Several research directions have emerged to address challenges in event reconstruction, including forensic readiness, improved artifact extraction, timeline verification, tamper detection, AI/NLP integration, and advanced analysis techniques. 

## **9. Discussion and research gaps** 

From the previous sections, the summary of key findings, and Table 1 (which provides a mapping of the focus areas in Sections 2 to 7, against the quadrants in Figure 1, illustrating the distribution of existing research) it is possible to infer general research gaps. However, this section highlights selected significant challenges and proposes specific potential avenues for future research. 

The section is organized by quadrant of the TER-model, demonstrating the utility of the model as an organizational tool. Given the vast body of literature, it is not feasible to reference every relevant article. Therefore, we focus on studies from our initial collection as well as recent works. 

One general point, is that throughout the TER-model (Q1Q4) a broad research gap is the understanding and handling of uncertainty, from system configuration through to a reliance on examiner knowledge for hypothesis generation and testing. This is considered an ongoing limitation to the process that requires addressing. 

**Research Gap 1.** Uncertainty is potentially introduced throughout the model and research into handling it at each stage, and how it could propagate is needed. 

## _9.1. Q1: Timeframe of interest_ 

Digital forensic readiness is a proactive approach ensuring systems and networks are prepared to efficiently collect, preserve, and analyze evidence when a security incident occurs (Sachowski, 2019). Forensic readiness for event logging has been researched, as demonstrated by Reddy & Venter (2013) and Kebande & Venter (2018). To support forensic readiness, administrators should activate extended logging, which records additional data and audit trails. Moreover, operating system developers could still provide more comprehensive system-related logs (Rivera-Ortiz & Pasquale, 2019) but this conflicts with privacy-centric approaches expected from consumers. 

This also has anti-forensics implications. If an attacker deletes logs (one of the primary sources for event reconstruction), investigators must first recover them (as discussed in Q2/Q3). To address this, security measures such as centralized or encrypted log servers could be implemented in systems where this is feasible, and even advanced techniques such as blockchain can be used to mitigate anti-forensic techniques (Kos & El Fray, 2020). 

**Research Gap 2.** Forensic readiness needs further development, and more creative solutions need researching to achieve similar goals on unmanaged systems where forensic readiness solutions cannot be deployed. 

## _9.2. Q2: Post-event period_ 

In evidence seizure, timing has an effect during forensic investigations. This affects if volatile artifacts are captured if not 

done on time, e.g., credentials stored in memory. Secondly, challenges related to cloud environments imply any delays in data acquisition may effortlessly cause the loss of crucial evidence, e.g., Alqahtany et al. (2016) discuss evidence that supports the need for timely acquisition. There is also the issue of long-term log retention by internet service providers, which may be important in some cases (Khan et al., 2016). Mandating extended retention ensures information can be accessed after an incident, but conflicts with privacy regulations. There are also awareness concerns. For victim systems, communication is crucial to ensure device owners minimize interactions with devices containing potential evidence. The same applies to examiners, where changes to the evidence should be anticipated and minimized from a data preservation/acquisition perspective (Gruber et al., 2023). Moreover, recent work by Spichiger & Adelstein (2025) highlights that preservation should not be narrowly focused on the trace itself but must also consider the reference environment in which the trace was produced. As systems evolve, e.g., through software updates, operating systems, or third-party services, insufficient preservation of reference data can result in a loss of contextual meaning and increase the uncertainty of later reconstructions. Expanding the definition of preservation to include such reference data is therefore essential in environments where evidence may need to be interpreted long after the fact. 

**Research Gap 3.** There is little work on the persistence of artifacts, and determining if the absence of data is due to configuration, tampering, or simply the passing of time. Work in this area could reduce this aspect of uncertainty within the model and process, and provide practical advice on the temporal boundaries of useful preservation periods. 

## _9.3. Q3: Timeline_ 

This aspect of event reconstruction has received the most attention and many articles and concepts have been discussed. 

- **Continuous updates** / **improvement to timestamp extraction:** Files and formats containing timestamps are subject to change. Ongoing research that tracks these changes and uncovers new timestamp sources provides the foundational data necessary. This means ongoing artifact research (as defined by Breitinger et al. (2024)) is critical. 

- **Integration of non-explicit timing information:** Dreier et al. (2024) discussed implicit timing (e.g., ordering of log file entries) to detect inconsistencies in an automated way. A second possibility is digital stratigraphy, as defined by Casey (2018), and further implemented in Schneider et al. (2024), which is a method that takes advantage of file systems and the behavior of their allocation algorithms. By analyzing the logical position of files on a disk, investigators can infer potential events, provided they understand how the file system allocates those files. This knowledge enables the reconstruction of hypothetical sequences of events based on file placement. These are still early implementations, and additional work is needed to evaluate more variations in environments, file systems, drivers, and behavior patterns. 

- **Timeline representation:** Timelines are mostly flat, i.e., textual files in chronological order. The community should explore alternatives. For instance, an ontology-based approach 

10 

improves event reconstruction by providing a structured and formal representation of data, which helps standardize and automate the analysis process (Bhandari & Jusas, 2020). An ontology captures the semantic relationships between events, objects, and subjects, allowing investigators to infer new facts, identify correlations between events, and visualize data more effectively (Chabot et al., 2015b; Turnbull & Randhawa, 2015). We should also reconsider visualizing timelines, moving beyond the frequently used basic bar charts counting the number of events within defined timeframes, and exploring AR or VR. 

- **Automated timeline verification:** Willassen (2008c) introduced a hypothesis-based approach where investigators create clock hypotheses to model historical clock values and test their consistency with timestamp evidence. Vanini et al. (2024b) suggested using time anchors (i.e., artifacts that include internal and external timestamps) and looking for anomalies. Research efforts need to continue to build verification methods that allow us to identify whether the timeline is out-of-sequence (irregularities found) or likely correct. 

- **Tamper detection:** Galhuber & Luh (2021) found that timestamp forgery tools may introduce detectable changes, such as reducing timestamp accuracy from nanoseconds to seconds. Among the tools they evaluated, only one was capable of modifying the full range of file system timestamps on Windows. Andrade (2020) noted that $FN timestamps are typically modified only by the Windows kernel and are generally unaffected by anti-forensic timestomping tools, offering an example of a timestamp that is harder to manipulate during event reconstruction. Jang et al. (2016) presented a method to detected time manipulation in NTFS file system. More general experiments as conducted by Schneider et al. (2020, 2022); Vanini et al. (2024a) show that the probability of detecting it is high, especially when it concerns file metadata. One reason is that it is difficult to forge a timestamp without causing subsequent inconsistencies. While some progress has been made in detecting tampering, this area still requires further exploration and automation. Ideally, a tool should be capable of analyzing a timeline and automatically highlighting all potential tampering events. 

**Research Gap 4.** Advances in timeline generation research are still needed in multiple areas: from artifact research, integration of non-timestamp-based timing information, visualization of timelines, and detecting inconsistencies and tampering. 

## _9.4. Q4: Analysis and investigative conclusions_ 

This includes the timeline analysis which bridges Q3 and Q4 since it may revisited as part of Q4 hypothesis testing. 

- **Timeline analysis:** Efforts focus on methods to reduce and manage data, including techniques for filtering, labeling, and aggregating data. Flagging entries that match certain criteria can be performed, or more complex approaches such as discussed by Hargreaves & Patterson (2012); Studiawan et al. (2020b) where patterns of events are bundled to provide multiple entries that support an event reconstruction. This reduces large timelines to more manageable sets of interesting events, but as they are inherently a reduced set, switching 

back to the lower-level entry view is an important feature to retain to see inferred events in context and show provenance of the reconstructed event. A limitation discussed by Hargreaves & Patterson (2012) is the need to manually create the patterns that need to be matched based on research and experience. Better centralized documentation of the expected changes from sets of actions in different environments, similar to Casey et al. (2022); Grajeda et al. (2018) and integration into a standard timeline analysis tool would make timeline analysis more accessible. 

Visualization is also a vital additional layer of abstraction to help make sense of the large amounts of data, and can be a valuable tool to assist with analysis, e.g., to support timelinebased cross drive analysis (Patterson & Hargreaves, 2012). An increased availability of ground truth data sets with annotation of the actions carried out would assist with developing analysis plugins for tools (Grajeda et al., 2017). Automated event inference, either using machine learning, or through automation in digital forensic experimentation to carry out actions and record the resulting traces may help with this. 
