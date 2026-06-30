
ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:26 

## **4.5 Timeline Reconstruction & Event Correlation** 

Timeline reconstruction in wireless forensics turns heterogeneous traces into a coherent, timeordered narrative of what happened when, while event correlation links cross-layer symptoms to root actions. Because investigations combine discrete events (logs/telemetry) and continuous streams (RF/CSI), defensible reconstruction requires explicit timelines, reproducible parsing, and correlation rules that can be replayed. 

**_1) Cross-domain telemetry correlation for reconstructing timelines:_** Recent systems reconstruct incident timelines by correlating heterogeneous telemetry across layers, emphasizing reliable correlation keys and causal/provenance structure beyond raw timestamps. For instance, cross-layer telemetry (CLT) [174] reconstructs end-to-end timelines by joining application distributed tracing with in-band network telemetry: it binds application trace context to IPv6 hop-by-hop evidence and correlates them at collectors so each application span is explained by per-hop network measurements. Their evaluation shows the enhanced span exposes router queue growth, attributing the issue to network congestion rather than application logic, and reports round-trip time (RTT) distributions remain unchanged relative to the congested baseline because overhead is dominated by a lightweight netlink call. Building on correlated events, other works [175] assemble reconstructed incident paths using distributed tracing and provenance graphs [176]. For example, [177] reconstructs causal request paths from network-observable remote procedure call boundaries without code instrumentation: it adds lightweight in-network/host-side capture at application boundary interfaces and assembles traces via a storage/indexing pipeline for high-rate events. Reported per-event overhead is 277889 ns, a single-trace reconstruction query takes 1 s, and a 15-minute window search returns in 0.06 s, supporting rapid forensic pivoting. 

**_2) Wireless sensing driven timeline reconstruction:_** While cross-domain telemetry correlation restores missing linkages among logs for cyber incident timelines, wireless sensing links RF traces to physical-world actions by converting continuous measurements into timestamped event primitives (e.g., counts, identity cues) that can be fused with telemetry. For example, [178] treats WiFi sensing as event-stream construction: it extracts fidgeting and silent intervals from CSI and maps them into an occupancy estimate, producing a time-ordered event sequence alignable with access logs, camera timestamps, and alarm logs. Technically, it computes CSI phase differences across multiple Rx antennas, selects reliable subcarriers via SNR-based calibration, and denoises using principal component analysis (PCA). Crowd fidgeting is detected via spectral energy outside the breathing band, and seated-people counts are inferred by maximum a posteriori (MAP) estimation under a fidget-to-occupancy model. Reported performance includes 96.3% average counting accuracy, mean absolute error 0.44, and normalized mean square error 0.015. 

**_3) Case Study:_** For crowd-related incidents (e.g., stations), investigators need a defensible, timeordered account of how flows formed and split/merged. Wireless sensing can leverage existing infrastructure without relying on cameras [179]: routine downlink signals are timestamped, logged, and parsed into structured flow primitives, producing a replayable timeline artifact for correlation with operational logs and incident reports. This case uses synchronized capture of timestamped downlink CSI and generative-AI-assisted inference to extract flow events. Concretely, downlink CSI is captured at an infrastructure-side receiver and segmented into short sliding windows, where each window is one time slice on the reconstructed timeline. For each window, it derives a velocityacceleration spectrum to estimate target count, then applies a weighted conditional diffusion model to denoise/sharpen the spectrum for stable estimates. In parallel, antenna-array CSI estimates direction of arrival (DoA) and ToF, and diffusion enhances the DoA spectrum when spacing exceeds half a wavelength. Velocity, DoA, and ToF are fused and clustered to infer subflow count and subflow size, then stitched over time to form the flow timeline. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:27 

The experiments in a corridor and meeting room use two APs and one UE: one AP transmits, and the other captures CSI via Nexmon [180]. The receiver has four RF channels (one directional reference plus three-antenna ULA with one-wavelength spacing) at 5.805 GHz and 80 MHz. With UE activities (downloading, online gaming, video streaming), target-count accuracies in the corridor are 92%, 87%, and 79% (Fig. 10), while subflow-size accuracies are 91%, 87%, 73% (corridor) and 90%, 87%, 72% (meeting room). Overall, this case shows routine downlink CSI can be converted into replayable primitives (target count, subflow count, subflow size) for timeline reconstruction, but reliability depends on capture conditionspacket rate and environment complexity affect estimation and event orderingso reporting should preserve raw CSI, timestamps, and capture context controlling sampling density and spectrum resolution. 

<!-- Start of picture text -->
0.930.91 0.92 0.91 0.950.93 0.93 0.92 0.95 0.91 0.9<br>0.89 0.91 0.9 0.87 0.87<br>0.87 0.87 0.86 0.89 0.87 0.88 0.85<br>0.85 0.87<br>0.85 0.8<br>0.83 0.83<br>0.810.79 0.79 0.79 0.810.79 0.8 0.79 0.750.7 0.73 0.72<br>0.77 0.77<br>0.75 0.75 0.65<br>Downloading Online gaming Video streaming Downloading Online gaming Video streaming Downloading Online gaming Video streaming<br>Corridor Meeting room Corridor Meeting room Corridor Meeting room<br>(a) (b) (c)<br>Detection accuracy Detection accuracy Detection accuracy<br><!-- End of picture text -->

Fig. 10. The results of the case for flow detection in wireless forensics [179]. (a) The performance of detecting the number of targets. (b) The performance of detecting the number of subflows. (c) The performance of detecting the subflow size. 

## **4.6 Lessons Learned** 

Across Sections 4.14.5, the key lesson is that defensible cyber-(physical) detection depends not on isolated alerts, but on replayable, attributable, and uncertainty-aware evidence that remains credible across heterogeneous devices, networks, and sensing modalities. In _Detection & Anomaly Discovery_ , learning-based methods over radio and spectrum measurements support continuous anomaly discovery and help identify where evidence should be preserved under label scarcity [146, 181]. However, they remain limited by threshold sensitivity, drift, uncertainty, and periodicity shifts, and often stop at anomaly scores rather than preservation decisions. In _Attribution & Identification_ , infrastructure-visible traces and flow-level metadata can map incidents to accountable services, slices, or applications [149], but remain sensitive to traffic mixing, site dependence, and weak confidence calibration. In _Provenance & Localization_ , auditable intermediates such as heatmaps and ellipse constraints, help turn RSS and CSI measurements into defensible spatial claims [154, 159], although calibration dependence and incomplete provenance reporting still limit cross-site reproducibility. For _Authenticity and Anti-forgery_ , signal-interface authentication, including physical-layer tags and RIS challenge mechanisms, can strengthen evidentiary credibility by binding observations to claimed transmitters or contexts [162, 173]. However, operational overhead and incomplete preservation of decoding context hinder third-party replay. In _Timeline Reconstruction & Event Correlation_ , telemetry fusion, provenance graphs, and CSI/RF sensing improve causal reconstruction and align RF activity with system logs [104], but remain fragile under clock mismatch, caching, concurrency, and capture-dependent conditions. Overall, evaluations report false alarms, overhead, scalability, and sometimes uncertainty or domain shift [100, 157], but unified evidence-package standards and key reporting details, such as calibration and synchronization, remain lacking. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:28 

## **5 OPEN CHALLENGES & FUTURE DIRECTIONS** 

## **5.1 Generalization and Domain Shift** 

Generalization remains a core challenge because cross-site transfer, mobility, and calibration drift can alter the mapping from wireless measurements (RSS/CSI/KPI/trace) to forensic conclusions, causing out-of-distribution error growth and weakening evidentiary strength unless calibration state and robustness diagnostics are preserved as provenance metadata [182]. Future work should therefore pursue audit-friendly adaptation that separates portable inference from site-specific calibration. 

## **5.2 Resource-aware Forensics** 

Resource-aware forensics must balance evidentiary fidelity against edge constraints, because continuous RF/CSI/KPI capture consumes storage, bandwidth, compute, and latency, and may affect operational performance; the key challenge is to design cost-aware pipelines that decide what to capture, at what resolution, and for how long, while preserving replayable reconstruction and responsive correlation [159]. Promising directions include anomaly-driven adaptive capture, multi-tier processing that retains lightweight timestamped primitives broadly. 

## **5.3 Benefits and Risks of Generative Evidence** 

Generative models can improve forensic usability by denoising and sharpening intermediate evidence such as motion spectra and DoA spectra, thereby turning continuous RF/CSI into stable, timestamped event primitives for timeline reconstruction and correlation [110]. However, they also introduce admissibility risks because enhanced artifacts may be challenged as synthetic, may suppress rare but real features, or may be exploited to fabricate narratives. A defensible direction is therefore to treat raw traces as primary evidence and generative outputs as derived artifacts. 

## **6 CONCLUSION** 

This survey reviewed intelligent wireless forensics in 5G/6G and beyond. Unlike prior surveys that mainly emphasize detection, attribution, or defense, this work systematized wireless evidence sources across physical, device, network, and cross-layer views, and unified the forensic pipeline from readiness and acquisition to correlation, analysis, and reporting. We also showed how AI can support evidence capture, cross-layer interpretation, and reproducible reconstruction, while highlighting that defensible conclusions still depend on provenance, calibration, synchronization, and replayability. Looking ahead, key challenges include domain shift, cost-aware evidence preservation, missing evidence-package standards, and the admissibility risks of generative evidence enhancement. By bridging wireless evidence, signal processing, networking, and AI, this survey outlines a practical path toward reproducible, auditable, and trustworthy forensic reconstruction in future wireless systems. 

## **REFERENCES** 

> [1] Alex Olushola Akinbi. 2023. Digital forensics challenges and readiness for 6G Internet of Things (IoT) networks. _Wiley Interdisciplinary Reviews: Forensic Science_ 5, 6 (2023), e1496. 

> [2] Alex Nelson, Sanjay Rekhi, Murugiah Souppaya, and Karen Scarfone. 2025. Incident Response Recommendations and Considerations for Cybersecurity Risk Management. (2025). 

> [3] Ziyue Wang et al. 2022. CNN-and GAN-based classification of malicious code families: A code visualization approach. _Int. J. Intell. Syst._ 37, 12 (2022), 1247212489. 

> [4] Rabia Khan et al. 2019. A survey on security and privacy of 5G technologies: Potential solutions, recent advancements, and future directions. _IEEE Commun. Surv. Tutor._ 22, 1 (2019), 196248. 

> [5] Syed Rizvi, Mark Scanlon, Jimmy McGibney, and John Sheppard. 2022. Application of artificial intelligence to network forensics: Survey, challenges and future directions. _Ieee Access_ 10 (2022), 110362110384. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:29 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

- [6] Maria Stoyanova et al. 2020. A survey on the internet of things (IoT) forensics: challenges, approaches, and open issues. _IEEE Commun. Surv. Tutor._ 22, 2 (2020), 11911221. 

- [7] Yaoqi Yang et al. 2026. Data Freshness Performance Analysis and Optimization in Timely and Secure Low Altitude Economics. _IEEE Trans. Cognit. Commun. Networking_ 12 (2026), 60166030. 

- [8] Alexander Nelson et al. 2024. _Incident response recommendations and considerations for cybersecurity risk management: a CSF 2.0 community profile_ . Technical Report. National Institute of Standards and Technology. 

- [9] Dalal Alrajeh, Liliana Pasquale, and Bashar Nuseibeh. 2017. On evidence preservation requirements for forensic-ready systems. In _Proceedings of the 2017 11th joint meeting on foundations of software engineering_ . 559569. 

- [10] Fran Casino et al. 2022. Research trends, challenges, and emerging topics in digital forensics: A review of reviews. _Ieee Access_ 10 (2022), 2546425493. 

- [11] Emmanuel S Pilli, Ramesh C Joshi, and Rajdeep Niyogi. 2010. Network forensic frameworks: Survey and research challenges. _digital investigation_ 7, 1-2 (2010), 1427. 

- [12] Suleman Khan et al. 2016. Network forensics: Review, taxonomy, and open challenges. _Journal of Network and Computer Applications_ 66 (2016), 214235. 

- [13] Darren Quick and Kim-Kwang Raymond Choo. 2014. Impacts of increasing volume of digital forensic data: A survey and future research challenges. _Digital Investigation_ 11, 4 (2014), 273294. 

- [14] Ruud B van Baar, Harm MA van Beek, and EJ Van Eijk. 2014. Digital forensics as a service: A game changer. _Digital Investigation_ 11 (2014), S54S62. 

- [15] Ameer Pichan, Mihai Lazarescu, and Sie Teng Soh. 2015. Cloud forensics: technical challenges, solutions and comparative analysis. _Digital investigation_ 13 (2015), 3857. 

- [16] Bharat Manral et al. 2019. A systematic survey on cloud forensics challenges, solutions, and future directions. _ACM Comput. Surv._ 52, 6 (2019), 138. 

- [17] Ibrar Yaqoob et al. 2019. Internet of things forensics: Recent advances, taxonomy, requirements, and open challenges. _Future Generation Computer Systems_ 92 (2019), 265275. 

- [18] Hany F Atlam et al. 2020. Internet of things forensics: A review. _Internet of Things_ 11 (2020), 100220. 

- [19] Francesco Servida and Eoghan Casey. 2019. IoT forensic challenges and opportunities for digital traces. _Digital Investigation_ 28 (2019), S22S29. 
