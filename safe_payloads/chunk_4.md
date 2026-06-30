
- [16] RedHat Inc. 2022. _CVE-2022-4543_ . https://access.redhat.com/security/cve/cve2022-4543 

- [17] Intel. 2020. _Intel 64 and IA-32 Architectures Software Developers Manual: System Programming, Volume 3_ . 

- [18] Seth Jenkins. 2022. _Exploiting CVE-2022-42703 - Bringing back the stack attack_ . https://googleprojectzero.blogspot.com/2022/12/exploiting-CVE-202242703-bringing-back-the-stack-attack.html 

- [19] Ken Johnson. 2018. _KVA Shadow: Mitigating Meltdown on Windows_ . https://msrc. microsoft.com/blog/2018/03/kva-shadow-mitigating-meltdown-on-windows/ 

HASP 23, October 29, 2023, Toronto, Canada 

William Liu, Joseph Ravichandran, and Mengjia Yan 

- [20] Taehun Kim, Taehyun Kim, and Youngjoo Shin. 2021. Breaking KASLR Using Memory Deduplication in Virtualized Environments. _Electronics_ 10, 17 (2021). https://www.mdpi.com/2079-9292/10/17/2174 

- [21] Paul Kocher, Jann Horn, Anders Fogh, Daniel Genkin, Daniel Gruss, Werner Haas, Mike Hamburg, Moritz Lipp, Stefan Mangard, Thomas Prescher, Michael Schwarz, and Yuval Yarom. 2019. Spectre Attacks: Exploiting Speculative Execution. In _2019 IEEE Symposium on Security and Privacy (SP)_ . 119. https://doi.org/10.1109/ SP.2019.00002 

- [22] Moritz Lipp, Michael Schwarz, Daniel Gruss, Thomas Prescher, Werner Haas, Anders Fogh, Jann Horn, Stefan Mangard, Paul Kocher, Daniel Genkin, Yuval Yarom, and Mike Hamburg. 2018. Meltdown: Reading Kernel Memory from User Space. In _Proceedings of the 27th USENIX Conference on Security Symposium_ (Baltimore, MD, USA) _(SEC18)_ . USENIX Association, USA, 973990. 

- [23] Fangfei Liu, Yuval Yarom, Qian Ge, Gernot Heiser, and Ruby B. Lee. 2015. LastLevel Cache Side-Channel Attacks are Practical. In _2015 IEEE Symposium on Security and Privacy_ . 605622. https://doi.org/10.1109/SP.2015.43 

- [24] William Liu. 2022. _CVE-2022-0185 - Winning a $31337 Bounty after Pwning Ubuntu and Escaping Googles KCTF Containers._ https://www.willsroot.io/2022/01/cve2022-0185.html 

- [25] William Liu. 2022. _CVE-2022-4543: KASLR Leakage Achievable even with KPTI through Prefetch Side-Channel_ . https://www.openwall.com/lists/oss-security/ 2022/12/16/3 

- [26] William Liu. 2022. _EntryBleed: Breaking KASLR under KPTI with Prefetch (CVE2022-4543)_ . https://www.willsroot.io/2022/12/entrybleed.html 

- [27] Colin Percival. 2009. Cache missing for fun and profit. (08 2009). 

- [28] Joseph Ravichandran and Michael Wang. 2022. _Lord of the io_urings_ . Technical Report. 

- [29] Lszl Szekeres, Mathias Payer, Tao Wei, and Dawn Song. 2013. SoK: Eternal War in Memory. In _2013 IEEE Symposium on Security and Privacy_ . 4862. https: //doi.org/10.1109/SP.2013.13 

- [30] Wenhao Wang, Guoxing Chen, Xiaorui Pan, Yinqian Zhang, XiaoFeng Wang, Vincent Bindschaedler, Haixu Tang, and Carl A. Gunter. 2017. Leaky Cauldron on the Dark Land: Understanding Memory Side-Channel Hazards in SGX. In _Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security_ (Dallas, Texas, USA) _(CCS 17)_ . Association for Computing Machinery, New York, NY, USA, 24212434. https://doi.org/10.1145/3133956.3134038 

- [31] Yuval Yarom and Katrina Falkner. 2014. FLUSH+RELOAD: A High Resolution, Low Noise, L3 Cache Side-Channel Attack. In _Proceedings of the 23rd USENIX Conference on Security Symposium_ (San Diego, CA) _(SEC14)_ . USENIX Association, USA, 719732. 

HASP 23, October 29, 2023, Toronto, Canada 

EntryBleed: A Universal KASLR Bypass against KPTI on Linux 

## **A ENTRYBLEED POC** 

~~<mark></mark>~~ <mark></mark> 1 <mark>#include <stdio.h></mark> 2 <mark>#include <stdlib.h></mark> 3 <mark>#include <stdint.h></mark> 4 5 <mark>#define KERNEL_LOWER_BOUND 0xffffffff80000000ull</mark> 6 <mark>#define KERNEL_UPPER_BOUND 0xffffffffc0000000ull</mark> 7 <mark>#define entry_SYSCALL_64_offset 0xe00000ull</mark> 8 9 <mark>uint64_t sidechannel(uint64_t addr) {</mark> 10 <mark>uint64_t a, b, c, d;</mark> 11 <mark>asm volatile (".intel_syntax noprefix;"</mark> 12 <mark>"mfence;"</mark> 13 <mark>"rdtscp;"</mark> 14 <mark>"mov %0, rax;"</mark> 15 <mark>"mov %1, rdx;"</mark> 16 <mark>"xor rax, rax;"</mark> 17 <mark>"lfence;"</mark> 18 <mark>"prefetchnta qword ptr [%4];"</mark> 19 <mark>"prefetcht2 qword ptr [%4];"</mark> 20 <mark>"xor rax, rax;"</mark> 21 <mark>"lfence;"</mark> 22 <mark>"rdtscp;"</mark> 23 <mark>"mov %2, rax;"</mark> 24 <mark>"mov %3, rdx;"</mark> 25 <mark>"mfence;"</mark> 26 <mark>".att_syntax;"</mark> 27 <mark>: "=r" (a), "=r" (b), "=r" (c), "=r" (d)</mark> 28 <mark>: "r" (addr)</mark> 29 <mark>: "rax", "rbx", "rcx", "rdx");</mark> 30 <mark>a = (b << 32) | a;</mark> 31 <mark>c = (d << 32) | c;</mark> 32 <mark>return c - a;</mark> 33 <mark>}</mark> 34 35 <mark>#define STEP 0x200000ull</mark> 36 <mark>#define SCAN_START KERNEL_LOWER_BOUND + entry_SYSCALL_64_offset</mark> 37 <mark>#define SCAN_END KERNEL_UPPER_BOUND + entry_SYSCALL_64_offset</mark> 38 39 <mark>#define DUMMY_ITERATIONS 5</mark> 40 <mark>#define ITERATIONS 1000</mark> 41 <mark>#define ARR_SIZE (SCAN_END - SCAN_START) / STEP</mark> 42 43 <mark>uint64_t leak_syscall_entry(void)</mark> 44 <mark>{</mark> 45 <mark>uint64_t data[ARR_SIZE] = {0};</mark> 46 <mark>uint64_t min = ~0, addr = ~0;</mark> 47 48 <mark>for (int i = 0; i < ITERATIONS + DUMMY_ITERATIONS; i++)</mark> 49 <mark>{</mark> 50 <mark>for (uint64_t idx = 0; idx < ARR_SIZE; idx++)</mark> 51 <mark>{</mark> 52 <mark>uint64_t test = SCAN_START + idx * STEP;</mark> 53 <mark>syscall(104);</mark> 54 <mark>uint64_t time = sidechannel(test);</mark> 55 <mark>if (i >= DUMMY_ITERATIONS)</mark> 56 <mark>data[idx] += time;</mark> 57 <mark>}</mark> 58 <mark>}</mark> 59 60 <mark>for (int i = 0; i < ARR_SIZE; i++)</mark> 61 <mark>{</mark> 62 <mark>data[i] /= ITERATIONS;</mark> 63 <mark>if (data[i] < min)</mark> 64 <mark>{</mark> 65 <mark>min = data[i];</mark> 66 <mark>addr = SCAN_START + i * STEP;</mark> 67 <mark>}</mark> 68 <mark>printf("%llx %ld\n", (SCAN_START + i * STEP), data[i]);</mark> 69 <mark>}</mark> 70 71 <mark>return addr;</mark> 72 <mark>}</mark> 73 74 <mark>int main()</mark> 75 <mark>{</mark> 76 <mark>printf ("KASLR base %llx\n", leak_syscall_entry() - entry_SYSCALL_64_offset);</mark> 77 <mark>}</mark> ~~<mark></mark>~~ <mark></mark> 

# SOURCE: 2412.12814v1.pdf

# Evaluating tamper resistance of digital forensic artifacts during event reconstruction 

Celine Vanini<sup>a</sup> , Chris Hargreaves<sup>b</sup> , Frank Breitinger<sup>a</sup> 

aSchool of Criminal Justice, University of Lausanne, 1015 Lausanne, Switzerland 

> bDepartment of Computer Science, University of Oxford, Wolfson Building, Parks Road, Oxford OX1 3QD, United Kingdom 

## Abstract 

Event reconstruction is a fundamental part of the digital forensic process, helping to answer key questions like who, what, when, and how. A common way of accomplishing that is to use tools to create timelines, which are then analyzed. However, various challenges exist, such as large volumes of data or contamination. While prior research has focused on simplifying timelines, less attention has been given to tampering, i.e., the deliberate manipulation of evidence, which can lead to errors in interpretation. This article addresses the issue by proposing a framework to assess the tamper resistance of data sources used in event reconstruction. We discuss factors affecting data resilience, introduce a scoring system for evaluation, and illustrate its application with case studies. This work aims to improve the reliability of forensic event reconstruction by considering tamper resistance. 

Keywords: Event Reconstruction, Resistance, Tampering, Timeline, Digital Traces, Terminology, Factors 

## 1. Introduction 

Event reconstruction is a fundamental phase in digital forensic investigations where examiners attempt to answer the questions of who, what, when, whom/what with, where, and how after a crime or incident occurred [1]. The reconstruction process often starts with the creation of a timeline using automatic tools such as Plaso<sup>1</sup> , or other (commercial) tools. These tools extract information contained within the file system as well as application-related files and then chronologically organize the data from these different sources. 

Timeline analysis is the second most commonly used digital forensic technique, after keyword searching [2]. However, analyzing timelines poses significant challenges, particularly due to the large amount of information they contain, which makes the process time-consuming. Prior research has often concentrated on methods to reduce timeline complexity, such as filtering, labelling [3], and aggregation [4]. 

Analysis of timelines relies on the extracted timestamps being correct, but these timestamps, like all digital evidence, can be vulnerable to tampering. Despite its importance, the issue of tampering, i.e., the deliberate manipulation of evidence by adversaries, has received less attention. In timelines, this can result in incorrect ordering, aggregation, or filtering of entries, leading to substantial errors in interpretation. When tampering occurs, the risk of misinterpretation rises significantly [5], an error that was not explicitly covered in the consideration of the timeline analysis technique in recent work on tool error in [6]. 

> Email addresses: celine.vanini@unil.ch (Celine Vanini), christopher.hargreaves@cs.ox.ac.uk (Chris Hargreaves), frank.breitinger@unil.ch (Frank Breitinger) URL: https://FBreitinger.de (Frank Breitinger) 

> 1https://github.com/log2timeline/plaso 

Tampering is not a fictive problem. For instance, World AntiDoping Agency [7] discusses the World Anti-Doping Agency vs. Russian Anti-Doping Agency case. The examiners had to analyze MySQL databases using MyISAM storage engine on an Ubuntu Server and developed an approach to find alteration and/or deletion of database records: (1) Recovery (carving) to obtain historical backups of MySQL databases (e.g., MySQL dumps, file-level backups); (2) comparing the recovered database versions to detect specific alterations of records, enabling targeted analysis (rows added, deleted, altered and/or copied in each table); (3) targeted analysis of database tables to create customized content carving methods to recover historical data; (4) in-depth analysis of recovered database table structures to detect specific records only existing in the deleted state, as well as out-of-sequence records due to overwritten records; (5) comparing timestamps in databases and on the file systems to detect backdating. While for this case the examiners had only one source (databases), several sources may be encountered during an investigation providing divergent information. 

To help investigators interpret data, the C-Scale (also referred to as the Strength of Evidence scale) may be applied [5]. The scale aims at helping practitioners to express their evaluative opinion in a more understandable and refined manner, at the final stages of the investigation. It involves two essential components: the number of sources that align and their resistance to tampering. As outlined by the C-Scale, evidence becomes stronger when multiple independent sources agree, particularly if these sources are tamper-proof or more difficult to manipulate. 

The C-Scale is a powerful resource but it requires practitioners to differentiate between tamper-resistant and not tamperresistant sources. While some investigators may intuitively consider this, research to date has not attempted to evaluate the tamper resistance of artefacts (data sources) used as the basis for 

Preprint submitted to Elsevier 

December 18, 2024 

event reconstruction. Therefore, this paper aims to bridge that gap and explores factors that could be used to formally evaluate the resistance against tampering of artefacts used as the basis for event reconstruction. 

In summary, this article provides the following contributions: we assess the resilience of artefacts by providing an extensive discussion of factors that may affect their resistance to any active modifications and/or deletions in a contextual manner. Additionally, we propose a scoring system that can be used to support the evaluation (Sec. 5). Ultimately, we illustrate the use of the scoring with a set of case study examples (Sec. 6). 

## 2. Terminology 

Jaquet-Chiffelle and Casey [8] formally define a Trace in the context of forensic science as well as several other concepts. For this paper, only the following concepts are relevant (some definitions are shortened for brevity): 

An event is a complete collection of related things that have happened (or are happening) in a World within a specific closed interval of time. The authors do not make use of this term to represent the output of a typical timestamp extraction from digital data. This then leads to a Trace being defined as the full modification of the Scene [...] resulting from the Event E, completed or not, and subsequent intrinsic events. An important part here is that the Trace is every modification that happens. However in reality, what we end up observing are facets of the Trace. This is explained further as when scientists study a Trace [...] only certain facets are observed, and other facets remain unobserved due to lacking knowledge, methods, technology, or resources. They go on to describe that explanations often do not make that distinction and that a trace is very often described according to a particular observed facet and the perspective that is chosen to observe this facet. 

Given the focus on timelines in this article, these definitions and this explanation are essential. We acknowledge that an event creates many modifications. These consist of different types of digital and non-digital traces. Timeline analysis involves the extraction of specific facets that are centred around a timestamp, but also with associated data attached to that timestamp. For example, usually recorded is a timestamp, its context (e.g., that it is in the last modified field of a Standard Information Attribute (SIA) within an MFT record), and other information such as the MFT record ID, the filename that the timestamp relates to, etc. Depending on the software used to generate the timeline, there may be even more data. 

Note, while the timestamp from a timeline perspective is associated with a specific object (a specific NTFS file in this example), the timestamp also has a source, which in this case is the SIA within the MFT. We argue that considering the source of the timestamp is important when studying tampering. 

## 3. Event reconstruction and its challenges 

Given that we need to observe one or more facets of a Trace in an attempt to perform event reconstruction, several general problems can frustrate this: 
