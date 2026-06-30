
**_2) IoT and Mobile security:_** IoT security surveys stress that investigations span an ecosystem rather than a single host. The survey in [6] shows that evidence is fragmented across devices, companion mobile apps, gateways, and cloud backends, complicating acquisition, correlation, and chain-of-custody management. Similarly, [17] explains how IoT scale and heterogeneity constrain evidence visibility and attribution, motivating cross-source fusion and structured investigation. The work in [18] further reviews forensic requirements and tools, emphasizing automation and learningbased assistance for diverse devices and growing evidence volume, while [19] shows that minor artifacts such as application records and device interactions can be decisive when traditional logs are incomplete. Unlike IoT security surveys, wireless-oriented surveys mainly focus on functions. The review in [20] examines physical-layer identification and shows how hardware-imposed radio features support device attribution, while [21] extends this to device fingerprinting across protocol layers and discusses robustness issues affecting evidentiary reliability. More recently, [22] surveys radio frequency (RF) fingerprinting with emphasis on deep learning pipelines and datasets, and [23, 24] review wireless intrusion and misbehavior detection through semantic and behavioral inconsistencies. In summary, IoT security surveys explain where evidence resides and why crossdomain correlation is necessary, while wireless surveys provide technical blocks for attribution and anomaly sensing. 

## **1.3 Contributions** 

While many surveys exist on digital investigations, most works do not study the problem from a wireless evidence perspective. Existing reviews of digital, network, and cloud forensics mainly focus on investigation phases, toolchains, and trace correlation across distributed systems, with emphasis on logs, files, software artifacts, and network telemetry rather than radio observations [12, 16]. IoT and mobile security surveys are closer to wireless scenarios, as they emphasize heterogeneous endpoints, limited device visibility, and multi-party evidence ownership [6, 17]. However, they rarely treat radio measurements and time-varying channels as primary evidence requiring calibration and uncertainty modeling. By contrast, wireless security surveys need to address device identification, RF fingerprinting, and anomaly or misbehavior detection [22, 24], focusing on classification robustness and online detection accuracy. Therefore, as summarized in Table I, this contributes to the forensics and wireless fields by treating wireless measurements themselves as forensic evidence and by organizing the field around accountable reconstruction rather than only detection or defense. By 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:4 

<!-- Start of picture text -->
Intelligent Wireless Forensics in 5G/6G and Beyond: Evidence, Methods, Evaluation, and Practice<br>IntroductionSection I.  Forensics in Section II. Wireless  Section III. Forensics Wireless  ApplicationsSection IV. Forensic  Open ChallengesSection V.& Future<br>Systems Workflow Directions<br>A. Background and Motivation A. Information Forensics A. Forensic Readiness & Preservation-by-design A. Detection & Anomaly Discovery A. Generalization and Domain Shift<br> Provenance  Evidence Cross-layer Corroboration  Proactive Triggers  Adaptive Retention  Lifecycle  Spectrum Anomaly  KPI Monitoring  Rogue Detection  Cross-site Transfer  Calibration Drift  Distribution Shift<br>B. Related Work B. Physical-layer Forensics B. Acquisition B. Attribution & Identification B. Resource-aware Forensics<br> RF Fingerprinting  Waveform Source Association  Placement  Event-driven Capture  Confidence Sampling  Infrastructure Attribution  Signal-native ID  Flow Analysis  Adaptive Capture  Edge Constraints  Cost-aware<br>C. Contributions C. Network-layer Forensics C. Correlation & Analysis C. Provenance & Localization C. Generative Evidence<br> Flow Telemetry  Control-plane Logs  Session Reconstruction  Fusion  Disentanglement Causal Narrative  Confidence RegionsSource Localization  Trajectory    Enhancement  Admissibility Risk  Derived Artifacts<br>D. Cross-layer Fusion D. Authenticity & Anti-forgery<br>D. Reporting & Reproducibility<br> Identity Binding  Multi-modal Fusion  Attribution  Uncertainty Quantification   Signal Authentication  Replay Resistance  PHY Watermark<br> Pipeline  Re-execution<br>E. Lessons Learned E. Timeline Reconstruction &<br>E. Lessons Learned Event Correlation<br> Cross-domain Telemetry<br> Timeline  Causal Correlation<br>F. Lessons Learned<br><!-- End of picture text -->

Fig. 1. Survey organization and taxonomy overview. 

connecting traditional wireless measurements to the broader goal of forensic reconstruction, this survey aims to extend wireless communication research toward more accountable and trustworthy digital infrastructures, with its main contributions summarized as follows. 

- We introduce a unified wireless evidence taxonomy spanning radio, protocol, and architectural traces, with explicit discussion of evidentiary value and uncertainty sources. 

- We systematize a wireless forensics workflow that distinguishes readiness, acquisition, correlation and analysis, and reporting, and it clarifies where learning assisted methods can be safely inserted. 

- We summarize analysis methods from signal level identification to cross layer correlation, highlighting design principles needed for defensible conclusions, including provenance and reproducibility considerations. 

- We present evaluation practices, representative case studies, and open challenges, aiming to connect practical wireless measurements to auditable and reproducible forensic outcomes. 

The rest of this survey is organized as follows. Section 2 provides a unified taxonomy of forensics in wireless communication systems. Section 3 compares the traditional and artificial intelligence (AI)-based forensics. After that, Section 4 discusses the application of forensics via several cases, Section 5 gives the open challenges and future detections, and followed by Conclusion in Section 6. 

## **2 FORENSICS IN WIRELESS SYSTEMS: A UNIFIED TAXONOMY** 

This section adopts a unified taxonomy for wireless forensics, organized from information forensics to physical layer, device layer, network layer, and cross-layer forensics. For each layer, it summarizes the evidence types, supported claims, limitations and failure modes, adversarial manipulation, and corresponding mitigation and cross-checks. 

## **2.1 Information Forensics** 

Wireless investigations treat short-lived over-the-air artifacts as strictly verifiable evidentiary objects. Defensible wireless forensics demands rigorous provenance metadata to guarantee independent reproducibility. This explicit metadata encompasses signal acquisition configurations, temporal synchronization references, receiver calibration statuses, and observation geometries [25]. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:5 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

Table 1. Overview of representative related surveys. 

|**Scope**<br>**Ref.**<br>|**Overview**<br>|**Wireless**<br>**evidence**|**Uncertainty/**<br>**calibration**|**Forensic**<br>**reconstruction**<br>|
|---|---|---|---|---|
|<br>[10]|Reviews digital forensics workfows, tools, and open<br>challenges.||||
|Digital and<br>Network<br>[11]<br>|Surveys network forensic frameworks for collection,<br>correlation, and reporting.<br>|||<br>|
|Forensics<br>[12]|Taxonomizes network forensics and attribution under<br>partial visibility.||||
|[13]|Examines forensic data growth and the need for<br>triage.|||Partially|
|[14]<br>|Discusses forensic-as-a-service for scalable analysis.|||Partially<br>|
|[15]|Reviews cloud forensics challenges in access and<br>provider dependence.||||
|[16]|Organizes cloud forensic artifacts and challenges by<br>investigation stage.||||
|[6]|Surveys IoT forensics across devices, gateways, and<br>clouds.|Partially|||
|IoT and<br>[17]|Reviews IoT forensic taxonomy, requirements, and<br>scalabilityissues.|Partially|||
|Mobile<br>Forensics<br>[18]|Surveys IoT forensic requirements, tools, and automa-<br>tion.|Partially|||
|[19]|Highlights trace-centric IoT artifacts and their eviden-<br>tiaryvalue.|Partially|||
|[20]<br>|Reviews physical-layer identifcation for device attri-<br>bution.|<br>|Partially<br>|Partially|
|[21]|Surveys device fngerprinting and its robustness<br>across layers.|||Partially|
|[22]|Surveys RF fngerprinting methods, datasets, and chal-<br>lenges.||Partially||
|[23]|Reviews wireless intrusion detection threats and<br>methods.||||
|[24]|Surveys misbehavior detection in cooperative ITS.||||
|**This survey**<br>|Systematizes wireless evidence, workfows, methods,<br>and evaluation from an evidence-centricperspective.||||

Fulfilling these stringent requirements successfully transforms transient RF phenomena into admissible investigative conclusions [26]. Wireless evidence naturally distributes across over-the-air observables, endpoint artifacts, and infrastructure-side telemetry. Strong investigative conclusions inherently rely on rigorous cross-layer corroboration [25]. Physical-layer forensics extracts reexaminable signal observables to anchor incident manifestation claims and enable probabilistic source association. Device-layer forensics explicitly links these wireless-facing phenomena to endpoint operations, configuration evolution, and security-material usage. Network-layer forensics utilizes control-plane and data-plane telemetry to reconstruct session evolution and infrastructureside identifier dynamics. Cross-layer forensics systematically fuses these heterogeneous artifacts to strengthen transmitter attribution, validate temporal ordering, and strictly bound alternative explanations under documented uncertainty sources [26]. 

## **2.2 Physical-layer Forensics** 

Physical-layer forensics extracts over-the-air waveforms to reconstruct wireless incidents. Because receiver-induced distortions can overwhelm intrinsic RF fingerprints, raw signal measurements must be tied to clear capture provenance, including receiver settings and sampling parameters, to support defensible re-examination [27]. 

**_1) Evidence Types:_** Physical-layer wireless evidence primarily falls into three distinct categories. The first category comprises waveform and snapshot artifacts including baseband in-phase and quadrature samples. The second category involves channel and measurement artifacts mapping directly to geometric constraints. The third category encompasses hardware-impairment and transmitter-signature artifacts supporting same-origin assessments [28]. Concrete studies operationalize the extraction of these physical-layer artifacts. Highlighting waveform artifacts, the study in [29] constructs raw waveform recordings combined with explicit capture descriptors. This standardized metadata allows independent examiners to re-estimate synchronization parameters from the same structured fields. Addressing channel artifacts, the authors in [30] modify the firmware of 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:6 

an Intel 5300 network interface card. This modification exports per-packet subcarrier-level channel state information to user space. Investigating hardware impairments, the work in [31] extracts transmitter signatures from standard IEEE 802.11 waveforms. The processing estimates carrier frequency offsets and constellation distortion patterns to perform robust device classification. 

**_2) Supported Forensic Claims:_** Physical-layer evidence supports specialized forensic claims encompassing incident manifestation determining abnormal radio spectrum activities, source association providing probabilistic transmitter attribution under defined propagation assumptions, and spatiotemporal evolution deriving region-valued constraints for event-window reconstruction [28]. These verifications inherently demand calibrated scores strictly bounded by acquisition quality and environmental comparability. Concrete studies operationalize these physical-layer claims. Addressing incident manifestation, the framework in [32] analyzes reactive jamming utilizing external radio sensors collecting short-time received-energy traces. Separating mixed signal components via blind source separation quantifies directed influence using all-versus-one transfer-entropy statistics to capture jammer reaction patterns. This explicitly reports detection probabilities and false-alarm trade-offs under severe shadowing and collision conditions. Investigating source association, the research in [33] adopts generative adversarial networks (GANs) to adversarially suppress receiveridentifiable cues while strictly preserving transmitter separability. Applying open-set decision rules rejecting previously unseen emitters on unseen receivers yields physical-layer identity evidence explicitly conditioned on receiver calibration and operating thresholds. 

**_3) Limitations and Adversarial Threats:_** Physical-layer evidence stability suffers from nonadversarial limitations and active adversarial manipulations. Intrinsic propagation dynamics reshape waveform statistics to produce severe feature drift. Evidence sparsity increases estimation variance and destabilizes learned physical-layer fingerprints. Calibration drift gradually alters observables through temperature fluctuations and equipment aging [34]. Adversaries actively exploit these physical-layer vulnerabilities through three primary vectors. Active waveform manipulation suppresses discriminative structures while maintaining communication link viability. Learning-pipeline poisoning implants stealthy backdoors into physical-layer classifiers. Signature forgery utilizes synthesized impairment patterns for device impersonation [35]. These combined factors successfully degrade physical-layer attribution traces without causing obvious network-layer denial of service anomalies [36]. Concrete studies illustrate how these vulnerabilities restrict physical-layer wireless investigations. Investigating calibration drift, the study in [37] trains ResNet50 architectures on physical-layer transient regions at standard room temperature. Testing these models at extreme temperatures ranging from -40 to 80 degrees Celsius reveals severe accuracy degradation. Investigating learning-pipeline poisoning, the authors in [38] generate stealthy triggers tailored to physical-layer wireless temporal dynamics. This specific attack achieves a 99.2% success rate while limiting clean data classification degradation to less than 0.6%. These severe vulnerabilities force investigators to report calibrated confidence regions and explicitly verify training provenance. 

**_4) Mitigations and Cross-checks:_** Physical-layer mitigations strengthen independent verifiability and bound alternative explanations under adversarial variability. Mitigation strategies primarily include acquisition-chain hardening, uncertainty-aware reporting, and cross-layer state reconciliation [39]. Addressing acquisition-chain hardening, the system in [40] synchronizes distributed receivers using wireless two-way time transfer. A two-step procedure resolves matched-filter ambiguity while frequency locking actively limits hardware drift. This mechanism yields a 2.26 picoseconds timing precision. This extreme precision establishes quantitatively interpretable regionvalued constraints for physical-layer multi-vantage agreement tests. Operationalizing uncertaintyaware reporting, the authors in [41] introduce a conformal-prediction module wrapping pretrained physical-layer fingerprint classifiers. The system calibrates nonconformity scores on a dedicated 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:7 

<!-- Start of picture text -->
Limits / failure modes:<br> propagation/measurement-<br>chain distortion<br> sparse/short evidence windows<br> sync & delay uncertainty<br><!-- End of picture text -->

<!-- Start of picture text -->
Anti-forensics & adversarial<br>manipulation:<br> waveform manipulation<br> impersonation/signature forgery<br> poisoning/backdoors<br> reactive interference<br><!-- End of picture text -->

<!-- Start of picture text -->
Mitigations & cross-<br>checks:<br> Channel-agnostic<br>transformations suppress<br>multipath distribution shifts.<br> Robust adversarial training<br>hardens classifiers against<br>environmental fluctuations.<br> Explicit provenance<br>logging securely records<br>receiver calibration<br>parameters.<br> Multi-receiver correlation<br>mathematically bounds<br>spatial localization errors.<br><!-- End of picture text -->

<!-- Start of picture text -->
Evidence sources Evidence type Supported forensic<br> Raw baseband  claims<br>SDR probe /monitor  Channel & observables  Incident manifestationSource association &<br>WiFi/network interface controller interface controller controller   Hardware-measurement artifacts  identitySpatiotemporal<br>Multi-receiver telemetrytelemetry Replayable signal  impairment & transmitter-  constraintsMechanism-oriented<br>array evidence signature artifacts indicators<br><!-- End of picture text -->

<!-- Start of picture text -->
WiFi/network interface controller interface controller controller<br>Multi-receiver telemetrytelemetry<br>array<br><!-- End of picture text -->

Fig. 2. Physical-layer forensics framework from signal evidence to defensible claims. It illustrates provenancebound signal capture across multiple observation points, and summarizes the supported forensic claims, representative failure modes and adversarial manipulation, and key mitigation principles [39]. 
