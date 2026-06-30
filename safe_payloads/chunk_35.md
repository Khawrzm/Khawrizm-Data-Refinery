
**Comparison to InvisiSpec [69].** Since _NDA_ and InvisiSpec have different threat models, detailed in Table 2, a direct comparison is not straight forward. In our evaluation, InvisiSpec-Spectre defeats all cache-based control-steering attacks with 7.6% slowdown. In comparison, _NDA_ blocks control-steering attacks, regardless of the covert channel they use, with 10.7%-36.1% slowdown, depending on where secrets reside. For futuristic chosen-code attacks, InvisiSpec-Future introduces 32.7% overhead compared to 125% in _NDA_ . However, _NDA_ blocks all covert channels, including port contention [8], the FPU [55], and the BTB (3). 

### **7 RELATED WORK** 

The first micro-architectural side-channel attacks used the cache side channel to infer AES keys from a neighboring process or 

VM [7, 10, 47]. Since then, a myriad of side channel techniques have been developed, such as Flush+Reload [73] and other advanced techniques [5, 18, 19, 24, 30, 49, 72, 76]. We refer to these attacks as _classical_ cache attacks. These attacks do not leverage speculative wrong-path execution. Other work demonstrates how the cache side channel can be used as a _covert channel_ [40, 66, 68]. DRAM [50] and issue ports [4, 8] are also demonstrated as viable covert channels. 

The first speculative execution attacksMeltdown [36] and Spectre [34]leveraged prior work on cache covert channels to transmit data obtained from wrong-path execution via the data-cache ( _dcache_ ). Other speculative attacks using various techniques to access secrets or steer execution also leveraged the d-cache covert channel [13, 16, 25, 27, 33, 35, 38, 59, 62, 65]. Since the d-cache covert channel is widely exploited, initial defenses [31, 48, 53, 69] have exclusively focused on protecting the d-cache. However, these defenses do not mitigate non d-cache speculative execution attacks [8, 12, 39, 55, 63]. Specifically, Mambretti et al. [39] demonstrated covert transmission of secrets via the instruction-cache ( _i-cache_ ). 

Unfortunately, it is not trivial to apply the same d-cache defensetechniques to provide i-cache protection. For example, Sakalis et al. [53] delay speculative loads on an L1 cache-miss to prevent speculative d-cache modifications. However, the authors mention it is difficult to apply the same policy to i-cache misses with low overhead: While d-cache delays do not preclude other in-flight instructions from executing OoO, i-cache delays stall the front-end and starve the entire pipeline. 

InvisiSpec [69] allows speculative loads to execute using a dedicated buffer, only committing updates to the d-cache once speculation resolves. While the authors hypothesize that a similar method could be applied to the i-cache, they do not implement or evaluate the performance overhead of such i-cache protection. In comparison to cache-only defenses, _NDA_ is agnostic to the covert channel used in the _Transmit Phase_ and blocks all known attacks. 

Conditional-Speculation [48] protects secrets placed in memory, but not in GPRs. In comparison, _NDA_ s strict-propagation prevents the attacker from performing the pre-processing required for the 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

1 <u><mark>stop_speculative_exec();</mark></u> 2 **<mark>register long</mark>** <mark>secret = *secret_addr;</mark> 3 _<mark>// ... operate on secret</mark>_ 4 <mark>secret = 0;</mark> _<mark>// scrub secret</mark>_ 5 <mark>resume_speculative_exec();</mark> 

#### **Listing 4: Closing the registers-to-memory security gap.** 

_Transmit Phase_ . _NDA_ thus defeats NetSpectre and SMotherSpectre attacks, while providing better protection for secrets in registers. 

Prior work [17, 60] suggest mitigations to defeat the Spectre v1 variant. Taram et al. [60] suggest Context Sensitive Fencing, a hardware modification to automatically insert lfence micro-ops where needed, to block the d-cache channel. SpectreGuard [17] suggested delaying broadcast of completed micro-ops to defeat Spectre v1 across multiple covert channels. However, as stated by the authors, their main goal is to block Spectre v1 attacks. NDA defeats all known variants regardless of the covert channel they use. 

Recent work (such as DAWG [32], CEASER [51], and others [70, 71]) hinder the attackers ability to deterministically cause a cache line collision with another process or VM, thwarting most cachebased side and covert channels. However, these techniques do not mitigate attacks that use non-cache covert channels. 

We addressed related work on deployed defense mechanisms for speculative execution attacks in 3.2. 

### **8 DISCUSSION** 

_NDA_ is capable of defeating both control-steering and chosen-code attacks while performing considerably better than in-order processors. However, even though _NDA_ blocks all known attacks, it may still be possible to use a control-steering attack to read generalpurpose registers if there exists a feasible single micro-op that can leak the registers contents. 

To protect registers, one can introduce an instruction or a processor mode that temporarily disables speculation and out-of-order execution during the window of vulnerability when a secret value is loaded from memory and resides in a register until it is overwritten. We illustrate such a defense in Listing 4. We note this defense would only be effective if used in addition to _NDA_ . Without _NDA_ , a controlsteering attack could simply steer the execution to bypass Line 1 and speculatively execute Lines 2-3 to leak the registers contents. 

### **9 CONCLUSION** 

Speculative execution attacks are challenging to mitigate. Blocking individual covert channels or specific exploitation techniques is insufficient. To design effective mitigations, we introduced a new classification of speculative execution attacks based on how each attack induces wrong-path execution. Our new technique for controlling speculative data propagation, _NDA_ , defeats all known speculative execution attacks and drastically reduces the attack surface for future variants. On SPEC 2017, we show that the four _NDA_ design points offer effective and performant mitigations. 

### **REFERENCES** 

[1] 2019. _InvisiSpec-1.0 source code_ . https://github.com/mjyan0720/InvisiSpec-1.0. 

- [2] 2019. _Lapidary: Crafting more beautiful gem5 simulations_ . https://medium.com/ @iangneal/lapidary-crafting-more-beautiful-gem5-simulations-4bc6f6aad717. 

- [3] 2019. _Lapidary: creating beautiful gem5 simulations_ . https://github.com/efeslab/ lapidary. 

- [4] Alejandro Cabrera Aldaya, Billy Bob Brumley, Sohaib ul Hassan, Cesar Pereida Garcia, and Nicola Tuveri. 2018. Port Contention for Fun and Profit. Cryptology 

ePrint Archive, Report 2018/1060. https://eprint.iacr.org/2018/1060. 

- [5] Thomas Allan, Billy Bob Brumley, Katrina E. Falkner, Joop van de Pol, and Yuval Yarom. 2016. Amplifying side channels through performance degradation. In _ACSAC_ . ACM, 422435. 

- [6] AMD. 2018. Speculative Store Bypass Disable. https://developer.amd.com/wpcontent/resources/124441_AMD64_SpeculativeStoreBypassDisable_ Whitepaper_final.pdf. 

- [7] Daniel J Bernstein. 2005. Cache-timing attacks on AES. (2005). http://palms.ee. princeton.edu/system/files/Cache-timing+attacks+on+AES.pdf. 

- [8] Atri Bhattacharyya, Alexandra Sandulescu, Matthias Neugschwandtner, Alessandro Sorniotti, Babak Falsafi, Mathias Payer, and Anil Kurmus. 2019. SMoTherSpectre: exploiting speculative execution through port contention. _arXiv preprint arXiv:1903.01843_ (2019). 

- [9] Nathan Binkert, Bradford Beckmann, Gabriel Black, Steven K Reinhardt, Ali Saidi, Arkaprava Basu, Joel Hestness, Derek R Hower, Tushar Krishna, Somayeh Sardashti, et al. 2011. The gem5 simulator. _ACM SIGARCH Computer Architecture News_ 39, 2 (2011), 17. 

- [10] Joseph Bonneau and Ilya Mironov. 2006. Cache-collision timing attacks against AES. In _International Workshop on Cryptographic Hardware and Embedded Systems_ . Springer, 201215. 

- [11] Erik Buchanan, Ryan Roemer, Hovav Shacham, and Stefan Savage. 2008. When good instructions go bad: Generalizing return-oriented programming to RISC. In _Proceedings of the 15th ACM conference on Computer and communications security_ . ACM, 2738. 

- [12] Claudio Canella, Jo Van Bulck, Michael Schwarz, Moritz Lipp, Benjamin von Berg, Philipp Ortner, Frank Piessens, Dmitry Evtyushkin, and Daniel Gruss. 2018. A Systematic Evaluation of Transient Execution Attacks and Defenses. _arXiv preprint arXiv:1811.05441_ (2018). 

- [13] G. Chen, S. Chen, Y. Xiao, Y. Zhang, Z. Lin, and T. H. Lai. 2019. SgxPectre: Stealing Intel Secrets from SGX Enclaves Via Speculative Execution. In _2019 IEEE European Symposium on Security and Privacy (EuroS P)_ . 142157. https: //doi.org/10.1109/EuroSP.2019.00020 

- [14] Yuan Chou, Brian Fahs, and Santosh Abraham. 2004. Microarchitecture optimizations for exploiting memory-level parallelism. In _Computer Architecture, 2004. Proceedings. 31st Annual International Symposium on_ . IEEE, 7687. 

- [15] Debian 2018. _Debian Bug report logs - #886367 intel-microcode: spectre microcode updates_ . Debian. https://bugs.debian.org/cgi-bin/bugreport.cgi?bug= 886367. 

- [16] Dmitry Evtyushkin, Ryan Riley, Nael CSE Abu-Ghazaleh, Dmitry Ponomarev, et al. 2018. BranchScope: A New Side-Channel Attack on Directional Branch Predictor. In _Proceedings of the Twenty-Third International Conference on Architectural Support for Programming Languages and Operating Systems_ . ACM, 693707. 

- [17] Jacob Fustos, Farzad Farshchi, and Heechul Yun. 2019. SpectreGuard: An Efficient Data-centric Defense Mechanism against Spectre Attacks.. In _DAC_ . 611. 

- [18] Cesar Pereida Garcia and Billy Bob Brumley. 2017. Constant-Time Callees with Variable-Time Callers. In _USENIX Security Symposium_ . USENIX Association, 8398. 

- [19] Cesar Pereida Garcia, Billy Bob Brumley, and Yuval Yarom. 2016. Make Sure DSA Signing Exponentiations Really are Constant-Time. In _ACM Conference on Computer and Communications Security_ . ACM, 16391650. 

- [20] Kourosh Gharachorloo, Anoop Gupta, and John L Hennessy. 1991. Two techniques to enhance the performance of memory consistency models. (1991). https: //courses.engr.illinois.edu/cs533/sp2019/reading_list/gharachorloo91two.pdf. 

- [21] Google 2018. _Retpoline: a software construct for preventing branch-targetinjection_ . Google. https://support.google.com/faqs/answer/7625886. 

- [22] Google 2018. _Speculative Load Hardening_ . Google. https: //docs.google.com/document/d/1wwcfv3UV9ZnZVcGiGuoITT_61e_ Ko3TmoCS3uXLcJR0/edit#heading=h.phdehs44eom6. 

- [23] Daniel Gruss, Moritz Lipp, Michael Schwarz, Richard Fellner, Clmentine Maurice, and Stefan Mangard. 2017. Kaslr is dead: long live kaslr. In _International Symposium on Engineering Secure Software and Systems_ . Springer, 161176. 

- [24] Daniel Gruss, Raphael Spreitzer, and Stefan Mangard. 2015. Cache Template Attacks: Automating Attacks on Inclusive Last-Level Caches. In _USENIX Security Symposium_ . USENIX Association, 897912. 

- [25] Intel 2018. _Deep Dive: Intel Analysis of L1 Terminal Fault_ . Intel. https://software.intel.com/security-software-guidance/insights/deep-diveintel-analysis-l1-terminal-fault. 

- [26] Intel 2018. _Details and Mitigation Information for Variant 4_ . Intel. https://newsroom.intel.com/editorials/addressing-new-research-for-sidechannel-analysis/#gs.4778nz. 

- [27] Intel. 2018. Intel Analysis of Speculative Execution Side Channels. https://software.intel.com/security-software-guidance/apiapp/sites/default/files/336983-Intel-Analysis-of-Speculative-ExecutionSide-Channels-White-Paper.pdf. 

- [28] Intel 2018. _Retpoline: A Branch Target Injection Mitigation_ . Intel. https://software.intel.com/sites/default/files/managed/1d/46/Retpoline-ABranch-Target-Injection-Mitigation.pdf. 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

- [29] Intel. 2018. Speculative Execution Side Channel Mitigations. https: //software.intel.com/security-software-guidance/api-app/sites/default/files/ 336996-Speculative-Execution-Side-Channel-Mitigations.pdf. 

- [30] Gorka Irazoqui Apecechea, Mehmet Sinan Inci, Thomas Eisenbarth, and Berk Sunar. 2014. Wait a Minute! A fast, Cross-VM Attack on AES. In _RAID (Lecture Notes in Computer Science)_ , Vol. 8688. Springer, 299319. 

- [31] Khaled N Khasawneh, Esmaeil Mohammadian Koruyeh, Chengyu Song, Dmitry Evtyushkin, Dmitry Ponomarev, and Nael Abu-Ghazaleh. 2018. SafeSpec: Banishing the Spectre of a Meltdown with Leakage-Free Speculation. _arXiv preprint arXiv:1806.05179_ (2018). 

- [32] Vladimir Kiriansky, Ilia Lebedev, Saman Amarasinghe, Srinivas Devadas, and Joel Emer. 2018. DAWG: A defense against cache timing attacks in speculative execution processors. In _2018 51st Annual IEEE/ACM International Symposium on Microarchitecture (MICRO)_ . IEEE, 974987. 

- [33] Vladimir Kiriansky and Carl Waldspurger. 2018. Speculative buffer overflows: Attacks and defenses. _arXiv preprint arXiv:1807.03757_ (2018). 

- [34] Paul Kocher, Jann Horn, Anders Fogh, Daniel Genkin, Daniel Gruss, Werner Haas, Mike Hamburg, Moritz Lipp, Stefan Mangard, Thomas Prescher, et al. 2019. Spectre Attacks: Exploiting Speculative Execution. In _40th IEEE Symposium on Security and Privacy_ . 

- [35] Esmaeil Mohammadian Koruyeh, Khaled N Khasawneh, Chengyu Song, and Nael Abu-Ghazaleh. 2018. Spectre returns! speculation attacks using the return stack buffer. In _12th USENIX Workshop on Offensive Technologies, WOOT_ . 1314. 

- [36] Moritz Lipp, Michael Schwarz, Daniel Gruss, Thomas Prescher, Werner Haas, Anders Fogh, Jann Horn, Stefan Mangard, Paul Kocher, Daniel Genkin, et al. 2018. Meltdown: Reading kernel memory from user space. In _27th USENIX Security Symposium (USENIX Security 18)_ . 973990. 

- [37] LWN 2018. _A page-table isolation update_ . LWN. https://lwn.net/Articles/752621/. [38] Giorgi Maisuradze and Christian Rossow. 2018. ret2spec: Speculative execution using return stack buffers. In _Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security_ . ACM, 21092122. 

- [39] Andrea Mambretti, Alexandra Sandulescu, Matthias Neugschwandtner, Alessandro Sorniotti, and Anil Kurmus. 2019. Two methods for exploiting speculative control flow hijacks. In _13th USENIX Workshop on Offensive Technologies (WOOT 19)_ . 

- [40] Clmentine Maurice, Christoph Neumann, Olivier Heen, and Aurlien Francillon. 2015. C5: cross-cores cache covert channel. In _International Conference on Detection of Intrusions and Malware, and Vulnerability Assessment_ . Springer, 4664. 

- [41] Ross Mcilroy, Jaroslav Sevcik, Tobias Tebbi, Ben L Titzer, and Toon Verwaest. 2019. Spectre is here to stay: An analysis of side-channels and speculative execution. _arXiv preprint arXiv:1902.05178_ (2019). 

- [42] Frank McKeen, Ilya Alexandrovich, Alex Berenzon, Carlos V Rozas, Hisham Shafi, Vedvyas Shanbhogue, and Uday R Savagaonkar. 2013. Innovative instructions and software model for isolated execution. In _Proceedings of the 2nd International Workshop on Hardware and Architectural Support for Security and Privacy_ . ACM. http://software.intel.com/sites/default/files/article/413936/hasp2013-innovative-instructions-and-software-model-for-isolated-execution.pdf 

- [43] Microsoft 2018. _Mitigating speculative execution side channel hardware vulnerabilities_ . Microsoft. https://blogs.technet.microsoft.com/srd/2018/03/15/mitigatingspeculative-execution-side-channel-hardware-vulnerabilities/. 
