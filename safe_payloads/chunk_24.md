
**_3) AI-based Acquisition: Intelligent Execution and Active Interaction_** AI-based acquisition extends wireless forensics beyond static, rule-based thresholds toward adaptive and context-aware evidence collection. Rather than relying on fixed-rate recording, learning-based execution dynamically allocates sensing effort according to evidentiary value, observation uncertainty, and signal ambiguity, thereby improving both acquisition efficiency and forensic fidelity. 

_Confidence-aware and Uncertainty-quantified Sampling:_ Unlike traditional multi-tier sampling policies, AI-driven acquisition adjusts sampling density according to uncertainty estimates. Prior work on uncertainty-aware wireless sensing and Bayesian data collection shows that physical variations can be mapped into confidence-aware acquisition policies [97, 98]. In forensic settings, this enables adaptive escalation from low-overhead routine monitoring to evidentiary-grade capture: systems such as uncertainty-based monitoring frameworks continuously assess the confidence of extracted physical-layer artifacts, triggering full-resolution raw in-phase and quadrature (I/Q) capture and denser cross-layer logging when hardware fingerprints become unreliable under fading or adversarial obfuscation, while permitting sparse sampling when confidence remains high [99, 100]. 

_Active Sensing for Evidence Disambiguation:_ AI-based acquisition can also move beyond passive observation toward active interaction with the wireless environment. This is particularly important when ambiguous phenomena, such as impulsive attacks and environmental interference, cannot be reliably separated from passive traces alone [101]. Emerging 5G/6G capabilities, including integrated sensing and communication (ISAC) and reconfigurable intelligent surface (RIS), allow forensic agents to probe the environment through controlled pilots, adaptive beamforming, and reflection-path reconfiguration. Recent studies show that such active sensing can expose otherwise hidden threats, including passive eavesdroppers, by eliciting scenario-specific responses from suspicious nodes [102104]. 

_AI-driven Evidence Reconstruction and Super-resolution:_ When hardware limits, packet loss, or severe channel impairments fragment the evidence trail, AI models can help reconstruct incomplete observations and enhance low-resolution measurements. Building on CSI reconstruction and super-resolution methods, recent work shows that generative and multitask learning models, e.g., Transformer, can recover missing channel fingerprints, upscale sensing data, and restore degraded electromagnetic leakage signals [105109]. Such outputs can improve downstream analysis when continuous high-resolution capture is infeasible, but they should remain clearly identified as derived rather than original evidence. 

_The PrivacyVisibility Paradox:_ At the operational level, improving evidentiary fidelity often conflicts with privacy regulation. Fine-grained protocol telemetry and raw physical-layer captures may inadvertently expose sensitive payloads, identifiers, or location information, creating a privacy visibility paradox for forensic-ready networks. Future acquisition architectures should therefore embed privacy-by-design mechanisms, such as edge-side signal separation, zero-knowledge telemetry proofs, and real-time payload sanitization [65, 110], so that actionable forensic observables can be preserved without retaining raw user data. 

## **3.3 Correlation & Analysis** 

Acquisition improves visibility, but the resulting traces remain fragmented across sensors, protocol layers, and timescales. Therefore, correlation and analysis can form the analytical core of wireless 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:16 

<!-- Start of picture text -->
Endpoint radio<br><!-- End of picture text -->

<!-- Start of picture text -->
Infrastructure<br><!-- End of picture text -->

<!-- Start of picture text -->
Sensing &<br>access<br>substrate<br>CSI extraction<br>tools<br>Firmware /<br>monitor-mode<br>access<br>Modem diagnostic<br>interface<br>Multi-sniffer<br>placement &<br>coordination<br><!-- End of picture text -->

<!-- Start of picture text -->
Confidence-aware sampling<br><!-- End of picture text -->

<!-- Start of picture text -->
controller Evidence reconstruction<br>& super-resolution<br>Active sensing for<br>disambiguation<br><!-- End of picture text -->

<!-- Start of picture text -->
Time-stamped<br>forensic primitives<br>Privacy-by-<br>design<br>Sanitization<br><!-- End of picture text -->

<!-- Start of picture text -->
User payload stripped<br><!-- End of picture text -->

Fig. 5. The acquisition framework for provenance-aware wireless evidence collection. It organizes a shared wireless observation substrate for both static rule-based acquisition and AI-driven closed-loop acquisition, and summarizes key operations including selective triggering, confidence-aware sampling, active sensing for disambiguation, and evidence reconstruction [111, 112]. 

forensics, which can transform heterogeneous observations into defensible causal claims by aligning evidence across sources, disentangling device-dependent effects from channel distortion, and connecting low-level measurements to high-level incident narratives. 

**_1) Evidence Alignment and Synchronization:_** Correlation begins by mapping heterogeneous captures into a common temporal, spectral, and logical reference frame. This is more difficult in wireless environments than in host-centric forensics because clocks drift across software-defined radios (SDRs), monitors, devices, and cloud-native functions, while identifiers change across mobility, protocol transitions, encryption boundaries, and address translations. Hence, defensible cross-source analysis requires not only timestamp normalization, but also preservation of synchronization metadata, including clock sources, drift compensation, frequency-offset correction, and transformation history. For example, the study in [113] illustrates the above principle by pairing raw RF recordings with machine-readable metadata about timing, sampling, hardware context, and annotations, while the work in [111] show that event reconstruction must rely on explicit time anchors rather than assumed clock correctness. 

At the physical layer, alignment is typically performed at sample, frame, or burst granularity. Independent receivers must be reconciled through timing-offset estimation, carrier-frequency correction, and landmark matching before they can be treated as observations of the same transmission. The authors in [31] demonstrate this in radiometric identification through burst synchronization and offset compensation, while the study in [112] shows that receptions from multiple access points can be associated through time-of-arrival processing and cross-receiver matching. At higher layers, control-plane messages, telemetry, and protocol records must likewise be normalized and anchored to preserve causal order. For example, the work in [52] exposes fine-grained cellular signaling on commodity smartphones, and the study in [114] further reconstructs 5G core activity as provenance graphs while explicitly tracking identity transformations across IP addresses and network functions. 

**_2) Rule-based Correlation and Signal Disentanglement:_** Once alignment is established, traditional correlation reconstructs incident structure through deterministic association, statistical testing, and model-based interpretation. For example, signal disentanglement can be achieved through correlation at the physical layer, the observed waveforms often combine transmitter impairments with fading, interference, mobility, and receiver artifacts. The study in [115] shows that RF fingerprinting degrades sharply in realistic channels, illustrating how device-specific 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:17 

signatures become entangled with propagation and noise. Therefore, classical signal processing remains central: synchronization refinement, matched filtering, blind source separation, sparse decomposition, spectral segmentation, and parameter estimation are used to isolate evidentiary components from environmental variability. For instance, the authors in [31] extract radiometric signatures through burst detection, alignment, and feature extraction, and the authors in [116] further emphasize the need to account for channel diversity in practical RF identification. Beyond detection, these methods also recover interpretable intermediate artifacts, such as delay estimates, occupancy masks, and impairment descriptors, that can be cross-checked with higher-layer traces. 

However, deterministic methods become fragile under encrypted services, identifier churn, mobility, missing logs, and large-scale heterogeneity. Public cellular datasets and measurement toolchains remain limited [117], while operational cellular traffic is often encrypted and integrity protected [118], reducing the completeness of cross-layer reconstruction. Signal models are also vulnerable to replay-style impersonation and channel mismatch [115, 119]. In this regard, traditional correlation can remain a defensible baseline, but not a sufficient solution for dynamic and partially observed wireless environments. 

**_3) AI-assisted Analysis and Causal Inference:_** AI-assisted analysis extends wireless forensics beyond explicit rule matching toward learned representation, multimodal fusion, and forensic hypothesis ranking. Rather than manually defining correspondences across sensors and protocol layers, learning-based models project heterogeneous evidence into shared latent spaces, enabling stronger association of temporally adjacent, behaviorally consistent, or causally related observations. For instance, beyond correlation, AI can also support causal reconstruction by helping explain mechanism, ordering, and competing alternatives rather than merely assigning anomaly scores. One example is that the authors in [120] use graph neural networks for root-cause analysis over multivariate network key performance indicators (KPI), allowing hidden dependencies among network elements to be inferred directly. Similarly, the authors in [121] show that learned models can rank multiple concurrent root causes in the specific environment settings, while the study in [122] uses temporal graph networks to capture spatiotemporal dependencies in wireless traces for anomaly and intrusion analysis. In this sense, AI is most valuable not as a final judge, but as an auditable tool for ranking plausible forensic explanations. 

Additionally, AI can mitigate partial observability by imputing missing traces, denoising corrupted captures, and fusing incomplete evidence windows. Taking an example, the authors in [123] use diffusion-based modeling to repair missing CSI and improve downstream inference. However, such derived outputs must remain explicitly separated from original evidence. Learned models may otherwise rely on dataset artifacts, receiver-specific shortcuts, or site-dependent bias rather than mechanism-relevant signals. Channel shift alone may degrade RF fingerprinting performance [115], and recent work further demonstrates adversarial, backdoor, and practical attack vulnerabilities in learned wireless signal analysis and RF identification systems [124127]. In this regard, AI should be incorporated only with provenance tracking, uncertainty reporting, and preservation of intermediate outputs for independent review. 

## **3.4 Reporting & Reproducibility** 

The value of wireless forensics depends not only on whether evidence is captured and analyzed, but also on whether conclusions can be communicated, challenged, and independently replayed. Because wireless claims often depend on synchronization quality, calibration state, observation coverage, preprocessing choices, and model assumptions, reporting must preserve the linkage from raw evidence to intermediate transformations and final conclusions, while clearly separating direct observation from probabilistic inference and analyst interpretation. Hence, reporting and 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:18 

<!-- Start of picture text -->
time<br>normalization<br><!-- End of picture text -->

<!-- Start of picture text -->
burst-frame<br><!-- End of picture text -->

<!-- Start of picture text -->
Rule-based correlation & signal<br>disentanglement<br>deterministic<br>association statistical testing<br>signal  interpretable<br>disentanglement artifacts<br>complementary<br><!-- End of picture text -->

<!-- Start of picture text -->
deterministic<br>association statistical testing<br>signal  interpretable<br>disentanglement artifacts<br>complementary<br>analytic paths<br>AI-assisted analysis & causal inference <br><!-- End of picture text -->

<!-- Start of picture text -->
Cross-layer<br>incident<br>narrative<br> aligned events<br> causal relations<br> candidate<br>explanation<br><!-- End of picture text -->

Fig. 6. The correlation-and-analysis workflow for turning fragmented wireless traces into defensible crosslayer incident narratives. Depict multi-source evidence aligned in a common temporal, spectral, and logical frame, and summarize how rule-based correlation and signal disentanglement, together with AI-assisted fusion, hypothesis ranking, and trace recovery, support cross-source association, causal reconstruction, and incident interpretation with provenance and uncertainty awareness [120]. 

reproducibility are the mechanisms through which wireless forensic results become reviewable, transferable, and, where necessary, admissible. 

**_1) Calibrated Reporting and Evidence Packaging:_** Traditional forensic reporting emphasizes explicit claim-to-evidence linkage and preservation of acquisition context; in wireless settings, this discipline must be strengthened to account for environmental uncertainty and cross-layer ambiguity. The works in [113, 128, 129] have illustrated this principle in digital and RF settings by pairing evidence with provenance, tool, and metadata records. Therefore, a defensible wireless report should state not only what was observed, but also how it was captured, under which assumptions it was interpreted, and which alternatives remain unresolved. Capture provenance, sensor placement, synchronization status, calibration state, preprocessing steps, encryption-related visibility limits, and privilege constraints should all be treated as first-class reporting elements. 

A practical evidence package should contain three layers: preserved raw evidence, condensed analytical views, and explicit uncertainty information. To be specific, raw evidence may include I/Q traces, CSI matrices, packet captures, baseband logs, and control-plane records, with signal metadata format (SigMF) serving as a portable substrate for machine-readable RF preservation [113]. In addition, as illustrated by works in [114, 130], analytical views such as timelines, provenance graphs, localization regions, and protocol-state summaries support triage without discarding causal structure. Finally, uncertainty information should further disclose confidence bounds, missing observations, coverage limits, and rejected hypotheses; this is consistent with both confidence calibration concerns in modern learning systems and time-anchor-based reasoning about timestamp validity [111, 131]. In summary, such layering allows reviewers to examine the same case at different abstraction levels without conflating model outputs or narrative summaries with equally strong forms of evidence. 

**_2) Reproducible Pipelines and Re-execution Artifacts:_** Reproducibility requires preserving not only the data, but also the computational pathway that transformed it into conclusions. In wireless forensics, this includes software and firmware versions, feature-extraction code, model checkpoints, parameter settings, random seeds, calibration files, schema mappings, synchronization procedures, and transformation logs. Even with identical captures, small differences in preprocessing, decoder options, or toolchains may materially change attribution outcomes. Prior work on 
