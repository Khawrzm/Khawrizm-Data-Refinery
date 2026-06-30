
## **3. Terminology** 

According to Neale (2023), there is a lack of harmonization in terms and definitions. This section briefly revisits (Sec. 3.1) and then highlights the terminology we use for this article (Sec. 3.2). 

## _3.1. Terms and terminology in existing literature_ 

Carrier & Spafford (2004a) define an event as an occurrence that changes the state of one or more objects. Over time, researchers suggested to differentiate between low-level and highlevel events (human-understandable) (Hargreaves & Patterson, 2012; Vanini et al., 2024b) or introduced terms such as activity (Marrington et al., 2007) or user-browser interaction and click which are used interchangeably by Neasbitt et al. (2014). Chabot et al. (2014) defines an event as a single action occurring at a given time and lasting a certain duration. 

Jaquet-Chiffelle & Casey (2021) define an event as a complete collection of related things that have happened (or are happening) in a World within a specific closed interval of time. [...] The Event can be considered as a whole entity or as a collection of smaller sub-events. Notably, their framework emphasizes the role of traces and introduces several key concepts, including trace, facet, and observable facet. While these terms are well-established in forensic science (Ribaux, 2023), they are less common in digital forensics. Therefore, we adopt a different terminology, while drawing conceptual links to their work. 

Similarly, the term _artifact_ is used with different meanings. For instance, Harichandran et al. (2016) compares various definitions and concludes properties an artifact should have such as artificiality/external force, antecedent temporal relation, and exceptionality. Horsman (2019) suggests a digital object containing data which may describe the past, present or future use 

or function of a piece of software, application or device for which it is attributable to. Casey et al. (2022) differentiates between atomic artifacts (a singular unit of interpretable data that can be extracted from a given data source) and dependable artifacts (one or more atomic artifacts needed to expose the atomic artifact of interest). Lyle et al. (2022) extends the atomic artifact definition by adding ...that is useful for addressing questions in forensic investigations, but assessing usefulness is difficult, subjective and may change over time. 

## _3.2. Terminology used in this article_ 

_Environments_ / _systems._ An environment/system is a computational setting or a software/hardware system that reacts to events such as user actions, API calls, or sensor inputs. Typically, it is one or more devices such as computers or smartphones but it could also be a virtual machine, network device, or cloud environment. For readability, the remainder of this paper uses the term environments instead of environments/systems. Note we use the plural, i.e., environments, considering that changes may be in one or more environments, locally, remotely, or both. 

_Artifact._ This article uses Casey et al. (2022) atomic artifact definition: a singular unit of interpretable data that can be extracted from a given data source. For simplicity, we will only say artifact throughout the paper. Examples include log files, registry keys, timestamps, or network traffic data. 

_Event._ Based on Jaquet-Chiffelle & Casey (2021), an event is a complete collection of related things that have happened (or are happening) in a World within a specific closed interval of time. These can be treated as a singular entity or decomposed into smaller sub-events and cause environmental changes. This broad definition provides the flexibility for an event to be at the resolution of: file was accessed, or Google search was performed, or user account was used to run a program (consisting of at least two events: user logged in and user executed binary). Events can be triggered internally, e.g., a cron job, or externally, e.g., someone clicking the mouse. Note that the distinction between event and sub-event is blurred and it is up to the user to define the granularity. For instance, 

- an event is _sending an email_ with sub-events such as opening the email client, typing, establishing a connection to the SMTP server, and sending the message, or 

- an event is _establishing a connection to the SMTP server_ with sub-events such as performing a DNS lookup, initiating a handshake, and authenticating the user credentials. 

## **4. Model for event reconstruction** 

This work draws inspiration from Vanini et al. (2023), which, in turn, is influenced by the work of Ribaux (2023, p226, Fig. 4.4)<sup>2</sup> . We adjusted these models to align with standard digital forensics terminology and emphasize timeline-based event reconstruction. Our model, named _TER-Model_ (timeline-based 

> 2Note, this is an updated version from the previous work by Ribaux (2014) and thus has over a decade of history. 

2 

event reconstruction), is depicted in Fig. 1 and can be separated into a _reality_ space (Sec. 4.2) and a _reconstruction_ space (Sec. 4.3). Each of these spaces can be further separated resulting in four quadrants (Q1-Q4). Before describing the model, this section first summarizes the goals of temporal event reconstruction which influenced the TER-Model. The summary of systematization of knowledge (SoK) in the TER-Model is shown in Table 1. 

## _4.1. Goals of temporal event reconstruction_ 

Temporal event reconstruction aims to accurately recreate the sequence of events that occurred which includes finding gaps and inconsistencies, even if they cannot be accurately filled or corrected. Thus, it enables investigators to draw meaningful conclusions about what transpired. 

Event reconstruction involves several interrelated analytical processes that together provide a coherent and defensible narrative of what transpired. At its core is temporal sequencing and correlation, where a precise order of events is created. It may be necessary to analyze their relationships across different timelines to uncover causal links, sequence dependencies, or concurrent activities (Adderley & Peterson, 2020). Beyond simple chronology, contextual analysis places these events within a broader framework, considering factors such as user behavior, system settings, or external influences to give the data deeper interpretive meaning (Chabot et al., 2015a). This groundwork supports hypothesis testing and scenario building, where investigators construct and refine possible explanations for what occurred, evaluating multiple narratives and ruling out those that conflict with the evidence (Willassen, 2008a,b; Batten et al., 2012). It is crucial that the reconstructed timelines are confirmed through correlation and verification of evidence to ensure consistency and reliability. The goal is to produce a report to support legal proceedings that not only stands up to technical scrutiny, but also serves court proceedings by providing a clear, accurate and accessible story for stakeholders such as lawyers or jurors (Chabot et al., 2014; Xu & Xu, 2022). 

## _4.2. Reality and its two dimensions (Q1, Q2)_ 

_Q1: Timeframe of interest T._ This quadrant is an interval that has a start time _tS_ and an end time _tE_ , i.e., _T_ = [ _tS_ , _tE_ ] during which the event ( _E_ ) and sub-events ( _e_ 1, _e_ 2, ... _em_ ) occurred. Each _E_ or _e_ causes multiple environmental changes, e.g., new log entries, modified registry values, files marked as non-allocated, or updated timestamps. 

The event (E) is what we wish to be able to say something about through the event reconstruction process. Carrier (2006) describes that an event can be any an occurrence that changes the state of the system and Hargreaves (2009) continues that digital events occur on a system often as a result of interactions with another digital device, or as a result of interactions with the real world. However, in Jaquet-Chiffelle & Casey (2021) event is formalized such that these external triggers are integrated into the event itself, defining an event that can capture the very broad, or the very detailed. In addition, there are _concurrent events_ such as antivirus scanning files resulting in changes not tied to the primary event. 

_Q2: Post-Event Period (_  _)._ During this interval , the environment changes caused by _E_ may become intermingled with, altered, or overwritten by an ensemble of other data generated by unrelated _subsequent events_ . Jaquet-Chiffelle & Casey (2021) categorized these changes as adjunction, suppression, and change. This second interval ends at time _tP_ when the data is preserved/extracted, i.e., = ( _tE_ , _tP_ ]. As _tE_ belongs to _T_ , we exclude it here from this interval using a half-open interval. It is important to note that not all environment changes can be extracted, such as missing/deleted files or new artifacts without a parser. These gaps may stem from many causes, for example a lack of knowledge in digital forensics, a tool setup, or errors in the timeline generation process. Hence, what can be extracted is named _extractable artifact_ , which is therefore context specific. 

_Timeline Generation._ Combined with preservation and acquisition, timeline generation bridges the Reality and Reconstruction spaces. Hargreaves et al. (2024b) define it as a process within a forensic analysis tool for extracting timestamps from the file system...[and] applying file specific processing and extracting timestamps from within files such as the Windows Registry, log files, SQLite databases etc., that contain timestamps. This artifact and timestamp extraction is complemented by normalization, which is required since timestamps exist in a variety of formats (e.g., ASCII in a log vs. little-endian hexadecimal in a proprietary format), and resolutions (i.e., hours, minutes, seconds, nanoseconds, etc.) depending on their source (Raghavan & Saran, 2013). They may also be stored in UTC or local time. Ideally, after normalization, all timestamps should be presented in the same format for better readability and sortability. 

## _4.3. Perception_ 

The lower section of the diagram represents how examiners attempt to reconstruct past events using reasoning and available evidence. This process involves uncertainty, as the past cannot be revisited, making absolute certainty unattainable. 

_Q3: Timeline._ Examiners construct a timeline to facilitate analysis, and the DFPulse 2024 Practitioner Survey (Hargreaves et al., 2024a) reports 80.3% are using timelines often or almost always. Timelines are composed of a series of entries, each derived from individual artifacts that are arranged chronologically. Artifacts may originate from multiple independent data sources, e.g., a computer and a smartwatch. While specific implementations store multiple data points per event, fundamentally these _timeline entries_ are defined as a 3-tuple ( _t_ , _S_ , _C_ ): 

- The normalized timestamps ( _t_ ) are used to order the timeline chronologically. 

- A source _S_ refers to the specific location from which the timestamp and context originate, such as the Master File Table (MFT), Windows registry, EPROCESS block in memory, or Chrome browser history file. For clarity, _S_ should be as detailed as possible; instead of stating the registry, the exact registry key path should be specified. 

- A context _C_ defines what the timestamp represents, such as the modification timestamp within the Standard Information Attribute (SIA) of MFT entry, or a value in a specific row or field within a database. Given the wide variety of 

3 

<!-- Start of picture text -->
Q1 Timeframe of Interest ( T ) Q2 Post-Event Period ()<br>Environments/Systems<br>Potential  Subsequent eventsSubsequent events<br>external trigger cause- suppression<br>Event (E) causes Environments/Systems - modification<br>changes<br>concurrent<br>events<br>extractable Artifacts<br>Timeline generation<br>Reality Artefact extraction Time Generation<br>Reconstruction Timestamp extraction<br>Examiner  Timestamp normalization<br>knowledge<br>Inferred events ( E )<br>Timeline<br>Timeline  Timeline entry 1 (Timestamp, Source, Context)<br>generates analysis Timeline entry 2 (Timestamp, Source, Context)<br>Hypothesis (filtering,<br>searching,  Timeline entry 3 (Timestamp, Source, Context)<br>Hypothesis testing uses additional categorizing)aggregating, labelling, Timeline entry n  (Timestamp, Source, Context)  <br>(research, experimentation)<br>Q4 Decision Making Q3 Timeline<br><!-- End of picture text -->

Figure 1: TER-Model: Model of timeline-based event reconstruction in digital crime scenes. The small squares (3x4) in the upper part of the diagram represent changes by the primary event (gray box) and additional changes from subsequent events (white-gray stripes). 

contexts, a generic term is used to encompass the diverse nature of these representations. 

These timeline entries should not be conflated with events themselves or low-level events (Hargreaves & Patterson, 2012). The context provided by each entry, such as a value in a modified or last change field within a file system structure, does not inherently represent a specific event, such as a file modification. Instead, it reflects environmental behavior that must be understood before making any assumptions about what event occurred. This distinction is critical: while timeline entries provide the raw data needed for event reconstruction, they are not events in and of themselves. Rather, they are normalized, sorted compilations of data that result from parsing artifacts left by events. Therefore, we argue that the term event should be reserved for the inferred actions, while the term timeline entry more accurately describes the data points that examiners use to reach those inferences. 
