
- **Artifact reliability:** If the timeline contains conflicting information i.e., at least two artifacts provide conflicting information, a resolution is needed. Automation in identifying accurate artifacts would be advantageous. One possibility is to compare artifacts and assess their reliability, e.g., the ease of manipulating an artifact (Vanini et al., 2024a). Hargreaves & Patterson (2012) began work on handling conflicting artifacts, where each inferred high-level event was assigned a series of expected artifacts. On a match, the supporting _and contradictory_ timeline entries were stored within the inferred event, highlighting entries that were expected but absent, forming the basis for the evaluation of reliability assessment. Casey (2011) discusses the number of independent sources and their resistance to tampering as part of the C-Scale, but if this were to be more strictly quantified, e.g., with Bayesian networks for example (Kwan et al., 2008), in terms of assigning weight to expected artifacts, other factors may have an impact. For example artifact longevity, i.e., how long an artifact is known to persist may allow appropriate weight to be given to the absence of specific, expected, hypothesissupporting information. It remains unclear how appropriate precise numerical assessments in event reconstruction are. 

- **AI integration:** The use of AI for digital forensics is becoming more common (Du et al., 2020a; Jarrett & Choo, 2021). AI can help analyze and identify digital evidence (Henseler & van Beek, 2023; Sreya et al., 2023) or aid investigators in writing forensic reports (Michelet & Breitinger, 2024). As discussed by Scanlon et al. (2023), LLMs may help with event analysis, such as suspicious activities or attack identification. However, they may hallucinate when responding to investigator questions. Future work should focus on evaluating and validating this new technology for forensic purposes. Others have tried to apply AI techniques to accelerate the process, e.g., by searching for anomalies (Studiawan et al., 2017; Studiawan & Sohel, 2021) or relevant artifacts (Du et al., 2020b; Markov et al., 2022). 

- **Natural Language Processing (NLP) integration:** NLP may support timeline analysis as each event is represented by a descriptive message. These messages contain valuable infor- 

11 

mation that can be extracted and analyzed. By applying traditional NLP techniques, such as sentiment analysis (Silalahi et al., 2023c; Studiawan et al., 2020b), named entity recognition (Silalahi et al., 2023a,b; Studiawan et al., 2023), and information extraction, researchers can derive insights. For future research, there is potential to explore other NLP methods to enhance the field. For instance, topic modeling and dependency parsing could be employed to gain deeper insights into events and establish relationships between them. 

- **Process mining:** Event reconstruction is a common task in process mining (Weijters & van der Aalst, 2001; Jrgensen, 2021), though it is typically applied to business process logs (Nguyen & Comuzzi, 2019). However, the domain faces similar challenges. For example, Dixit et al. (2018) describe a set of timestamp-based indicators for identifying event ordering imperfections in logs and present a method for resolving these issues using domain knowledge. Therefore, future research could explore various process mining techniques (van der Aalst, 2016) for forensic event reconstruction. 

- **Training and education:** Specialized training and continuous education play a key role in ensuring investigators can handle complex cases and maintain the admissibility of evidence in court (Jahankhani & Hosseinian-far, 2014). However, cognitive biases and human errors can impact the integrity of findings, but some techniques can be used to mitigate this, e.g., collaborative approaches, such as the 4-eye principlewhere at least two individuals review the findings. More research is needed to explore how collaborative techniques and advanced decision-support systems, including AI-assisted tools, can further minimize human errors and biases, ensuring more reliable and transparent event reconstruction processes. 

**Research Gap 5.** The challenge of performing efficient and effective timeline analysis remains. Handling the volume of extracted timestamps in an effective way is needed (Q3/4), which could include technological solutions such as performance improvements or AI based filtering, but also process changes, where the extract everything model needs research to ensure it is still the most appropriate approach. 

**Research Gap 6.** Automation is likely the only practical way to handle the challenge of inferring events at scale (Q4), but how to handle the practical research challenge of automated inference of events from timeline entries that are subject to operating system, application, and environmental changes earlier on in the process (Q1,Q2) is challenging. 

**Research Gap 7.** Ensuring and communicating a clear delineation between extracted timestamp values as facts, and inferred events as working hypothesis, in both research and in forensic tooling (Q4), requires work from digital forensic scientists, and potentially UX experts to clearly communicate residual uncertainty. 

## **10. Conclusions** 

Event reconstruction is a critical part of the digital forensic process, yet the process and terminology are vague and inconsistent. This work has shown that this mixture of terms can be 

unified and as a result, a systematic organization of issues associated with timeline-based event reconstruction can be compiled. When an event reconstruction is completed, these potential issues can be considered and evaluated as to whether they may have influenced the result of the reconstruction. Aside from practical uses, it has also allowed clear future directions in event reconstruction research to be identified. 

While some of these identified challenges will be obvious to seasoned investigators, there is a need within digital forensics, to formalize definitions and make explicit that which is currently tacit. This provides the foundation for more formal and potentially future quantitative evaluation of the trustworthiness or indeed reliability of reconstructed events in a digital forensic investigation. 

## **Acknowledgments** 

We acknowledge Eoghan Casey for the comments and feedback. The authors also thank Cline Vanini for the initial diagram and discussions. 

## **Disclosure of AI-assisted writing tools** 

Some authors utilized ChatGPT-4 to assist in revising, condensing text, and correcting grammatical errors, typos, and awkward phrasing. All AI-generated suggestions were carefully reviewed and modified as necessary to ensure they aligned with the authors intended meaning before being incorporated into this paper. 

## **Declaration of interest** 

The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **References** 

- van der Aalst, W. (2016). Data science in action. In _Process Mining: Data Science in Action_ (pp. 323). Berlin, Heidelberg: Springer Berlin Heidelberg. URL: `https://doi.org/10.1007/978-3-662-49851-4_1` . doi: `10.1007/978-3-662-49851-4_1` . 

- Adderley, N., & Peterson, G. (2020). Interactive temporal digital forensic event analysis. In G. Peterson, & S. Shenoi (Eds.), _Advances in Digital Forensics XVI_ IFIP Advances in Information and Communication Technology (pp. 39 55). Cham: Springer International Publishing. doi: `10.1007/978-3-03056223-6_3` . 

- Adedayo, O. M., & Olivier, M. S. (2015). Ideal log setting for database forensics reconstruction. _Digital Investigation_ , _12_ , 2740. 

- Alqahtany, S., Clarke, N., Furnell, S., & Reich, C. (2016). A forensic acquisition and analysis system for IaaS. _Cluster Computing_ , _19_ , 439453. doi: `10.1007/s10586-015-0509-x` . 

- Amato, F., Cozzolino, G., Mazzeo, A., & Mazzocca, N. (2017). Correlation of Digital Evidences in Forensic Investigation through Semantic Technologies. In _2017 31st International Conference on Advanced Information Networking and Applications Workshops (WAINA)_ (pp. 668673). doi: `10.1109/WAINA.2017.4` . 

- Andrade, R. (2020). Expose evidence of timestomping with the ntfs timestamp mismatch artifact. URL: `https://www.magnetforensics.com/blog/e xpose-evidence-of-timestomping-with-the-ntfs-timestampmismatch-artifact-in-magnet-axiom-4-4/` . 

- Arshad, H., Jantan, A. B., & Abiodun, O. I. (2018). Digital forensics: review of issues in scientific validation of digital evidence. _Journal of Information Processing Systems_ , _14_ , 346376. 

12 

- Bang, J., Yoo, B., Kim, J., & Lee, S. (2009). Analysis of time information for digital investigation. In _2009 Fifth International Joint Conference on INC, IMS and IDC_ (pp. 18581864). IEEE. 

- Batten, L., Pan, L., & Khan, N. (2012). Hypothesis generation and testing in event profiling for digital forensic investigations. _Int. J. Digit. Crime Forensics_ , _4_ , 114. doi: `10.4018/jdcf.2012100101` . 

- Battistoni, R., Di Pietro, R., & Lombardi, F. (2016). Curetowards enforcing a reliable timeline for cloud forensics: Model, architecture, and experiments. _Computer Communications_ , _91_ , 2943. 

- Becker, D., Rabenseifner, R., & Wolf, F. (2008). Implications of non-constant clock drifts for the timestamps of concurrent events. In _2008 IEEE International Conference on Cluster Computing_ (pp. 5968). 

- Berggren, J., Gudjonsson, K., Jger, A. et al. (2024). Timesketch: Collaborative forensic timeline analysis. `https://github.com/google/timesketch` . 

- Bhandari, S., & Jusas, V. (2020). An ontology based on the timeline of Log2timeline and Psort using abstraction approach in digital forensics. _Symmetry_ , _12_ , 642. URL: `https://www.mdpi.com/2073-8994/12/4/642` . doi: `10.3390/sym12040642` . Number: 4 Publisher: Multidisciplinary Digital Publishing Institute. 

- Bhat, W. A., AlZahrani, A., & Wani, M. A. (2021). Can computer forensic tools be trusted in digital investigations? _Science_ & _Justice_ , _61_ , 198203. 

- Boyd, C., & Forster, P. (2004). Time and date issues in forensic computinga case study. _Digital Investigation_ , _1_ , 1823. 

- Breitinger, F., Hilgert, J.-N., Hargreaves, C., Sheppard, J., Overdorf, R., & Scanlon, M. (2024). Dfrws eu 10-year review and future directions in digital forensic research. _Forensic Science International: Digital Investigation_ , _48_ , 301685. 

- Buchholz, F., & Tjaden, B. (2007). A brief study of time. _Digital Investigation_ , _4_ , 3142. doi: `10.1016/j.diin.2007.06.004` . 

- Buchholz, F. P., & Falk, C. (2005). Design and implementation of zeitline: a forensic timeline editor. In _DFRWS_ . 

- Carrier, B., & Spafford, E. (2004a). Defining event reconstruction of a digital crime scene. _Journal of Forensic Sciences_ , _49_ , 12911298. doi: `10.1520/ JFS2004127` . 

- Carrier, B., & Spafford, E. (2004b). An event-based digital forensic investigation framework. In _Proceedings of the The Digital Forensic Research Conference_ (pp. 112). 

- Carrier, B. D. (2006). _A hypothesis-based approach to digital forensic investigations_ . Ph.D. thesis Purdue University. 

- Casey, E. (2011). _Digital evidence and computer crime: forensic science, computers and the Internet_ . (3rd ed.). Waltham, MA: Academic Press. 

- Casey, E. (2018). Digital Stratigraphy: Contextual Analysis of File System Traces in Forensic Science. _Journal of Forensic Sciences_ , _63_ , 13831391. doi: `10.1111/1556-4029.13722` . Number: 5. 

- Casey, E. (2020). Standardization of forming and expressing preliminary evaluative opinions on digital evidence. _Forensic Science International: Digital Investigation_ , _32_ , 200888. doi: `https://doi.org/10.1016/j.fsid i.2019.200888` . 

- Casey, E., Nguyen, L., Mates, J., & Lalliss, S. (2022). Crowdsourcing forensics: Creating a curated catalog of digital forensic artifacts. _Journal of Forensic Sciences_ , _67_ , 18461857. doi: `10.1111/1556-4029.15053` . _eprint: https://onlinelibrary.wiley.com/doi/pdf/10.1111/1556-4029.15053. 

- Chabot, Y., Bertaux, A., Nicolle, C., & Kechadi, M.-T. (2014). A complete formalized knowledge representation model for advanced digital forensics timeline analysis. _Digital Investigation_ , _11_ , S95S105. doi: `10.1016/j.di in.2014.05.009` . 

- Chabot, Y., Bertaux, A., Nicolle, C., & Kechadi, M.-T. (2015a). Event Reconstruction: A State of the Art. In M. M. Cruz-Cunha, I. M. Portela, & A. Piekarz (Eds.), _Handbook of Research on Digital Crime, Cyberspace Security, and Information Assurance:_ Advances in Digital Crime, Forensics, and Cyber Terrorism (p. 15). IGI Global. doi: `10.4018/978-1-46666324-4` . 

- Chabot, Y., Bertaux, A., Nicolle, C., & Kechadi, T. (2015b). An ontology-based approach for the reconstruction and analysis of digital incidents timelines. _Digital Investigation_ , _15_ , 83100. 

- Choi, H., Lee, S., & Jeong, D. (2021). Forensic recovery of SQL server database: Practical approach. _IEEE Access_ , _9_ , 1456414575. 

- Conlan, K., Baggili, I., & Breitinger, F. (2016). Anti-forensics: Furthering digital forensic science through a new extended, granular taxonomy. _Digital Investigation_ , _18_ , S66S75. doi: `10.1016/j.diin.2016.04.006` . 

- Debinski, M., Breitinger, F., & Mohan, P. (2019). Timeline2GUI: A Log2Timeline CSV parser and training scenarios. _Digital Investigation_ , _28_ , 3443. doi: `10.1016/j.diin.2018.12.004` . 

- Dixit, P. M., Suriadi, S., Andrews, R., Wynn, M. T., ter Hofstede, A. H., Buijs, 

J. C., & van der Aalst, W. M. (2018). Detection and interactive repair of event ordering imperfection in process logs. In _Advanced Information Systems Engineering: 30th International Conference, CAiSE 2018, Tallinn, Estonia, June 11-15, 2018, Proceedings 30_ (pp. 274290). Springer. 

- Dreier, L. M., Vanini, C., Hargreaves, C. J., Breitinger, F., & Freiling, F. (2024). Beyond timestamps: Integrating implicit timing information into digital forensic timelines. _Forensic Science International: Digital Investigation_ , _49_ , 301755. doi: `10.1016/j.fsidi.2024.301755` . 
