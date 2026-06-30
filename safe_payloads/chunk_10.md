
_Timeline Analysis._ Timeline analysis bridges Q3 and Q4, and describes the process of moving from having a timeline to reconstructing events, which uses refinement techniques such as: filtering irrelevant entries, highlighting key entries, or aggregating entries into more meaningful events (Hargreaves & Patterson, 2012). Several other concepts have been discussed such as event abstraction (Studiawan et al., 2020a; Studiawan, 2023), the application of machine learning (Khan & Wakeman, 2006), or visualization (Berggren et al., 2024; Debinski et al., 2019). Timeline analysis also draws in _examiner knowledge_ to understand potential events that are capable of producing the timeline entries and integrating them into a reasoning process (Gladyshev & Patel, 2004). 

_Q4: Hypotheses and Event Inference._ To accurately approach event reconstruction, it is essential to distinguish between the event _E_ that occurred in reality and the inferred event _E_<sup></sup> which is derived from the analysis of timeline entries. In the context of hypothesis generation, _E_<sup></sup> represents the best approximation 

based on the available evidence. We define an inferred event _E_<sup></sup> as _a reconstructed scenario that may have occurred within a specific time frame, based on the interpretation and analysis of timeline entries and associated artifacts._ This definition acknowledges the uncertainty in reconstructing past events. 

Consideration of the timeline entries in the context of examiner knowledge may result in multiple plausible scenarios (Jaquet-Chiffelle & Casey, 2021; Gladyshev & Patel, 2004). Hargreaves (2009) states if there are multiple events that could cause the same state of digital data, there is an actual, true event that caused it, and one or more other events that did not. This means that rather than arriving at a single definitive inferred event _E_<sup></sup> , we may generate _k_ alternative events, denoted as _E_<sup></sup> _j_ where 1  _j_  _k_ . Each _E_<sup></sup> _j_<sup>representsadistinctinterpreta-</sup> tion of the evidence, each of which could potentially explain the observed data. These multiple instances of _E_<sup></sup> highlight the complexity and ambiguity, where different sequences of events could produce similar artifacts. The process involves not only constructing these alternatives but also systematically and repeatedly testing and eliminating hypotheses to converge on the most likely scenario while acknowledging that multiple interpretations may still be viable based on the available evidence. To test and eliminate hypotheses, Casey (2020)s Strength of evidence scale (C-Scale) may be used, and it may involve research into artifact interpretation and experiments to determine if a set of actions could produce the observed system changes. 

## **5. Methodology for challenge identification** 

To identify and categorize the challenges in event reconstruction, we followed a structured literature review process designed to balance breadth with relevance. The goal was not to exhaustively capture all existing work but to obtain a representative and insightful overview of the key challenges discussed in the field. 

**Search strategy:** We defined a set of core search terms related to the topic: event reconstruction, timeline, timestamp anal- 

4 

Table 1: Summary of Systematization of Knowledge (SoK) for Timeline-based Event Reconstruction (TER) 

|**Paper**<br>**Sec. 2 Event reconstruction**|**Focus area**|**Contribution type**/**Challenge**|**TERquadrant**<br>**Data sou**<br>**Q1**<br>**Q2**<br>**Q3**<br>**Q4**<br>**Physical**<br>**File system**<br>**Multi sources**<br>**Logs**<br>|**rce categ**<br>**Other**<br>**Timestamp**<br>**Analysis**|**ory**<br>**Mobile**/**IoT**<br>**Volatile**<br>**Network**|
|---|---|---|---|---|---|
|Lee et al.(2001)<br>Carrier & Spaford(2004a,b)<br>Casey(2011)<br>Chabot et al.(2015a)<br>Adderley & Peterson(2020)<br>Willassen(2008a,b)<br>Batten et al.(2012)|Foundational event reconstruction<br>Event-based investigation process<br>Temporal, relational analysis<br>Terminology, data volume<br>Temporal sequencing<br>Hypothesis testing<br>Hypothesis development|Conceptual framework<br>Process model<br>Analytical framework<br>State-of-the-art review<br>Timeline correlation<br>Model-based reconstruction<br>Reasoning methodology|<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>|<br><br><br>|<br><br><br><br><br>|
|Xu & Xu (2022)|Knowledgegraph reasoning|Visualization and reasoningmodel|<br>|||
|**Sec. 3 Terminology**<br>Neale(2023)<br>Carrier & Spaford(2004a,b)<br>Hargreaves & Patterson(2012)<br>Marrington et al.(2007)<br>Neasbitt et al.(2014)<br>Chabot et al.(2014)<br>Jaquet-Chifelle & Casey(2021)<br>Harichandran et al.(2016)<br>Horsman(2019)<br>C t l(2022)|Artifact terminology harmonization<br>Event-based investigation process<br>Event granularity<br>Computer activity<br>User interaction terminology<br>Duration-based event defnition<br>Forensic event structure<br>Artifact properties analysis<br>Artifact as digital object<br>Atift dfiti|Systematic terminology review<br>Process model<br>Event granularity<br>Activity terminology<br>Interaction terminology<br>Terminology refnement<br>Forensic event model<br>Artifact comparison<br>Practical defnition<br>Atift tl|<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>|<br>|<br><br><br><br>|
|asey e a. <br>Lyle et al. (2022)<br>|rac enon<br>Artifact identifcation|rac caaog<br>Digital investigation techniques|<br><br>|||
|**Sec. 4 Model for event reconstruction**<br>Ribaux(2014,2023)<br>Vanini et al.(2023)<br>Vanini et al.(2024b)<br>Carrier(2006)<br>Hargreaves(2009)<br>Jaquet-Chifelle & Casey(2021)<br>Hargreaves et al.(2024b)<br>Raghavan & Saran(2013)|Forensic trace model<br>Event source reliability<br>Time anchor model<br>Investigation process model<br>Evidence reliability testing<br>Event structure<br>Tool transparency<br>Timestamp interpretation|Trace-based model<br>Reliability modeling<br>Timestamp interpretation framework<br>Hypothesis-based model<br>Reliability criteria<br>Formal event model<br>Tool capability model<br>Timestamp model|<br><br><br><br><br> <br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>|<br><br>|<br><br>|
|Hargreaves & Patterson(2012)<br>Studiawan et al.(2020a);Studiawan(2023)<br>Carrier & Spaford(2004a,b)<br>Gladyshev & Patel(2004)<br>|Timeline generation model<br>Event abstraction<br>Hypothesis-based investigation<br>Event inference<br>|Timeline generation model<br>Event abstraction model<br>Hypothesis model<br>FSM reconstruction<br>|<br><br><br><br><br><br><br><br><br><br><br><br><br><br>|<br><br><br>|<br><br><br><br>|
|Amato et al.(2017)<br>Xu & Xu (2022)|Semantic evidence correlation<br>Knowledgegraphpresentation|Ontology-based model<br>Reasoningmodel|<br><br><br><br><br>|||
|**Sec. 6 Challenges stemming from environme**<br>**Sec. 6.1.1 Incorrect environment time**<br>Stevens(2004)|**ntal and process-related factors**<br>Misconfgured system clocks|Clock drift challenge||||
|<br>Rahavan & Saran(2013)|<br>Timestam normalization and storae issues|<br>Timestam interretation framewor|<br><br>|||
|g <br>Vanini et al.(2024b)<br>|p   g<br>Time anchor abstraction model<br>|p p<br>Time anchor modeling<br>|<br><br><br><br><br><br><br>|<br><br><br><br>|<br><br><br><br><br><br>|
|Kaart & Laraghy(2014)<br>|Incorrect timezone data handling<br><br>|Time zone confguration<br>|<br>|<br>|<br><br>|
|Schatz et al.(2006);Buchholz & Tjaden(2007)|Network-induced skew, unsync clocks|Distributed system time consistency|<br>|<br>|<br>|
|Henderson(2009)<br>**Sec. 6.1.2 Confgurations and implementatio**|Clock skew in shared environments<br>**ns**|Network delay and skew|<br>||<br>|
|<br>Adedayo & Olivier(2015)|<br>Log suppression redirection|Log misconfguration|<br><br>|||
|<br>Fernndez-Fuentes et al.(2022)<br>**Sec. 6.1.3 Environmental anomalies**<br>Studiawan et al.(2019)<br>Oh et al.(2022)<br>Marrington et al.(2011)<br>**Sec. 6.1.4 Data fuctuation**<br>Sandvik et al.(2021)<br>Marangos et al.(2016)|,<br>Absence of traceability in apps<br>Unrecoverable system restarts<br>Sudden device restarts<br>Program faults, data corruption<br>Short lifespan of traces<br>Evidence afected by operational cycles|<br>Limited logging capability<br>Environmental disruption<br>Restart-induced log gaps<br>Software instability<br>Volatile trace loss<br>Temporal instability|<br><br><br><br><br><br><br><br><br><br><br>|<br><br><br><br><br>|<br><br><br><br><br><br><br>|
|<br>**Sec. 6.2 Post-event period**<br>Gruber et al.(2023)<br>Jaquet-Chifelle & Casey(2021)<br>Khan et al.(2007)<br>Soltani et al.(2019);Schuster(2007)|<br>Evidence altered during acquisition<br>Evidence fragility and impermanence<br>Overwriting of data, log aging<br>Metadata decay, inaccuracy|<br>Contamination challenge<br>Temporal evidence integrity<br>Aging challenge<br>Artifact degradation|<br><br><br><br><br><br><br><br><br><br><br>|<br><br>|<br><br><br><br>|
|**Sec. 6.3 Timeline**<br>Patterson & Hargreaves(2012)|Cross-source correlation|Source integration challenge|<br><br><br>|||
|<br>Mohammed et al.(2016)<br>Horsman(2019)<br>Soltani & Seno(2017)<br>|<br>Data format diversity<br>Artifact parsing complexity<br>Missing/incomplete timestamps<br>|<br>Data normalization challenge<br>Parser dependency challenge<br>Extraction incompleteness<br>|<br><br><br><br><br><br><br><br>|<br>|<br><br>|
|Gmez et al.(2005);Levett et al.(2010)<br>Klber et al.(2013);Hargreaves et al.(2024b)<br>Bhat et al.(2021)|Correlation of heterogeneous data<br>Tool transparency and automation limitations<br>Misconfgured analysis environments|Multi-source correlation<br>Human-tool balance challenge<br>Tool setup challenge|<br><br><br><br><br><br><br>|||
|**Sec. 6.4 Decision making**<br>Chabot et al.(2015a)<br>Quick & Choo(2014)<br>Buchholz & Falk(2005)<br>|Data volume for timeline analysis<br>Computational resource limitations<br>Event aggregation<br>|Scalability and overload challenge<br>Resource requirement challenge<br>Event abstraction for analysis<br>|<br><br><br><br><br><br><br><br><br>|||
|Kiernan & Terzi(2009)<br>Osborne & Turnbull (2009)|Event summarization<br>Visualization accuracy|Abstraction and streamlining<br>Visual representation integrity|<br><br>|||
|**Sec. 7 Challenges stemming from deliberate**<br>Casey(2020)|**interference**<br>Strength and scale of inference|Evaluative opinion framework|<br>|||
|<br>Vanini et al(2024b)|<br>Time maniulation clock tamerin|<br>Timeframe maniulation|<br>|||
|. <br>MITRE(2023)<br>|p,  pg<br>Environment manipulation, disabled logging<br>|p<br>Environment tampering<br>|<br><br><br>|||
|Conlan et al.(2016)<br>Palmbach & Breitinger(2020)|Erasure or alteration of evidence using tools<br>File and log manipulation using malware|Anti-forensics tool usage<br>Malware-assisted anti-forensics|<br><br><br>|||
|Malhotra et al.(2015)<br>Choi et al. (2021)|Service manipulation (e.g., NTP tampering)<br>Post-event manipulation: logs, timestamps, fles|Service compromise<br> <br>Artifact modifcation & deletion|<br><br>|||

Notes: <u></u> Mentioned in the paper <u></u> Not specifically mentioned, but can be implemented using the data source 

ysis, digital forensics, correlation, challenges, and problems. These terms were combined using Boolean operators and phrasing variations (e.g., quotation marks for exact matches). Searches were conducted using Google Scholar, which indexes most major academic publishers (e.g., IEEE, ACM, Wiley, Springer) and relevant platforms such as DFRWS.org and arXiv. 

**Selection criteria:** For each query, we considered the first two pages of results (i.e., 20 entries). Articles were initially screened based on metadata displayed: title, author(s), publication venue, and two-line extract. If no direct reference to 

digital forensics was evident, the article was discarded. This filtering yielded a preliminary pool of approximately 200 articles. 

- **Challenge extraction:** We extracted mentions of challenges primarily from the abstract and introduction sections, where such content is frequently summarized. Targeted keyword searches (e.g., challenge, problem, limitation) were also used within full texts to uncover implicit references. 

- **Classification:** The identified challenges were then mapped onto a diagram, categorizing them according to the stage or context in which they occur within the event reconstruction 

5 

process. 

We also incorporated our domain expertise to address gaps in the literature, recognizing that some relevant challenges may not have been explicitly highlighted in existing works. 

_Limitations._ The article collection and analysis were conducted manually, which may have led to the omission or misclassification of relevant articles. By restricting searches to Google Scholar and considering only the first two pages of results, important sources further down the list or from other databases may have been excluded. The focus on abstracts and introductions might have caused us to overlook challenges discussed deeper within the papers. Moreover, the subjective nature of challenge classification introduces potential bias based on the researchers interpretations. Finally, the absence of automated or statistical tools for extraction and categorization limits the objectivity and comprehensiveness of the analysis. Despite these limitations, we believe the following sections offer a comprehensive and nuanced overview of the challenges. 

## **6. Challenges stemming from environmental and processrelated factors** 

This section focuses on _unintentional_ challenges and the structure follows the diagrams flow, discussing each quadrant. 

Note, that while we have strived to define the challenge categories as distinctly as possible, some overlap is inevitable due to the interconnected nature of these activities. Certain actions may reasonably fall into multiple categories, depending on the context. The categorization is designed to provide guidance rather than enforce strict mutual exclusivity. 

- **Time zone changes:** As systems traverse different time zones, whether due to travel or daylight-saving time changes, the system time may change (Stevens, 2004). This adjustment process can also be error-prone, e.g., due to an inaccurate time zone database (Kaart & Laraghy, 2014). Compared to skew and drift, the range is significantly larger, i.e., hours instead of seconds. Typically this is only relevant where local time is stored in a data structure rather than storing UTC. 

Note that virtual environments come with their challenges which are beyond the scope of this article but have been discussed in VMware (2008). 

## _6.1.2. Configurations and implementations_ 

Environments, systems, and application configurations define how/what data is generated, stored, and logged. These configurations comprise a wide range of settings, including logging levels, storage policies, network settings, and security controls. 

- **Suppression** / **deletion:** Conservative default settings can result in insufficient logging, leading to missing artifacts, e.g., database logs prioritizing space efficiency over detail (Adedayo & Olivier, 2015). Systems may also be configured to suppress artifacts, such as private browsing (FernndezFuentes et al., 2022), or delete them, such as printer jobs removed after completion (Gladyshev & Patel, 2004) or when an application is closed. 

- **Inconsistent implementations:** Different resolutions lead to inconsistencies, e.g., timestamps recorded in hh:mm vs. hh:mm:ss format (Song et al., 2016). File systems, drivers, and implementations may behave differently leading to unpredictable behavior (Bang et al., 2009; Nordvik & Axelsson, 2022). 

## _6.1.3. Environmental anomalies_ 

## _6.1. Q1: Timeframe of interest_ 

Four areas have been identified: 

## _6.1.1. Incorrect environment time_ 

Clock-related challenges originate from the system time which is used to derive timestamps. If the clock is incorrect, all timestamps originating from this clock are incorrect (Stevens, 2004; Raghavan & Saran, 2013; Vanini et al., 2024b). 

- **Clock skew:** Skew refers to the difference in time readings between different systems. One reason for clock skew could be propagation delays which may occur due to network delays (Schatz et al., 2006; Henderson, 2009) or due to synchronization problems, e.g., NTP servers providing incorrect times (Buchholz & Tjaden, 2007; Hampton & Baig, 2016). 

- **Clock drift:** Drift is the gradual deviation of a clock from the correct time, often caused by factors such as changes in temperature, voltage fluctuations, or inherent defects in the clock circuitry (Sandvik & rnes, 2018). Clock drift may exacerbate over time. As drift accumulates, the discrepancies between different systems clocks can grow, making it increasingly difficult to correlate events across environments (Becker et al., 2008). 

Environments may not behave as expected leading to the destructing of evidence or the not-creation of artifacts: 

- **(OS) Crashes:** A crash (system, application) can result in the loss or corruption of artifacts, potentially leaving logs incomplete and missing key events (Studiawan et al., 2019; Oh et al., 2022). Detecting crashes can be challenging, particularly if the logging mechanisms themselves are compromised during the crash. Crashes may also lead to restart anomalies such as services or applications that are supposed to start automatically failing to do so potentially altering the way subsequent events are logged. 

- **Software bugs:** Bugs in software may cause errors in data logging, such as incorrect timestamps or missing events (Marrington et al., 2011). 

- **Resource exhaustion and failure:** Environments under heavy load may fail to log events properly due to resource constraints, leading to delayed or missed entries in the event data. Failures, including hardware malfunctions, can lead to inadequate data (Marrington et al., 2011). 

## _6.1.4. Data fluctuation_ 

Data may not be accessible due to or only with additional burden: 

6 

- **Data volatility:** Volatile data, such as RAM content or network traffic, is lost if the  is too large. In addition, IoT devices often have resource constraints resulting in short-lived data (Sandvik et al., 2021). In cloud environments, VMs can be easily deleted including their logs (Marangos et al., 2016). 
