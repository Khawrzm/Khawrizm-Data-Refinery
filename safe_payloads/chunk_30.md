
- [161] Zhiping Jiang, Jizhong Zhao, Xiang-Yang Li, Jinsong Han, and Wei Xi. 2013. Rejecting the attack: Source authentication for wi-fi management frames using csi information. In _2013 Proceedings IEEE INFOCOM_ . IEEE, 25442552. 

- [162] Pinchang Zhang et al. 2023. Tag-based PHY-layer authentication for RIS-assisted communication systems. _IEEE Trans. Dependable Secure Comput._ 20, 6 (2023), 47784792. 

- [163] Tiep M Hoang et al. 2024. Physical layer authentication and security design in the machine learning era. _IEEE Commun. Surv. Tutorials_ 26, 3 (2024), 18301860. 

- [164] Ning Xie et al. 2022. Physical layer authentication with high compatibility using an encoding approach. _IEEE Trans. Commun._ 70, 12 (2022), 82708285. 

- [165] Peter D Grnwald. 2024. Beyond NeymanPearson: E-values enable hypothesis testing with a data-driven alpha. _Proceedings of the National Academy of Sciences_ 121, 39 (2024), e2302098121. 

- [166] Yongli An, Shikang Zhang, and Zhanlin Ji. 2021. A tag-based PHY-layer authentication scheme without key distribution. _IEEE Access_ 9 (2021), 8594785955. 

- [167] Stefano Tomasin et al. 2022. Challenge-response physical layer authentication over partially controllable channels. _IEEE Communications Magazine_ 60, 12 (2022), 138144. 

- [168] Jiacheng Wang et al. 2025. Generative AI based secure wireless sensing for ISAC networks. _IEEE T INF FOREN SEC._ (2025). 

- [169] Yong Zhang et al. 2022. CSI-based location-independent human activity recognition using feature fusion. _IEEE Transactions on Instrumentation and Measurement_ 71 (2022), 112. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:35 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

- [170] Zhenghua Chen et al. 2018. WiFi CSI based passive human activity recognition using attention based BLSTM. _IEEE Trans. Mobile Comput._ 18, 11 (2018), 27142724. 

- [171] Jinyang Huang et al. 2020. Towards anti-interference WiFi-based activity recognition system using interferenceindependent phase component. In _IEEE INFOCOM 2020-IEEE Conference on Computer Communications_ . IEEE, 576585. 

- [172] Xiao Lu, Yuli Li, Wei Cui, and Haixia Wang. 2022. Cehar: Csi-based channel-exchanging human activity recognition. _IEEE Internet Things J._ 10, 7 (2022), 59535961. 

- [173] Ruiqi Kong and He Chen. 2024. CSI-RFF: Leveraging micro-signals on CSI for RF fingerprinting of commodity WiFi. _IEEE T INF FOREN SEC._ 19 (2024), 53015315. 

- [174] Justin Iurman, Frank Brockners, and Benoit Donnet. 2021. Towards cross-layer telemetry. In _Proceedings of the 2021 Applied Networking Research Workshop_ . 1521. 

- [175] Sachin Ashok et al. 2024. Traceweaver: Distributed request tracing for microservices without application modification. In _Proceedings of the ACM SIGCOMM 2024 Conference_ . 828842. 

- [176] Mert Toslali et al. 2021. Automating instrumentation choices for performance problems in distributed applications with VAIF. In _Proceedings of the ACM Symposium on Cloud Computing_ . 6175. 

- [177] Junxian Shen et al. 2023. Network-centric distributed tracing with deepflow: Troubleshooting your microservices in zero code. In _Proceedings of the ACM SIGCOMM 2023 Conference_ . 420437. 

- [178] Belal Korany and Yasamin Mostofi. 2021. Counting a stationary crowd using off-the-shelf wifi. In _Proceedings of the 19th annual international conference on mobile systems, applications, and services_ . 202214. 

- [179] Jiacheng Wang et al. 2024. Generative artificial intelligence assisted wireless sensing: Human flow detection in practical communication environments. _IEEE J. Sel. Areas Commun._ 42, 10 (2024), 27372753. 

- [180] Jrg Schfer et al. 2021. Human activity recognition using CSI information with nexmon. _Applied Sciences_ 11, 19 (2021), 8860. 

- [181] Mohammad Ariful Islam et al. 2022. A deep neural network-based communication failure prediction scheme in 5g ran. _IEEE Trans. Netw. Serv. Manag._ 20, 2 (2022), 11401152. 

- [182] Frost Mitchell et al. 2023. Learning-based techniques for transmitter localization: A case study on model robustness. In _2023 20th Annual IEEE International Conference on Sensing, Communication, and Networking (SECON)_ . IEEE, 133141. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

# SOURCE: nda (1).pdf

# **NDA: Preventing Speculative Execution Attacks at Their Source** 

Ofir Weisse University of Michigan 

## Ian Neal 

University of Michigan 

Kevin Loughlin University of Michigan 

## Thomas F. Wenisch University of Michigan 

### **ABSTRACT** 

Speculative execution attacks like Meltdown and Spectre work by accessing secret data in wrong-path execution. Secrets are then transmitted and recovered by the attacker via a covert channel. Existing mitigations either require code modifications, address only specific exploit techniques, or block only the cache covert channel. Rather than battling exploit techniques and covert channels one by one, we seek to close off speculative execution attacks at their source. Our key observation is that these attacks require a chain of dependent wrong-path instructions to access and transmit secret data. We propose _NDA_ , a technique to restrict speculative data propagation. _NDA_ breaks the attacks wrong-path dependence chains while still allowing speculation and dynamic scheduling. We describe a design space of _NDA_ variants that differ in the constraints they place on dynamic scheduling and the classes of speculative execution attacks they prevent. _NDA_ preserves much of the performance advantage of out-of-order execution: on SPEC CPU 2017, _NDA_ variants close 68-96% of the performance gap between in-order and unconstrained (insecure) out-of-order execution. 

### **CCS CONCEPTS** 

 **Security and privacy Hardware security implementation** ;  **Computer systems organization Architectures** . 

### **KEYWORDS** 

speculative execution, meltdown, spectre, security 

##### **ACM Reference Format:** 

Ofir Weisse, Ian Neal, Kevin Loughlin, Thomas F. Wenisch, and Baris Kasikci. 2019. NDA: Preventing Speculative Execution Attacks at Their Source. In _The 52nd Annual IEEE/ACM International Symposium on Microarchitecture (MICRO-52), October 1216, 2019, Columbus, OH, USA._ ACM, New York, NY, USA, 15 pages. https://doi.org/10.1145/3352460. 3358306 

### **1 INTRODUCTION** 

Speculative execution attacks [8, 13, 25, 27, 3336, 38, 39, 45, 54, 55, 59, 62, 64, 65] exploit micro-architectural behavior and side channels to exfiltrate sensitive information from a system. Unlike classical software exploits that modify and observe only architectural 

Permission to make digital or hard copies of all or part of this work for personal or classroom use is granted without fee provided that copies are not made or distributed for profit or commercial advantage and that copies bear this notice and the full citation on the first page. Copyrights for components of this work owned by others than ACM must be honored. Abstracting with credit is permitted. To copy otherwise, or republish, to post on servers or to redistribute to lists, requires prior specific permission and/or a fee. Request permissions from permissions@acm.org. _MICRO-52, October 1216, 2019, Columbus, OH, USA_  2019 Association for Computing Machinery. ACM ISBN 978-1-4503-6938-1/19/10...$15.00 https://doi.org/10.1145/3352460.3358306 

## Baris Kasikci University of Michigan 

<!-- Start of picture text -->
a) Control-Steering Attack b) Chosen-Code Attack<br>1 1<br>2 Steer control 2 Illegal access<br>Mispredicted - Access secret Speculative load Transmit secret<br>branch - Transmit secret (access secret)<br>4<br>4<br>3  Squash 3 Squash<br>Wrong-path Fault handler Wrong-path<br><!-- End of picture text -->

**Figure 1: Control-steering vs. chosen-code attacks. In controlsteering, the attacker steers control flow in existing victim code, inducing unwanted access to the victims memory space. In chosen-code, the attacker generates code that accesses privileged data or data that belongs to another context.** 

state (such as registers and memory), speculative execution attacks have demonstrated that attackers can retrieve secrets by controlling and observing micro-architectural state (e.g., the cache) during speculative wrong-path execution. 

Speculative execution attacks can be classified into two main categories. One class (e.g., Spectre [34], Spectre 1.1 [33], and others [8, 13, 35, 38, 55]) allows malicious code to mis-steer a victim programs control flow (e.g., by carefully mis-training branch predictors) to execute specific instructions on the speculative wrong path. Although wrong-path instructions are ultimately squashed (with no effect on architectural state), the victim program is coerced into leaking its own memory contents through a micro-architectural channel. For instance, Chen et al. [13] show how control-flow in an SGX secure enclave [42] can be steered to leak its own protected memory. We classify these attacks as _control-steering_ attacks (Fig. 1a). 

Another class of attacks [25, 36, 45, 54, 59, 62, 64, 65] enables unprivileged attacker code to access privileged memory that is temporarily exposed during wrong-path execution. For instance, Meltdown [36] allows reading kernel memory; Foreshadow [25, 62, 65] allows reading hypervisor, OS, SMM, or SGX memory; and LazyFP [59] allows reading AES keys from AVX registers used by another process. MDS attacks [45, 54, 64] allow reading recently accessed memory belonging to other processes. Since the attacker generates the code, they can select arbitrary instruction sequences in both correct-path and wrong-path execution. We classify these attacks as _chosen-code_ attacks (Fig. 1b). These two classes of attacks are fundamentally different and therefore require different approaches for mitigation. 

Existing software defenses against speculative execution attacks work by modifying a programs source code to block attack-specific mechanisms. Current software defenses for control-steering attacks such as Retpoline [21, 28], IBPB [29], and improved lfence [15] 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

instructionsfocus on preventing the attacker from steering the execution of victim code. Unfortunately, these defenses are not immediately applicable to existing binaries. Specifically, software mitigations against chosen-code attacks involve modifying the OS, hypervisor, and SMM code [23, 25, 37, 43]. A recent study by Google [41] discusses why software approaches aimed at mitigating timing channels by manipulating timers are insufficient. The authors show that any optimizations performed by micro-architecture, no matter how negligible, can become observable using an amplification technique. Even if code modifications are made, these defenses can be bypassed. For instance, attackers can redirect control flow to evade fence instructions (e.g., by mis-training the branch target buffer (BTB) [27, 34] or the return stack buffer (RSB) [33, 35, 38]). 

Hardware defenses, on the other hand, have the potential to obviate the need to modify existing software [29, 31, 32, 48, 51, 53, 60, 69]. The first disclosed speculative execution attacks [27, 34, 36] use caches as a covert channel to leak data from wrong-path execution. Consequently, initial hardware defensessuch as InvisiSpec [69], SafeSpec [31], and others [32, 48, 51, 53, 60]seek to prevent wrong-path execution from leaving secrets in the cache that can later be recovered. Taram et al. [60] suggests a hardware modification to automatically insert lfence micro-ops where needed. However, the authors claim mainly to address Spectre v1 attacks that use the data cache as a covert channel. 

While these techniques are effective, a recent study [12] noted that closing only the cache covert channel is insufficient to stop speculative execution attacks, since the cache is only one of many potential covert channels. Netspectre [55] and SMoTher Spectre [8] have already shown that secrets can be transmitted via the FPU or via port contention [4]. In 3, we further show how to transmit secrets via the BTB. 

Rather than isolating predictive structures [29] or sealing individual covert channels [31, 53, 69]a ceaseless arms racewe instead seek to close off speculative execution attacks at their source. Our philosophy is to treat potentially wrong-path values as secret and prevent these secret values from propagating through the microarchitecture. Our key observation is that speculative execution attacks require a _chain of dependent wrong-path instructions_ to access and transmit data into a covert channel. By preventing potentially wrong-path values from propagating, we break these dependency chains, thwarting the code sequences required to mount attacks. 

We propose _NDA_ Non-speculative Data Accessa technique to restrict speculative data propagation in out-of-order (OoO) processors. _NDA_ only allows instruction outputs to flow to dependents if the source instruction is considered _safe_ . _NDA_ restricts data propagation by preventing tag broadcast for unsafe instructions, delaying wakeup of their dependants in the issue queue until the source instruction becomes safe. 

We present a taxonomy of the building blocks of speculative execution attacks, show how each class of attack depends upon data propagation in wrong-path execution, and demonstrate how we can define _safe_ vs. _unsafe_ to prevent the data flow required by the attack. By composing various restrictions on when an instruction becomes safe, we create a design space of _NDA_ variants. The variants differ in (1) the constraints they place on the dynamic execution schedule (and therefore, performance), (2) the locations from which secret data might be extracted (e.g., whether general purpose registers are 

protected), and (3) the kind of speculation attacks they prevent (e.g., control-steering vs. chosen-code). 
