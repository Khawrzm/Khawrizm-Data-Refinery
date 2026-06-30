---
Title: Khawrizm OS Sovereign Knowledge Base
Status: Verified
---

# SOURCE: 2023.HASP.EntryBleed.pdf

# **EntryBleed: A Universal KASLR Bypass against KPTI on Linux** 

William Liu Joseph Ravichandran Mengjia Yan MIT CSAIL MIT CSAIL MIT CSAIL Cambridge, MA, USA Cambridge, MA, USA Cambridge, MA, USA wliu1@mit.edu jravi@mit.edu mengjiay@mit.edu 

## **ABSTRACT** 

For years, attackers have compromised systems by developing exploits that rely on known locations of kernel code and data segments. KASLR (Kernel Address Space Layout Randomization) is a key mitigation in modern operating systems which hampers these attacks through runtime randomization of the kernel image base address. KPTI (Kernel Page Table Isolation) is another defense mechanism, originally introduced to defend against the 2018 Meltdown attack by unmapping kernel addresses during user code execution. This security mechanism makes it harder for attackers to leak kernel address mappings through micro-architectural side channels. However, a few pages for system call and interrupt handling were exempted from isolation for the sake of user to kernel context transitions. 

We present the EntryBleed vulnerability (CVE-2022-4543) as a universal bypass against the KASLR protection mechanism through a combination of micro-architectural side channels and design flaws in the KPTI mitigation on Intel CPUs. We demonstrate that the bug we identified can accurately de-randomize the kernel address space within a second on modern Intel CPUs in both physical host and hardware-accelerated virtual machine environments. We then provide a root cause analysis to locate the core micro-architectural behaviors that enable EntryBleed, both on physical and under virtualized environments. Furthermore, we propose a performant mitigation based closely upon a pre-existing KASLR hardening mechanism. If left unpatched, attackers will be able to easily bypass KASLR, greatly lowering the barrier for exploit development and increasing the risk of serious threats against the Linux operating system. 

## **CCS CONCEPTS** 

 **Security and privacy**  **Side-channel analysis and countermeasures** ; **Operating systems security** . 

## **KEYWORDS** 

micro-architecture, side-channel, Linux kernel, ASLR, KPTI 

### **ACM Reference Format:** 

William Liu, Joseph Ravichandran, and Mengjia Yan. 2023. EntryBleed: A Universal KASLR Bypass against KPTI on Linux. In _Hardware and Architectural Support for Security and Privacy 2023 (HASP 23), October 29, 2023,_ 

Permission to make digital or hard copies of part or all of this work for personal or classroom use is granted without fee provided that copies are not made or distributed for profit or commercial advantage and that copies bear this notice and the full citation on the first page. Copyrights for third-party components of this work must be honored. For all other uses, contact the owner/author(s). _HASP 23, October 29, 2023, Toronto, Canada_ 

 2023 Copyright held by the owner/author(s). ACM ISBN 979-8-4007-1623-2/23/10. https://doi.org/10.1145/3623652.3623669 

_Toronto, Canada._ ACM, New York, NY, USA, 9 pages. https://doi.org/10.1145/ 3623652.3623669 

## **1 INTRODUCTION** 

Traditionally, low-level security research has focused on memory and thread safety [29]. Researchers studied attacks and defenses against memory bugs like buffer overflows or use-after-frees and concurrency bugs like race conditions. An attackers goal is to corrupt program memory and hijack its execution flow to gain more privileges in a victims device. 

With the discovery of the Meltdown [22] and Spectre [21] attacks in 2018, micro-architectural vulnerabilities came to the spotlight. These bugs do not result from programming errors, but rather from hardware design choices emphasizing aggressive performance optimizations. They mostly lead to side channels, i.e., attacks which leak sensitive system secrets through measurements of external and unintended side-effects of program behavior. For example, speculative execution allows instructions to execute transiently ahead of time in the CPU pipeline even if these instructions are illegal from a permission (Meltdown) or control flow (Spectre) perspective. While the hardware has a built-in rollback mechanism, it fails to rewind all modified micro-architectural states, leaving unintentional side effects that serve as sources of information leakage. Leaked secrets include cryptographic private keys [27], secure enclave contents [30], and KASLR layout [22]. 

Note that, among the secrets that can be leaked via microarchitectural attacks, KASLR layout is of special importance due to its role in kernel hardening, as it is a resilience measure against software exploits. In fact, Meltdown [22] can be used to leak sensitive program data across security boundaries, including kernel pointers that could lead to a KASLR bypass. This papers EntryBleed attack reveals TLB-resident kernel virtual addresses which can derandomize KASLR layout under the protection of KPTI, an important memory isolation mechanism between kernel and userspace as well as a KASLR hardening feature in the post-Meltdown era. 

## **1.1 KASLR Security** 

KASLR [11] is an important security feature that randomizes the layout of kernel memory regions (including code, the heap, the kernel stack, etc.) on each reboot. The Meltdown attack led to major security complications as it trivially broke this randomization barrier for attackers seeking to exploit memory corruption vulnerabilities. In particular, Meltdown relies on the shared page tables (further discussed in Section 2.1) between kernel and userspace. With this sharing, the CPU can continue its aggressive speculative execution across privilege boundaries as long as any requested address is resolvable by the MMU (Memory Management Unit). 

HASP 23, October 29, 2023, Toronto, Canada 

William Liu, Joseph Ravichandran, and Mengjia Yan 

The urgent need to address this vulnerability led to the current state-of-the-art mitigation known as KPTI (Kernel Page Table Isolation). This security mechanism separates kernel and user page table entries. As a result, upon switching between privilege levels, the OS is required to switch its top-level page table pointer. Most commercial operating systems (such as Windows) have adopted similar approaches [19] in an attempt to stop micro-architectural attacks like Meltdown. Since most production operating systems utilize KPTI as a hardening mechanism, a potential failure in KPTI to protect against KASLR bypasses has serious security implications for standard user and organizational threat models. 

Unfortunately, many have made overarching assumptions about the security of KPTI, overlooking that certain classes of microarchitectural attacks are still applicable as a bypass against KASLR in spite of KPTI. Even as of December 2022, the prominent security research group Google ProjectZero [18] wrote that KPTI mitigates prefetch attacks [14] across privilege boundaries. We show that this presumption is incorrect with the EntryBleed attack. 

## **1.2 EntryBleed** 

We recognized a design flaw in Linux KPTI related to insufficient isolation between the userspace and kernelspace addresses. Specifically, as the OS needs to handle exceptions, interrupts, and syscalls from userspace, there are still tiny stubs of kernel addresses mapped into user page tables, serving as the entry and exit portal for userland code. We refer to this code as a **trampoline region** . We hypothesized that the trampoline region could be a leakage source for a micro-architectural-based KASLR bypass. Based on this hypothesis, we extended the prefetch-based side channel attack to construct a universal KASLR bypass. Our attack is resilient to normal operating system noise, does not require per-system configuration tuning, and works within a second. 

An especially interesting question is the true root cause of this vulnerability on both physical hosts and hardware-accelerated VM environments. We found that in both contexts, the attack works because immediately before returning back from kernelspace to userland code, the TLB caches the page translation for the trampoline addresses. As such, the TLB states can then be inspected later in userspace via a prefetch side channel to leak the KASLR layout. 

Furthermore, it is of interest to understand how EntryBleed works in a hardware-accelerated VM environment, especially how the micro-architectural side effects survive across guest-host context switches. We experimentally found this to be due to modern ISA optimizations on virtualized MMUs. For the purposes of this project, the scope for the VM-related analysis focuses on Intel VT-x extensions utilized in the KVM hypervisor environment, with special emphasis on the MMU optimizations EPT, VPID [17], and shadow paging, as these features are extremely common in personal and commercial computing environments. Additionally, since this vulnerability is still unpatched and exploitable in the wild, we propose an effective and performant mitigation based on pre-existing work on KASLR hardening. 

_Our Contributions._ As of now, we have made the following contributions. 

- We discovered Entrybleed, a security issue which affects current production kernels. 

- We present a study of the root causes of EntryBleed in both bare metal and hardware accelerated virtualized environments. 

- We provide a potential fix to this systematic vulnerability that aims to also be performant, based closely upon the pre-existing FG-KASLR [4] mitigation. 

_Disclosure._ RedHat has publicly acknowledged EntryBleeds threat to KPTI (designated under CVE-2022-4543) [16], and other members of the security community have managed to cross-verify its success, as seen in the KASLD repository [5] for documenting KASLR bypasses. 

_Outline._ Section 2 provides important pre-requisite knowledge for the EntryBleed attack. Section 3 describes the methodology to which we discovered the systematic flaw in KPTI and how we designed our attack along with a root cause analysis. Section 4 follows with our experimental verification of EntryBleed and Section 5 then provides data on our POCs performance, accuracy, runtime, and behavior across a variety of Intel CPUs as well as Linux kernels. We provide metrics for its behavior under different hardware-accelerated VM configurations to further our root cause analysis. Section 6 summarizes the worrying implications of this attack and suggests avenues of potential future research. Section 7 discusses our variation of the pre-existing FG-KASLR mitigation as a defensive measure, and Section 8 concludes our work. 

## **2 BACKGROUND** 

## **2.1 Virtual Memory and Paging** 

To support process isolation, program portability, and memory optimizations, most CPUs support concepts known as virtual memory and paging. Rather than allowing programs to work directly with physical memory provided by DRAM, operating systems abstract it away with virtual addresses mapped to real addresses by the underlying hardware MMU. As this map is also stored in main memory, a direct translation structure would be extremely inefficient. 

The concept of multi-level paging resolves this aforementioned inefficiency. On x86_64, virtual addresses are split into 4 or 5 sections, in which bits from each section act as an index into an array known as a page table. The address for the first sections page table is in the CR3 register [17]. Each index stores the physical address of the array for the next levels page table given the current levels index (along with metadata related to memory properties). The final sections page table stores physical addresses representing either a 4KB, 2MB, or 1GB region of contiguous memory for the virtual address. This multi-level address translation design allows for many optimizations, and even saves memory as page table entries (along with its associated memory) can be populated on demand. 

_Translation Lookaside Buffer._ The above design works well for memory efficiency and abstractions but has one major flaw: each virtual memory access requires 3 to 5 lookups in DRAM (depending on the size of the target page), each of which takes hundreds of cycles. To address this issue, each CPU core maintains a structure known as the TLB, or Translation Look-Aside Buffer, where virtual addresses are mapped to their direct physical addresses. This structure fills up based on successful MMU resolutions of virtual addresses and can automatically evict or evict based on execution 

HASP 23, October 29, 2023, Toronto, Canada 

EntryBleed: A Universal KASLR Bypass against KPTI on Linux 

of specific x86_64 instructions, which the OS utilizes to maintain memory coherency. 

## **2.2 Address Randomization** 

ASLR (Address Space Layout Randomization) is a common userland and kernel security feature enabled on all modern operating systems, in which the memory layout of programs is scrambled per run or boot. Before this mitigation, attackers could just hardcode virtual addresses to desirable memory targets in their exploits. The added factor of randomization often requires attackers to achieve a leakage primitive. Exploits that work independently of ASLR, or without much knowledge of the virtual address space, are also a possibility, but much less common. Aside from greatly increasing attack complexity, it can even neutralize exploitability depending on the bug. 

However, in practice, the randomization only happens at the granularity of program regions such as the heap, stack, or binary image. Operating systems also impose a limit on the randomization granularity by ensuring that program regions remain within certain address ranges. For example, the Linux kernels code and data are mapped at a 2MB boundary; combined with its allowed virtual address range, the total randomization entropy is only 9 bits [20] [6]. While the entropy is somewhat low, an exploit that cannot bypass KASLR would only work in <u>5121</u><sup>attempts (assuming a reliance on</sup> only kernel data and code like in ROP chain attacks), which is much less dangerous and more easily detectable than a fully stable exploit. Hence, some security researchers have been interested in bypassing KASLR through micro-architectural side-channels, such as using Meltdown [22] and the double page fault attack [15]. 

## **2.3 Timing and Prefetch Side-Channels** 

Timing side-channels are vectors which attackers exploit to deduce secrets based on operating compute time that are data or input dependent. A very common toy example is the implementation of a naive memcmp for a password checker: as soon as one character is wrong, the function returns false. This password checker takes longer to finish if an attempt is more correct, thereby making runtime into an oracle for attackers, allowing them to derive the password byte by byte. 

Prior work has shown that various micro-architectural structures can be used to construct side channel attacks. For example, CPUs maintain a hierarchy of memory caches to cache contents in groups of 64 or 128 contiguous bytes known as cache lines. The CPU cache has long been used as a vector for side channels, as seen in the targeting of the L1 as well as L2 cache [27], the LLC cache [23], and full cache hierarchy in the FLUSH+RELOAD attack [31]. This is mainly because clear timing differences arise when measuring data accesses that are active in different levels of the cache hierarchy instead of just DRAM. Though less popular than caches, another micro-architectural structure for side-channels is the TLB. It has been used for side channels in the context of targeting SGX under hyperthreading [30], with the help of machine learning to deduce a victims memory access patterns [12], or, in our case, in tandem with prefetching. 

_Prefetch Attack._ For performance programming reasons, the ISA allows users to preemptively cache virtual memory with a family 

of instructions known as prefetch instructions. These instructions come with a known measurable micro-architectural side effect detectable within the granularity of CPU cycles. A given prefetch instruction will take longer depending on whether the prefetched address is mapped or not. Additionally, trying to prefetch an invalid or kernel memory address will never cause any architecturally visible exceptions, so there is no penalty for trying to prefetch a kernel address from user context. This technique is called the prefetch attack and a known bypass against standard ASLR [14]. 

## **3 THE ENTRYBLEED ATTACK** 

We now discuss the methodology for discovering the EntryBleed bug, in which we study the design of the micro-architectural defense KPTI [7] along with a source analysis of its implementation. We then provide our systematic approach to verifying the vulnerability, followed by a root cause analysis. 

## **3.1 A Security Vulnerability in KPTI** 

KPTI is a defense technique that is introduced to mitigate Meltdown [22] and was believed to be effective towards prefetch attacks [18]. It works by isolating kernel and user page tables. Previously, most systems used a shared kernel and userspace page-table scheme, where the two page tables were separated by only a permission bit in the same page table structure. However out-of-order execution in some CPUs simply ignored the permission bit during transient execution in the pipeline [22], allowing attackers to measure micro-architectural side-effects related to higher-privileged code and data. By isolating their page tables completely with KPTI (as shown in Figure 1a), many micro-architectural attacks which leak secrets from the kernel would no longer work as the CPU is unable to pre-emptively process untranslatable addresses. 

**Figure 1: (a) is the ideal representation of KPTI, in which userland and kernel page tables are completely separated. (b) is the reality, as the userland needs to have kernel trampolines for proper OS functionality.** 

However, we found there are noticeable points of isolation failure in both its design documentation and its implementation in Linux. When executing in userspace, a minimal subset of kernel addresses is still mapped for the sake of trampolining execution into the kernel when handling interrupts, exceptions, and syscalls. This region mostly serves as a way for userland to enter and exit kernelmode, and its mapping is available in the userspace (as shown in Figure 1b). Indeed, in the Linux kernel source, the address of the syscall handler entry_SYSCALL_64 is mapped into the LSTAR register in the function syscall_init() and is available in this 

HASP 23, October 29, 2023, Toronto, Canada 

William Liu, Joseph Ravichandran, and Mengjia Yan 

trampoline region between kernel and userspace [2]. The LSTAR serves as the MSR (Model-Specific Register) which informs the CPU of the instruction pointer to jump to upon syscall invocations [17]. 

Additionally, modern ASLR design only randomizes the start of different sections (such as program code, heap, stack, etc.). Because the syscall handler is part of kernel code, its randomized address will be at a constant offset to every other address in the kernel image. As a result, leaking the syscall handlers address would reveal the address of everything else in the kernel, breaking KASLR and rendering KPTI an ineffective mitigation against prefetch attacks. 

It must be noted that we were not the first to make this observation of this weakness in KPTIs isolation. The EchoLoad [6] microarchitecture attack, which relied on load stalls as a side-channel vector, noticed this as well. There have also been successful Meltdown exploits against this specific region in both Windows from BlueFrost Security Labs [10] and MacOS from RET2 [9]. Indeed, the paper that introduced the basis for KPTI [13] also noted this problem. To our knowledge, we are the first to use the prefetch side channel to exploit this vulnerability on the latest Intel KPTI-enabled Linux kernel and virtualized environments. 

## **3.2 Attack Strategy** 

From the above analysis, one can theorize the following attack scenario as a universal KASLR bypass from an unprivileged user (Figure 2). 

- **Cache the syscall handler in the TLB by making a syscall from userspace.** Recall that a successful virtual to physical address translation will result in TLB caching. Upon returning from kernel space, the CPU still needs to execute a series of epilogue instructions to return back to userspace, forcing this handler to remain cached in the TLB. This works despite the effects of a TLB flush caused by the user to kernel page table register switch. 

- **Guess the kernel address for entry_SYSCALL_64 and prefetch it.** Any address in the shared page table region should work too. As mentioned earlier, this instruction has noticeable side effects on execution latency based on whether the address is cached in the TLB. 

- **Iterate through all possible virtual addresses for the syscall handler based on the virtual address range for the kernel image, logging the execution cycles of prefetch for each.** The shortest measured latency implies that the virtual address is in the TLB, and is the address of the syscall handler page. 

**Figure 2: Visualization of the EntryBleed attack strategy.** 

To properly calculate CPU cycle latency, one can rely on the rdtscp instruction, which returns the value of a CPU clock cycle counter (the finest grained timer accessible to users of any privilege). Additionally, optimizations can be applied to the possible address space based on an analysis of Linux kernel memory mapping, and out of order execution scenarios that inaccurately skew the data can be prevented with serializing instructions like cpuid or mfence. 

## **3.3 Root Cause Analysis** 

Aside from the design flaw discussed in Section 3.1, the root cause of this attack is simple. Looking at the code for entry_SYSCALL_64 in arch/x86/entry/entry_64.S in the Linux source tree (version 6.0) [2], we can see that this part of kernel code starts and ends execution when the CR3 register is still holding the users page table. Hence, the effects of the CR3 switch into kernel space address, which should flush the entire TLB, becomes nullified by the final switch back into user space CR3, thereby keeping this section of memory cached back into the TLB. This loss of state in the TLB is further avoided by the fact that this page is marked with the global bit for performance reasons, which is an x86 feature to avoid TLB flushes on specified pages during a root page table pointer switch. Overall, this type of behavior cannot be easily patched from the entry handlers perspective due to its need to start somewhere in user space to trampoline into the kernel, nor can it easily be patched in the hardware level without some serious modification to the ISA in regards to prefetch semantics. 

As mentioned in the introduction, this micro-architectural attack also functions just as well when under hardware virtualization (specifically tested on Intel VT-x). The root cause is of the same reason, but the functionality of this attack even when faced with VM exits during the side-channel procedure is an interesting observation we made that will be discussed later in Section 5.4. 

## **4 EXPERIMENTAL VERIFICATION** 

To verify the functionality of our proposed micro-architectural attack, we utilized Intel-based systems ranging from 4th generation Haswell architecture to 9th generation Coffee Lake architecture. We did not test chips 10th generation and onwards as they have hardware mitigations against Meltdown built in [8]. With these mitigations Linux automatically disables KPTI; a prefetch attack is already known to work in this case as now the kernel and user share one page table again [14] [18]. 

The EntryBleed exploit was tested on Linux kernel builds with hardening settings standard for both personal and cloud computing purposes, including KPTI and KASLR, the two main victims in this attack. Several of systems were also popular Linux distributions, equipped with built-in micro-architectural hardening measures such as retpolines and the latest Intel microcode updates. We also relied on Linuxs KVM hypervisor driver to test the attack under hardware accelerated virtualization environments. 

To accurately replicate the scenario of an attacker attempting to perform LPE (Local Privilege Escalation), we created a standard low-privileged user account and ran code that performed the attack described previously in Section 3.2. 

We then transferred the attack binary over and ran it to leak the address of entry_SYSCALL_64, and confirmed the results through 

HASP 23, October 29, 2023, Toronto, Canada 

EntryBleed: A Universal KASLR Bypass against KPTI on Linux 

<!-- Start of picture text -->
180 140<br>160<br>160 120<br>140<br>140<br>100<br>120<br>120<br>100 80<br>100<br>0xffffffff8a400000, 58 80 0xffffffffb7600000, 50 60 0xffffffffa1600000, 35<br>80<br>60<br>60 40<br>             0xffffffff80000000 0xffffffffc0000000                  0xffffffff80000000 0xffffffffc0000000                  0xffffffff80000000 0xffffffffc0000000<br>Address Address Address<br>(a) 5.4.0-146-generic + Intel i5-4590 (b) 5.15.0-67-generic + Intel i7-6700 (c) 5.15.0-83-generic + Intel i7-9750H<br>Prefetch Latency (Cpu Cycles) Prefetch Latency (Cpu Cycles) Prefetch Latency (Cpu Cycles)<br><!-- End of picture text -->

**Figure 3: Visualization of prefetch CPU cycles when side-channeling for the address of entry_SYSCALL_64** 

**Table 1: Successful Experimental System Configurations** 

*** Cloud service (Digital Ocean) did not provide information for exact CPU model** 

|**CPU Model**|**Kernel Build**|**System Environment**|**Tested under KVM**|
|---|---|---|---|
|Intel i5-8265U|Arch 6.0.12-hardened1-1-hardened|PC|Yes|
|Intel i7-9750H|5.15.0-83-generic/custom 5.18.3|PC|Yes|
|Intel i7-9700F|6.0.12-1-MANJARO|PC|Yes|
|Intel i7-6700|5.15.0-56-generic|Server|No|
|Intel i5-4590|5.4.0-146-generic|Server|No|
|Intel Xeon CPU E5-2640|5.10.0-19-amd64|Cloud|No|
|(DO) Intel Xeon Skylake<sup></sup>|5.4.0-139-generic|Cloud|No|

the /proc/kallsyms pseudo-file interface as a higher privileged user. As discussed in Section 3.1, the offset of entry_SYSCALL_64 is at a constant offset relative to the kernel base symbol startup_64 so its leakage effectively breaks KASLR for any given kernel build. 

In the end, we developed a POC that was around 100 lines of C, and it successfully leaked KASLR base under a second with a high degree of accuracy. One can find our original reference implementation and security report at https://www.openwall.com/ lists/oss-security/2022/12/16/3 [25] and https://www.willsroot.io/ 2022/12/entrybleed.html [26], or an updated version that achieves higher accuracy in Appendix A. 

## **5 RESULTS** 

We now present an analysis of the effectiveness of EntryBleed across a variety of system configurations, demonstrating its near perfect accuracy and quick performance across many systems. We also present a root cause analysis of the EntryBleed side channel mechanism on bare metal and insights on how hardware virtualization optimizations affect the attack. 

## **5.1 Observable Effects of EntryBleed** 

Figure 3 showcases clearly observable effects of the EntryBleed attack for an attacker attempting to bypass KASLR on modern Linux kernels with KPTI enabled. 

In each of the graphs in Figure 3, the prefetch leakage code used ran for 1000 times at each KASLR address granularity (which should be 0x200000) and bounded the search range from 0xffffffff80000000 to 0xffffffffc0000000, the x86_64 range of possible KASLR bases [3]. As shown in the graphs in Figure 3 

(which relate a potential kernel virtual address to its prefetch instruction CPU execution latency), there is a noticeable drop in latency from over 100 cycles to around 35 to 60 cycles at each measurement of the entry_SYSCALL_64 region. The observed latency drop is due to the mapped address being cached in the TLB, preventing the need for a page table walk. The address for entry_SYSCALL_64 experienced the first major drop in latency on all the systems we analyzed. 

## **5.2 Scope of Vulnerability** 

We tested the POC on the following Intel CPU models across the Linux kernel versions as seen in Table 1. Note that AMD CPU models were not considered in our attack as they were never vulnerable to Meltdown, so KPTI would have never been enabled under normal conditions for those systems. 

## **5.3 Accuracy and Performance of EntryBleed** 

In Table 2 we demonstrate the attacks accuracy and speed. This is in stark juxtaposition to many other attacks in the literature which require much longer times (ranging from minutes to hours) in laboratory conditions, oftentimes requiring a post data analysis period to extract meaningful results. When run under normal system conditions, EntryBleed finishes in under half a second with effectively 100% accuracy by only taking simple averages of 1000 latency measurements, and works just as well under standard virtualization configurations. Even if the accuracy rates were to decrease on a noisier system, an attacker can easily re-run the attack for as many times as needed to confidently deduce KASLR base due to its speed. 

HASP 23, October 29, 2023, Toronto, Canada 

William Liu, Joseph Ravichandran, and Mengjia Yan 

**Table 2: Average time to leak KASLR and accuracy rate of EntryBleed (per 1000 runs of POC)** 

|**CPU Model**|**Kernel**<br>**Version**|**Average**<br>**Leakage**<br>**Time(s)**|**Accuracy**<br>**Rate**|
|---|---|---|---|
|Intel i5-4590|5.4.0-146|0.2236|100%|
|Intel i7-9750H|5.15.0-83|0.2761|99.7%|
|Intel i7-6700|5.15.0-67|0.1334|99.6%|
|Intel i7-9750H<br>(KVM)|5.15.0-58|0.4148|99.9%|

Another interesting question was in regards to the number of iterations needed for an accurate leakage (the default number of iterations during testing so far was 1000 as seen in Appendix A). We computed the accuracy of running the tests repeatedly to study this, starting from just 1 iteration of the prefetch attack up to 100. We repeated the test 50 times for each sample and tracked the number of correct KASLR leaks out of 50 attempts for the given iteration. As shown in Figure 4, EntryBleed can achieve a perfect success rate from just a single iteration, making it a remarkably efficient and effective attack. 

<!-- Start of picture text -->
100%<br>50%<br>0%<br>0 20 40 60 80 100<br>Iterations<br>Accuracy<br><!-- End of picture text -->

**Figure 4: Relationship between prefetch iterations to the accuracy of EntryBleed** 

## **5.4 Analysis of Virtualization Behavior** 

Lastly, we analyzed EntryBleed in the context of Intel VT-x (Intels virtualization technology) in Linux KVM in relation to VM relevant MMU optimizations: EPT (Extended Page Tables), VPID (Virtual Processor ID), and shadow MMU. EPT and VPID can be toggled off during load time of the KVM driver, and shadow MMU automatically activates when EPT is off. In this final case, the host maintains a shadow page table mapping guest virtual to host physical addresses that updates based on changes to a guest page table. 

As we are interested in the preservation of the side channels side effects across guest-host context switches, we need to force unconditional VM exits from our userland code. Otherwise, as long as there is no VM exit, it makes sense for hardware accelerated VMs to preserve the entries in the TLB as it is not switching between guests or to host. To do this, we injected a cpuid instruction after the syscall but before the prefetch measurement function, as cpuid triggers unconditional VM exits [17]. Figure 5 presents EntryBleed metrics in relation to different VM configurations. 

Based on Figure 5a, we see that EPT (Extended Page Tables) does not help preserve EntryBleeds side effects across VM exits by itself. This makes sense, as EPT just acts as a second layer page table; each virtual address access not in the TLB triggers a page table walk for guest virtual to guest physical address based on the guest CR3 

register, which then triggers a page table walk from guest physical to host physical address based on the EPT base pointer register in the VMCS (Virtual Machine Control Structure) [17]. EPT in general does not affect the TLB state, aside from storing a cached address translation after a successful page table walk (which shadow paging also performs). 

In contrast, VPID (Virtual Processor ID) plays a major role in preserving the side effects of this side channel attack, as seen in Figure 5b and Figure 5c. VPID allows the TLB to cache address translations for multiple address spaces (similar to Intels PCID technology or ASIDs in other architectures) [17]. The CPU can choose which TLB to use depending on its execution context and avoid TLB flushes when switching between guest and host. Even if a VM exit triggers between the syscall and prefetch measurement of EntryBleed, the guest TLB would not flush as the host TLB is in a separate VPID space. 

In Figure 5d, we observe an interesting effect for when there is only shadow paging without VPID. Somehow, EntryBleed can still observe its effects on the TLB and successfully carry out a prefetch attack, albeit with a much smaller latency difference from an incorrect guess. Currently, the root cause analysis for this phenomenon is unknown to us, but we suspect there might be a multitude of cache related micro-architectural subtleties at play here as well as hypervisor software-specific optimizations. It is still an open question that requires more investigation, either through system performance counters or modification of KVM source. 

## **6 DISCUSSION** 

## **6.1 Implications of EntryBleed** 

The ability to trivially leak KASLR at a nearly perfect success rate across many systems has serious implications for the state of Linux kernel hardening. As mentioned previously, KASLR is a major barrier for many exploits targeting kernel software bugs, with attackers often going to great lengths to obtain address leaks through other corruption bugs and maintain stability of the kernel to continue exploitation. A common scenario is to reuse the same bug for both an address leak and arbitrary memory write or control flow hijacking for privilege escalation, as seen in CVE-2022-0185 [24]. EntryBleed effectively cuts the work of an attacker in half, and can revive the exploitability of bugs previously thought to be unexploitable due to KASLR, such as the Lord of the IO_Urings bug (CVE-2022-29968) [28]. Unlike many other documented KASLR bypasses, EntryBleed is more universal as it is independent of system misconfigurations or special user privileges  in fact, there are no software system settings or available Intel x86 chips that can prevent this. It also finishes in under a second with nearly complete accuracy and can be re-run for as many times as needed due to its simplicity, in contrast to many other micro-architectural attacks. Devices running Linux on Intel systems face a grave and realistic risk against simpler LPE attacks. 

## **6.2 Future Work** 

EntryBleed has only been thoroughly explored on Linux systems, and remains untested on Windows, Darwin, or BSD based systems. Given that most systems adopted a similar approach to KASLR and KPTI, it would be unsurprising to see similar results there, and 

HASP 23, October 29, 2023, Toronto, Canada 

EntryBleed: A Universal KASLR Bypass against KPTI on Linux 

<!-- Start of picture text -->
160 160 140<br>150 140 180<br>140 120 160<br>120<br>130 100 140<br>120 100 120<br>80<br>110 80<br>0xffffffffb9600000, 46 0xffffffff8b400000, 46 100 0xffffffff9a800000, 71<br>100 60 60 80<br>90<br>             0xffffffff80000000 0xffffffffc0000000                  0xffffffff80000000 0xffffffffc0000000                  0xffffffff80000000 0xffffffffc0000000                  0xffffffff80000000 0xffffffffc0000000<br>Address Address Address Address<br>(a) EPT Only (b) EPT + VPID (c) Shadow MMU + VPID (d) Shadow MMU Only<br>Prefetch Latency (Cpu Cycles) Prefetch Latency (Cpu Cycles) Prefetch Latency (Cpu Cycles) Prefetch Latency (Cpu Cycles)<br><!-- End of picture text -->

**Figure 5: Visualization of prefetch CPU cycles under Intel VT-x on an Intel i7 9750H CPU when side-channeling for the address of entry_SYSCALL_64 on a 5.15.0-58-generic kernel. Note how no address was found in the graph when only EPT was enabled.** 

would further increase EntryBleeds threat. Another interesting avenue for future exploration is to see if similar prefetch semantics are vulnerable in mobile architectures like ARM and if an attack similar in style can be launched to bypass KASLR there. 

a leakage primitive for virtual address space derandomization. We also provide an analysis on how the attack can survive across VM exits under Intel VT-x, and conclude with a mitigation proposal based on pre-existing work for FG-KASLR. 

## **7 MITIGATION PROPOSAL** 

One possible solution to EntryBleed would be to relocate the addresses containing kernel exception handlers during boot time, before the exception tables and relevant MSR registers are initialized. While a prefetch attack can still leak the kernel trampolines addresses, their addresses would be at non-constant offsets to kernel base, thereby decoupling their leakage from a KASLR bypass for the rest of the kernel protected by KPTI. This one-time randomization would also add less overhead than a per process randomization of the exception handling code. 

Something similar to this idea already exists in the form of FGKASLR, in which all kernel functions are relocated at randomized offsets at boot time as an exploit mitigation. We have only been able to test the original implementation, which did not randomize assembly based functions so EntryBleed is still functional there; according to the Linux kernel mailing lists [1], the most recent version addresses this issue but we have yet to verify it. It is also known to cause about a second of delay during boot time [4], which is unacceptable for cloud based environments with the growing trend of heavy workloads related to micro-services and on demand VMs. Overall, we believe that only these exposed handlers between user space and kernel space require randomization for an adequate and effective mitigation, but do not have plans to develop a working prototype due to the extremely heavy engineering effort better suited for core developers of the Linux kernel. 

Lastly, although not completely related to EntryBleed, we would advise OS vendors against disabling KPTI in CPUs with hardware Meltdown mitigations, as previous work for the prefetch attack shows. We do not expect these to be the only and last bugs that exploit a shared page table scheme between kernel and userland. 

## **8 CONCLUSION** 

EntryBleed presents an efficient, noise-resilient, and system configuration independent mechanism to bypass KASLR on modern Intel based systems when running under KPTI through the usage of x86 prefetch instructions. Due to its effectiveness and fast rate of leakage, it can significantly lower the barrier for malicious attackers looking to design kernel exploits as it removes the need for 

## **REFERENCES** 

- [1] 2021. _Function Granular KASLR_ . https://lore.kernel.org/all/20211223002209. 1092165-1-alexandr.lobakin@intel.com/ 

- [2] 2023. _Linux source code (v6.0)_ . https://elixir.bootlin.com/linux/v6.0/source 

- [3] 2023. _Virtual Memory Map_ . https://www.kernel.org/doc/Documentation/x86/ x86_64/mm.txt 

- [4] Kristen Accardi. 2020. _Function-Granular KASLR_ . https://lwn.net/Articles/ 824307/ 

- [5] bcoles. 2023. _KASLD_ . https://github.com/bcoles/kasld 

- [6] Claudio Canella, Michael Schwarz, Martin Haubenwallner, Martin Schwarzl, and Daniel Gruss. 2020. KASLR: Break It, Fix It, Repeat. In _Proceedings of the 15th ACM Asia Conference on Computer and Communications Security_ (Taipei, Taiwan) _(ASIA CCS 20)_ . Association for Computing Machinery, New York, NY, USA, 481493. https://doi.org/10.1145/3320269.3384747 

- [7] Jonathan Corbet. 2017. _KAISER: hiding the kernel from user space_ . https://lwn. net/Articles/738975/ 

- [8] Intel Corporation. 2023. _Intel Software Security Guidance._ https: //www.intel.com/content/www/us/en/developer/topic-technology/softwaresecurity-guidance/processors-affected-consolidated-product-cpu-model.html 

- [9] Jack Dates. 2022. _The LDT, a Perfect Home for All Your Kernel Payloads_ . https: //blog.ret2.io/2022/08/17/macos-dblmap-kernel-exploitation/ 

- [10] Nico Economou. 2020. _Meltdown Reloaded: Breaking Windows KASLR by Leaking KVA Shadow Mappings_ . https://labs.bluefrostsecurity.de/blog/2020/06/30/ meltdown-reloaded-breaking-windows-kaslr/ 

- [11] Jake Edge. 2013. _Kernel address space layout randomization_ . https://lwn.net/ Articles/569635/ 

- [12] Ben Gras, Kaveh Razavi, Herbert Bos, and Cristiano Giuffrida. 2018. Translation Leak-aside Buffer: Defeating Cache Side-channel Protections with TLB Attacks. In _27th USENIX Security Symposium (USENIX Security 18)_ . USENIX Association, Baltimore, MD, 955972. https://www.usenix.org/conference/usenixsecurity18/ presentation/gras 

- [13] Daniel Gruss, Moritz Lipp, Michael Schwarz, Richard Fellner, Clmentine Maurice, and Stefan Mangard. 2017. KASLR is Dead: Long Live KASLR. 161176. https: //doi.org/10.1007/978-3-319-62105-0_11 

- [14] Daniel Gruss, Clmentine Maurice, Anders Fogh, Moritz Lipp, and Stefan Mangard. 2016. Prefetch Side-Channel Attacks: Bypassing SMAP and Kernel ASLR. In _Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ (Vienna, Austria) _(CCS 16)_ . Association for Computing Machinery, New York, NY, USA, 368379. https://doi.org/10.1145/2976749.2978356 

- [15] Ralf Hund, Carsten Willems, and Thorsten Holz. 2013. Practical Timing Side Channel Attacks against Kernel Space ASLR. In _2013 IEEE Symposium on Security and Privacy_ . 191205. https://doi.org/10.1109/SP.2013.23 

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

1. The passing of time; 

2. The tampering of facets of traces; 

3. The contamination of traces; and 

4. Insufficient knowledge of event traces 

The first point refers to the influence of time passing upon the facets of a trace remaining in the environment. Since an investigation starts after the event of interest occurred, it implies that a certain period passed between the time the event took place and the fixation of the crime scene. During this interval, the natural course of happenings (intrinsic events) impacts our ability to observe the facets resulting from the earlier event [8] (Gruber et al. [9] refer to this phenomenon as evidence dynamics). Intrinsic events may directly affect facets that can be observed, e.g., rolling logs overwriting facets; or indirectly, e.g., by affecting the environment in which they reside such as clocks drifting over time [10]. 

Tampering affects the facets of traces that were caused by events or other related events. To hide activities, an attacker or software (e.g., malware) may directly tamper with facets of traces, e.g., manipulating file timestamps or erasing files; or indirectly tamper with them by targeting the entire system (environment), e.g., by changing the system clock or attacking the Network Time Protocol (NTP). Note that tampering may also occur during the reconstruction process if the chain of custody is not correctly maintained, but this will not be our focus here. Palmbach and Breitinger [11] discussed various sources that can be found on the Windows operating system and concluded that none of the sources are reliable as they can be modified or deleted. Consequently, an (advanced) attacker may tamper with facets of traces which may lead to incomplete or out-of-order timelines. Eventually, this will impact the hypothesis creation of the investigator and in the worst case may lead to misinterpretations. 

Other subsequent events may also produce modifications of the environment, e.g., additional facets, after the initiation of the investigation process. Gruber et al. [9] refer to this phenomenon as the contamination of digital evidence (facets of traces in the newer terminology) and define it as being any inadvertent transfer of traits to an object of relevance at any point of the forensic process. An example of contamination, as given by the authors, may be failing to boot into a forensic live distribution at the acquisition stage, which would affect many timestamps. 

Finally, insufficient knowledge of traces from events is a challenge in multiple ways. Firstly, it is necessary to have experimental data to be able to generate a hypothesis as to what a pattern of timeline entries may suggest. There is then the problem that previous knowledge of traces generated by an event may now be incorrect on a new operating system, or even a new version of an application. For example, well-known references such as the SANS poster on NTFS time rules<sup>2</sup> may be outdated and may not consider application-specific deviations [12]. This can to some extent be mitigated through specific experiments related to the event that is trying to be inferred, but 

> 2https://www.sans.org/posters/windows-forensic-analysis/ 

2 

this is resource-intensive. However, additional limitations in knowledge will reduce the investigators ability to generate viable alternative hypotheses that would produce the same pattern of observed facets, and that cannot be so easily addressed via experiments. 

Overall, there are several problems to consider during the event reconstruction process, particularly when evaluating facets of traces in light of a certain number of hypotheses related to the event. While these problems may be well understood by some digital forensics practitioners and academics, their impact on the interpretation/reconstruction process is rarely discussed in published research. 

## 4. Towards assessing the tamper resistance of sources 

In addressing the tampering problem, research studies focused on the aspect of detection, i.e., through which means tampering of sources of facets may be detected by an investigator. Examples would be studies by [13] or [14] who concentrated on detecting timestamp manipulations. 

Palmbach and Breitinger [11] present an extended range of sources of facets that may be used to detect tampering of timestamps on Windows: the $USNjrnl, Prefetch and LNK files, Windows event logs, and the $LogFile. While these sources may contain substantial information, they may also be manipulated or deleted. In addition, these sources may be limited (e.g., fixed size) or unavailable on the system under investigation. 

In such situations, more niche techniques like digital stratigraphy, time anchors or hyper timelines may be used to detect time inconsistencies, e.g., backdating. Digital stratigraphy, as defined by Casey [15], is a method that takes advantage of the knowledge of file systems and the functioning of their allocation algorithms. By analysing the logical arrangement of files on a disk (e.g., as demonstrated in [16]), investigators can infer hypothetical events, provided they understand how the file system allocates and organizes those files. For instance, with the next fit strategy, a file created before another is usually positioned at a lower logical address on the disk than the latter. Thus, a file incorrectly ordered according to the functioning of the allocation algorithm may indicate that it was backdated [15, 17, 18]. Another concept is time anchors as discussed by [19] where an anchor is an artefact that contains two timestamps  one originating from the internal clock and one from an external source. If these timestamps do not match, this indicates tampering. A third approach is the concept of hyper timelines which focus on events without timestamps and focus on implicit timing information [20]. Inconsistency in implicit and exploit time information may also indicate tampering. For example, text-based log files are typically ordered by timestamp. If a timestamp is altered without the corresponding line being moved to maintain the correct order, it may indicate tampering. 

Experiments show that the probability of detecting tampering is high, especially when it concerns file metadata. For instance, it may be difficult for an attacker to forge a timestamp without causing subsequent inconsistencies that should be fixed as well [21, 22]. Similarly, Galhuber and Luh [12] highlight that timestamp forgery tools may modify timestamps in a detectable 

way, e.g., changing the accuracy of timestamps from nanoseconds to seconds. Additionally, only one of the tools they evaluated can modify the entire range of file system timestamps on Windows thus reducing the risks of being detected. On top of that, Andrade [23] highlights that the $FN timestamps are only modified by the Windows kernel and will generally go untouched by antiforensic timestomping tools. This provides another specific example where, during event reconstruction, one timestamp is more difficult to manipulate than another. 

Although there has been work on tampering in digital forensic investigations, none specifically relates to the general problem of event reconstruction. In addition, Neale [24] stresses the need to consider the reliability of facets, specifically in the context of tampering, and highlights that current research did not provide yet effective means to identify tampering. This paper aims to bridge that gap and explores the factors that could be used to formally evaluate the resistance to tampering. 

## 5. Factors 

From an examiners perspective, knowing about the resilience of sources of facets is essential when forming the hypothesis. Our results outline several factors that should be considered when assessing the resistance of sources. As this is a time-consuming process, it likely cannot be done for all sources but should at least be done for the most critical ones. The outcome of this process is a confidence. 

## 5.1. Factors to consider in evaluating tamper resistance 

We consider that each source of an observed facet (in our case sources of timeline entries) has a series of properties or factors that affect how resistant it is to active tampering. This is a combination of its intrinsic nature, e.g., the operating system or settings, in which the source resides. After reviewing sources of facets used for event reconstruction, seven common factors have been identified which are presented in the upcoming subsections. Each subsection includes a description, examples, and concludes with several categories within that particular factor. This is not exhaustive but has provided useful insight for the case study examples in Sec. 6. Future work may further refine these factors as more edge-case examples are identified. 

The use of these factors is a general concept. However, as will be seen later, in their current form as intentionally simple tables, they lend themselves to trying to improve the quality of results in mainstream investigations that do not have highly advanced attackers such as Advanced Persistent Threats (APTs). For these more challenging investigations, the factors presented are still relevant, but more resolution in the scoring would be needed, e.g., around advanced command line usage, and perhaps multiple iterations of evaluating tamper resistance. As an example, to score the presence of software that could be used for tampering, scoring is needed for the artefacts used to infer the presence of software. 

There are seven factors identified that can be considered when evaluating the resistance of a source to active tampering: Viability of source to user; permissions; software to edit 

3 

on system; observed facets of source access; encryption; and file format of the source; organization of the source. 

## 5.1.1. Visibility of the source to user 

This factor considers whether the source containing the digital forensic facet is visible to the user. For example, within Windows, a file on the users desktop (subject to custom hidden flags being set) is visible to a user. However, the file desktop.ini in the same location is not, which while not highly relevant for forensic investigations illustrates the example. Through a change within the Windows Explorer settings, this file can be made visible<sup>3</sup> . Therefore, two other categories of visibility are needed: Visible via user setting change (enabled) and Visible via user setting change (not enabled). There is also the possibility of viewing files via alternative mechanisms such as the terminal (ls -a) or Powershell (ls -Hidden) which also needs to be captured. 

There are additional challenges in evaluating this particular factor. For example, if we consider the Google Chrome Preferences file, which is normally located in the users home directory<sup>4</sup> . The folder AppData is by default not visible to a user, but the contents are visible (and also editable) via the Google Chrome interface, and therefore we should consider this trace to be user visible. However, the newer Firefox cache, again in a similar location<sup>5</sup> , is not visible by default. While Firefox itself does offer an interface to the cache via the UI (about:cache in the address bar), it provides no mechanism to edit the data. As a result, it should not be considered user visible. 

This leads us to several user visibility categories into which a source could fall: 

User visible via GUI 

User visible via other UI method (e.g., terminal) Visible via user setting change (enabled) Visible via user setting change (not enabled) Cannot be made visible 

## 5.1.2. Permissions 

Another factor to consider is whether permission exists to modify the source. In many systems, some users may have access to everything. Therefore, this consideration should be specific to a particular user and re-evaluated for different users. For example, on a Windows system, various operations are protected via User Account Control (UAC). The privileges to modify one source over another could be different. Again, taking the Chrome Preferences example, a user can access it without triggering the UAC interface. However, regarding protected files, on many system configurations the user is an Administrator on the system in question. Therefore, the UAC interface cannot be considered a barrier to accessing protected files as it only requires clicking Allow. On domain systems, however, the user of 

> 3Show hidden files, folders and drives, and Hide protected operating sys- 

> 4In .../AppData/Local/Google/Chrome/User Data/Default 

> 5In .../AppData/Local/Mozilla/Firefox/Profiles/xxx.defaultrelease 

a system may not be an administrator of the system and therefore, in the context of that user, manipulation of a privileged protected source should be considered as protected, e.g., access to another users Chrome preferences file. 

In specific, more advanced investigations involving intrusions on a system, another consideration is whether there are observed facets of privilege escalation (see Sec. 5.1.4). Suppose facets are found suggesting the use of an exploit to elevate a standard users privileges. In that case, files previously considered user inaccessible should be considered as if they are user accessible. 

If we consider Plaso parsers, we see a difficulty in applying the permissions factor generically. For example, for the PE parser for portable executables, some will be in user accessible locations, while others will not. Therefore, many sources will need to be evaluated on a source-by-source basis. However, for some that are in one protected location, e.g., prefetch files, the data extracted could be considered to all have the same property on a particular system. 

This leads to the following permission categories: 

User accessible 

User accessible with prompt User accessible with password / biometrics User inaccessible, but observed facets of privilege escalation User inaccessible 

## 5.1.3. Software to edit on system 

This factor addresses the ease by which a manipulation can be made. As direct manipulation of the physical representation of binary data is not possible, at some level of abstraction, a tool will be used to facilitate manipulations. Initially, this was considered from the perspective of whether a tool was available at all that would allow manipulation. However, this becomes very time-sensitive, e.g., a tampering tool release requires an update of all evaluations and is also difficult to provide a comprehensive answer for all possible file types. Therefore, the most sensible approach was to provide a context-sensitive evaluation of this, i.e., whether there is software on the system in question capable of modifying the trace, or observed facets of the use of such software. 

For example, on a Windows system, RegEdit is available, and therefore, ignoring all other factors in this section such as permissions, an adversary can use it to modify keys and values in the Registry. In contrast, a SQLite database cannot reasonably be modified on a system without third-party software or Powershell library (and installation of either could leave traces). 

Another software option is the presence of a hex editor where all files should be considered as editable. In this case, the relative complexity of making such a modification will be captured via other factors as discussed later. 

There are also some edge cases. For example for Windows event logs, the tool Event Viewer is built into Windows. However, this tool only provides the ability to read event logs and does not provide edit capabilities (other than clearing logs). A default tool for editing event logs on Windows should therefore 

4 

be considered in the category of Not on the system. There are also edge cases regarding the Windows Registry. While, as discussed above, Regedit could be used to change data in keys and values, it does not provide access to the last modified timestamp of a key often used in event reconstruction. Therefore, whether a tool is considered available or not, is dependent on the specific facet and what it is being used for. This is by design and will become clear in the examples in Sec. 6. 

A final example of the situation being important would be tampering with a value in the MountedDevices key. Here, Regedit can be used to access that registry key, but the drive letter values are the REG BINARY type, which Regedit will not provide an easy user interface method of tampering, but will provide a view that allows the hex to be manually edited. Therefore for this particular trace a summary of Tool available by default for low-level (hex) editing is the most appropriate. 

In summary, the following editing software categories exist: 

Tool available by default for UI-based editing* Tool added to this system for UI-based editing* Tool available by default for low-level (hex) editing Tool added to this system for low-level (hex) editing Not on the system 

However, the database that stores the data is encrypted using SQLCipher4 [25]. The key is available on the system in the config.json file so the database would still be accessible via an SQLbrowser supporting that encryption method. This could be classed as encrypted, but key recoverable is possible from local system. 

Another complexity within the encryption attribute is the different categories of encryption software implementation. In Hargreaves [26] the difference between file-based, file systembased, container-based and full disk encryption is described. An encrypted single file or an encrypted container, where it is not known whether a password is available to the attacker, should be initially considered as one of the last threee Encrypted variants shown below depending on the specific nature of the key/password storage. In contrast, the decryption of filebased (e.g. EFS) or full disk (e.g. Bitlocker) when the system is running means that all data is accessible to the system and the encryption may not be relevant for the accessibility of the data contained therein (subject to other permissions and exact abstraction layer of the tampering attempt). 

This leads to the following encryption categories: 

No encryption 

Encrypted but accessible live (e.g., EFS) 

*UI is used rather than GUI as manipulation tools may be a command line 

## 5.1.4. Observed facets of access 

In addition to considering if there is software on a system capable of accessing a source, it is also important to determine if there are observed facets of actual access to that source. For SQLite database viewers, the recent files list associated with the program may provide evidence of a specific database being accessed. In some cases, this information might be even more detailed; for instance, the Registry key /NTUSER.DAT/Software/Microsoft/Windows/CurrentVe rsion/Applets/RegEdit could indicate that a specific key was accessed using RegEdit. However, there may also be scenarios where no explicit traces of a relevant source being accessed are observable, but evidence shows that the associated program was executed. We define the following observed facets of access categories: 

Observed facets of edit-capable software accessing the specific source 

Observed facets of edit-capable software accessing the source Observed facets of edit-capable software being run No observed facets of source access 

Encrypted (trivial to break) e.g., ROT13 in Windows Registry Encrypted (key recovery possible from local system) Encrypted (key stored off device available to user) Encrypted (key stored off device not available to user) 

## 5.1.6. (File) Format 

The format of a source also impacts its resilience. There are many different (file) formats, and it is impossible to try and list them all here. However, given that in Sec. 5.1.3 we consider hex editors as software that allows editing of a source, it is important to capture the complexity of making such manual edits. We identified some broad categories: Sources could be plain text which would be easy to modify, they could be a structured but still text-based format such as XML or JSON, or they could be a binary format, which may be proprietary, proprietary but reverse engineered, or an open format. We also include a NA category here, to be used when a source is considered where a user interface tool is available since the low-level format becomes irrelevant at that point. 

The (file) format categories can be summarized as follows: 

Binary proprietary (currently unknown) Binary proprietary but reverse-engineered (e.g., MFT) Binary open format (e.g., SQLite) 

Text-based machine format (e.g., XML, JSON) Plain text 

## 5.1.5. Encryption 

NA (GUI edit tool available) 

Another consideration is if the source in question is encrypted. This could be argued as simply an enforcement of permissions, but there are some situations where this is not the same. For example, consider the messaging app Signal, paired with a Windows desktop computer. Here, the files are stored within a users home folder so they could have access to them. 

## 5.1.7. Organization of the source 

The organization of data within a source (structured, semistructured, or unstructured) is another factor impacting its resilience. Generally, more organized structures allow easier au- 

5 

tomation of manipulations (which then allows mass manipulation). As examples, structured data can often be accessed with tools and a potential manipulation is scriptable. For instance, it is possible to develop a Python script that scans for JPG files and manipulates the EXIF information in the header. In contrast, removing a watermark within an image<sup>6</sup> requires utilising artificial intelligence (more processing power) or manual work. Consequently, this factor considers indirectly how difficult it is to automate the manipulation of the contents of a source. Categories would therefore include: 

Structured - timestamp within a known data structure, e.g., MFT Semi-structured  a timestamp that is stored as a field within JSON but as a text string, e.g., Wed 25th Jan 2022 11:35 am Unstructured - within a Word document, within the content itself the author has made reference to a date and time of an event 

## 5.2. Scoring 

The previous section has suggested seven factors, all of which have been argued to affect the extent to which data could be tampered with. Each factor has several options or categories that allow the properties of a specific source or facet within a source to be evaluated and qualitatively described. That level of granularity is sometimes required, for example, one key within a Windows registry may have different properties to another, e.g., the user autorun keys vs the contents of keys in the SAM file, the latter being inaccessible via Regedit to even admin users on a live machine [27]. 

Let us now consider an event reconstruction that relies on some observed facets. If we can assign scores to each of the categories within each of the factors used to describe the source of a facet, then we can use this to begin to evaluate the reliability of an event reconstruction from a tamper resistance perspective. At present, there is no meaningful absolute score that could be assigned to those categories, nor data on the relative importance of each category. However, in other areas, quantitative measures are used which are broader values and are used to rate one situation over another. This is used as inspiration here. For the scoring, we decided to borrow concepts from security risk assessment [28] where the determination of risk can be seen as a function of harm and the likelihood of its occurrence. As the harm is difficult to predict, we use it to express the tampering concern of the source from that factors perspective (the tampering concern is the inverse of tampering resistance). Given a source (e.g., Windows Registry) and a factor (e.g., software to edit), we define three degrees of severity: 

- high (3) means that there is the highest tampering concern of the source from that factors perspective 

- moderate (2) means that there is a moderate tampering concern. 

## low (1) means that there is a low tampering concern. 

> 6Note that this kind of source is currently not considered by timeline generation software. 

We then looked at each factor for each category and assigned a severity where a higher number means that manipulating a source is easier. For instance, the category Cannot be made visible in the user visibility factor received a low score (1). Consequently, if a source is assigned this category, the tampering concern is low / the tampering resistance is high. Even though some categories have received the same severity, we keep the qualitative descriptions separate to facilitate a more granular analysis in the future and to provide provenance as to why a source has been given a particular rating. 

An important note is that each factor is independent of one other so a 3 in user visibility is not equivalent to a 3 in permissions. This means that at this stage a meaningful computed sum is not possible, but it does mean that sources with particular weaknesses can be easily identified. It is crucial to emphasize that evaluating the tamper resistance of the sources used for event reconstruction, as discussed above, is just as important as the numerical scores themselves. This consideration should carry significant weight when contributing to a C-Score assessment. 

## 6. Case Study examples 

We can now consider some examples taking into account the factors and scores. For the production of these examples, a template spreadsheet has been created that captures the factors discussed earlier and the available options are in a dropdown menu, and from that, a color-coded score is displayed. The template is available<sup>7</sup> and can be used to review an event reconstruction. The tables are used in these examples and could be used by an investigator to assist in structuring an assessment of tamper resistance of sources. We also consider them to be a step towards a more automated analysis in the future where some fields could be automatically populated. 

To perform a review of an event reconstruction, the expected facets that result from an event need to be known. This can be achieved either through existing documented forensic research, via live logging tools such as Procmon, or using timeline generation software such as Plaso. The relevant observed facets need to be identified and the source from which they are extracted determined. Then the tampering concern of the source can be reviewed and classified according to the factors discussed above. 

A simple example is provided on file creation on NTFS (see Sec. 6.1), and an extended example with multiple variations is given on USB device connection on Windows (see Sec. 6.2). Note, that the goal of this section is not to explore exhaustively each factor category but to improve the understanding of their usability in evaluating the tamper resistance of sources. 

## 6.1. File Creation reconstruction when timestomp is present 

As a first simple example, to illustrate the process, we con- 

> 7The template and examples are available via Google Sheets. To use it, open the link and create a copy (file > duplicate): https://docs.google.com/spreadsheets/d/1DnfYMtprmzp3dGt9SxRo2Jb83ruZHdRMStFz3PzZQ8/ 

6 

|n|Factors|Category<br>Score<br>SI attribute|n|Factors|Category<br>Windows/INF/setupapi.dev.log|Score|
|---|---|---|---|---|---|---|
|1|User visible|Cannot be made visible<br>1|1|User visible|User visible via GUI|3|
|2|Permissions|User inaccessible<br>1|2|Permissions|User accessible|3|
|3|Software to edit|Tool added to this system for UI-based editing<br>3|3|Software to edit|Tool available by default for UI-based editing|3|
|4<br>|Facets of access<br>|Observed facets of edit-capable software be-<br>ing run<br>2<br>|4<br>|Facets of access<br>|Observed facets of edit-capable software be-<br>ing run<br>|2|
|5|Encryption|No encryption<br>3|5|Encryption|No encryption|3|
|6|File format|NA (UI edit tool available)<br>3|6|File format|Plain text|3|
|7|Structural|Structured<br>2|7|Structural|Structured|2|
|||FN attribute||Syst|em/ControlSetxxx/Enum/USBSTOR/||
|8|User visible|Cannot be made visible<br>1|8|User visible|User visible via GUI|3|
|9|Permissions|User inaccessible<br>1|9|Permissions|User accessible with prompt|3|
|10|Software to edit|Not on the system<br>1|10|Software to edit|Tool available by default for UI-based editing|3|
|11|Facets of access|No observed facets of source access<br>1|11|Facets of access|No observed facets of source access|1|
|12|Encryption|No encryption<br>3|12|Encryption|No encryption|3|
|13|File format|Binary proprietary but reverse engineered<br>2|13|File format|NA (UI edit tool available)|3|
|14|Structural|Structured<br>2|14|Structural|Structured|2|
||||||System/MountedDevice||
|ble 1<br>e pre|: Computed severi<br>sence of timestomp|ty for two creation timestamps within the MFT given<br>.|15<br>16<br>17<br>18<br>19|User visible<br>Permissions<br>Software to edit<br>Facets of access<br>Encryption|User visible via GUI<br>User accessible with prompt<br>Tool available by default for UI-based editing<br>No observed facets of source access<br>No encryption|3<br>3<br>3<br>1<br>3|
|ut u|se the example|from Andrade [23] referenced earlier re-|20<br>|File format<br>|Binary proprietary but reverse engineered<br>|2<br>|
||||21|Structural|Structured|2|
|ardi|ng timestomp.|ooking at just the two creation timestamps|||Event Logs||
|ithi<br>b-t|n an MFT entry<br>ables, one for e|, we create Table1which consists of two<br>ach facet (the MFT SI and FN attributes).|22<br>23<br>24|User visible<br>Permissions<br>Software to edit|User visible via GUI<br>User accessible<br>Not on the system|3<br>3<br>1|
|n ro<br>ecau<br>utn|ws 3 and 10 on<br>se timestomp i<br>ottheFNattri|e can see the two diferent scores assigned<br>s capable of modifying the SIA attribute<br>uteThisthencascadesintotheflefor-|25<br>26<br>27<br>28|Facets of access<br>Encryption<br>File format<br>Structural|<br>No observed facets of source access<br>No encryption<br>Binary proprietary but reverse engineered<br>Semi-structured|1<br>3<br>2<br>2|

<!-- Start of picture text -->
SI attribute<br>1 User visible Cannot be made visible 1<br>2 Permissions User inaccessible 1<br>3 Software to edit Tool added to this system for UI-based editing 3<br>4 Facets of access Observed facets of edit-capable software be- 2<br>ing run<br>5 Encryption No encryption 3<br>6 File format NA (UI edit tool available) 3<br>7 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
Windows/INF/setupapi.dev.log<br>1 User visible User visible via GUI 3<br>2 Permissions User accessible 3<br>3 Software to edit Tool available by default for UI-based editing 3<br>4 Facets of access Observed facets of edit-capable software be- 2<br>ing run<br>5 Encryption No encryption 3<br>6 File format Plain text 3<br>7 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
FN attribute<br>8 User visible Cannot be made visible 1<br>9 Permissions User inaccessible 1<br>10 Software to edit Not on the system 1<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/ControlSetxxx/Enum/ USBSTOR/<br>8 User visible User visible via GUI 3<br>9 Permissions User accessible with prompt 3<br>10 Software to edit Tool available by default for UI-based editing 3<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format NA (UI edit tool available) 3<br>14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/MountedDevice<br>15 User visible User visible via GUI 3<br>16 Permissions User accessible with prompt 3<br>17 Software to edit Tool available by default for UI-based editing 3<br>18 Facets of access No observed facets of source access 1<br>19 Encryption No encryption 3<br>20 File format Binary proprietary but reverse engineered 2<br>21 Structural Structured 2<br><!-- End of picture text -->

Table 1: Computed severity for two creation timestamps within the MFT given the presence of timestomp. 

but use the example from Andrade [23] referenced earlier regarding timestomp. Looking at just the two creation timestamps within an MFT entry, we create Table 1 which consists of two sub-tables, one for each facet (the MFT SI and FN attributes). On rows 3 and 10 one can see the two different scores assigned because timestomp is capable of modifying the SIA attribute but not the FN attribute. This then cascades into the file format information which is not relevant when a tool is available in SIA, but is relevant for the FN. We conclude from this that in this context, the timestamp in the FN attribute is more tamper resistant than the SIA attribute, which aligns with the intuitive findings in [23] . An important highlight here is that the granularity of source in this case needs to be at the resolution of MFT attributes, one for the SIA, and one for the FN attribute, since they have different properties. 

<!-- Start of picture text -->
Event Logs<br>22 User visible User visible via GUI 3<br>23 Permissions User accessible 3<br>24 Software to edit Not on the system 1<br>25 Facets of access No observed facets of source access 1<br>26 Encryption No encryption 3<br>27 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
28 Structural Semi-structured 2<br><!-- End of picture text -->

Table 2: Computed severity for four sources used for event reconstruction of USB device connection. 

|n|Factors|Category|Score|
|---|---|---|---|
|||Registry within Shadow Copy||
|1|User visible|Visible via user setting change (not enabled)|1|
|2|Permissions|User accessible with prompt|3|
|3|Software to edit|Not on the system|1|
|4|Facets of access|No observed facets of source access|1|
|5|Encryption|No encryption|3|
|6|File format|Binary proprietary but reverse engineered|3|
|7|Structural|Structured|2|

<!-- Start of picture text -->
Registry within Shadow Copy<br>1 User visible Visible via user setting change (not enabled) 1<br>2 Permissions User accessible with prompt 3<br>3 Software to edit Not on the system 1<br>4 Facets of access No observed facets of source access 1<br>5 Encryption No encryption 3<br>6 File format Binary proprietary but reverse engineered 3<br><!-- End of picture text -->

<!-- Start of picture text -->
7 Structural Structured 2<br><!-- End of picture text -->

## 6.2. USB Device Connection 

Table 3: Computed severity of a Registry within a Windows Shadow Copy 

For the second example, we consider the connection of a USB device on Windows as summarized in Table 2. There are several known locations where modifications are made (setupapi.dev.log, Windows Registry, Event Logs). 

## 6.3. Discussion 

These examples illustrate the difference in the tamper resistance of different sources that are frequently used for event reconstruction. The factors that have been identified have been argued to have an effect on tamper resistance. While there may be additional factors or categories within those factors, this can easily be accommodated. In particular, new categories can be trivially added and appropriate scores assigned depending on if it is easier or harder to tamper with a source with those properties. 

One can see that given some specific conditions on this particular system: notepad has been run but no reference to setupapi.dev.log (row 4), and there are no observed facets of Regedit running on this system (rows 11, 18)). As a direct comparison between the sources, the Windows event logs should be considered the most difficult to tamper with from the set. Thus, if there were conflicting timestamps, from a tampering perspective only, and in the absence of other information, the times in the event logs should be considered the most reliable of the set. 

The examples demonstrated the use for some Windows event reconstructions, but the proposed categories are platformindependent. The examples did not consider external sources of facets, e.g., network server logs, although we acknowledge their importance. For brevity, this paper focuses on single evidence items which allows clarity in terms of the intrinsic resilience of individual sets of local traces. 

An extension of this would be if older copies of the Windows Registry were available within Shadow Copies as illustrated in Table 3. These can be accessed via the command line mounting of restore points (vssadmin) [29]. For that data, the tamper resistance score changes since they are not directly accessible without mounting shadow copies and Regedit cannot directly access those versions of the registry. 

In addition, Table. 4 shows that the difference in tamper resistance is significantly different when a corporate system is considered and the end-user accessibility of many of the sources is reduced compared with Table. 2. 

It has also been shown how factors can be risk scored to highlight differences between sources that are used for event reconstruction. A simplistic high, moderate, and low system has been chosen to provide an easy evaluation of sources, which given 

7 

<!-- Start of picture text -->
Windows/INF/setupapi.dev.log<br>1 User visible Cannot be made visible 1<br>2 Permissions User inaccessible 1<br>3 Software to edit Not on the system 1<br>4 Facets of access No observed facets of source access 1<br>5 Encryption No encryption 3<br>6 File format Plain text 3<br>7 Structural Semi-structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/ControlSetxxx/Enum/ USBSTOR/<br>8 User visible Cannot be made visible 1<br>9 Permissions User inaccessible 1<br>10 Software to edit Not on the system 1<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format Binary proprietary but reverse engineered 2<br>14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
Event Logs<br>15 User visible Cannot be made visible 1<br>16 Permissions User inaccessible 1<br>17 Software to edit Not on the system 1<br>18 Facets of access No observed facets of source access 1<br>19 Encryption No encryption 3<br>20 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
21 Structural Semi-structured 2<br><!-- End of picture text -->

cases where they differ. It can be used to express the tamper resistance of specific sources or used conjointly with the C-Scale [5] to help assess the tamper resistance of sources when expressing the strength of observed facets in light of competing hypotheses (e.g., understanding and evaluating indicators such as in C-value C2 only one source of evidence that is not difficult to tamper with). Whilst the scoring system is intended to remain partly subjective (to retain some insights into an investigators reasoning process), we imagine it being implemented in software such as Plaso to give an indicator of the tamper resistance of sources in timelines (if prepopulating the scores is feasible, which will part of future work). Finally, the scoring system has educational applications for raising awareness of the risks of relying on observed facets for event reconstruction and encouraging critical questioning of their reliability. 

## 7. Conclusions and further work 

Table 4: Computed severity for three sources when a corporate system is considered. 

that this is currently necessarily a manual process seems an appropriate level of granularity. 

At present manual scoring of sources is necessary, and many are situation/environment dependent, e.g., is the user admin or not. Some could be standardized, for example, if the machine is Windows and the user is the only account and therefore admin, then the Registry is always accessible (clicking through the UAC prompt) and Regedit will always be available. This offers the potential for prepopulating some of the scores. At present, we only offer a manual process, which is more timeconsuming but does encourage deeper thought about the nature of the sources being used for event reconstruction. It is also necessary to have a good knowledge of system behaviour to understand what a user can and cannot access on a system, plus details such as the binary nature of sources. However, we argue that a requirement of detailed knowledge of digital systems and their behaviour should not be an onerous requirement to improve the reliability of event reconstruction. 

It should also be stressed that due to the detailed nature of this process, this could be performed only for specific examples where the event reconstruction that being performed is critical to a case and offers an opportunity to further improve reliability and provides some defence to questions about whether tampering could have occurred; e.g., that an assessment was performed according to a structured framework. This is most value when used in a targetted manner e.g., when there are conflicting timestamps identified in an event reconstruction. However, it could also be undertaken less regularly using a dip sample method as part of a general quality assurance assessment of processes. 

In the future, with a programmatic assessment of these factors affecting reliability, the manual overhead does become negligible (potentially with the source knowledge preprogrammed), and also a more detailed quantitative measure could be used as a replacement for the simplistic high/moderate/low options. The proposed score can assist investigators in reconstructing events when multiple sources are not available or in 

Event reconstruction is a critical part of the digital forensic process. We introduced a process for reviewing and scoring sources contributing to timeline-based event reconstruction. Our analysis revealed that some commonly relied-upon facets, such as USB device connections, may not be as resistant to tampering as often assumed. While this does not preclude their use, it underscores the importance of understanding their limitations. 

The primary takeaway from this work is the importance of considering tamper resistance when reconstructing events. Although our proposed factors and scoring system represent an initial framework, future iterations and refinements will enhance its applicability, especially as edge cases emerge. The general principle of evaluating the tamper resistance of traces will be invaluable for improving the reliability of event reconstruction, handling uncertainty, and reducing errors in the process. 

Finally, while some of these identified factors will be obvious to seasoned investigators, and many will understand the reliability issues associated with certain sources, there is a clear need within what is now referred to as digital forensic science, to formalize definitions and make explicit that which is currently tacit. Furthermore, providing categories within these influencing factors, and including concrete examples of how they can be used provides the foundation for more formal and potentially future quantitative evaluation of the trustworthiness or indeed reliability of reconstructed events in a digital forensic investigation. 

## References 

- [1] C. Roux, R. Bucht, F. Crispino, P. De Forest, C. Lennard, P. Margot, M. D. Miranda, N. NicDaeid, O. Ribaux, A. Ross, S. Willis, The Sydney declaration  Revisiting the essence of forensic science through its fundamental principles, Forensic Science International 332 (2022) 111182. doi:10.1016/j.forsciint.2022.111182. 

- [2] C. Hargreaves, F. Breitinger, L. Dowthwaite, H. Webb, M. Scanlon, Dfpulse: The 2024 digital forensic practitioner survey, Forensic Science International: Digital Investigation 51 (2024) 301844. 

8 

- [3] H. Studiawan, F. Sohel, C. Payne, Sentiment Analysis in a Forensic Timeline With Deep Learning, IEEE Access 8 (2020) 6066460675. doi:10.1109/ACCESS.2020.2983435 , conference Name: IEEE Access. 

- [4] C. Hargreaves, J. Patterson, An automated timeline reconstruction approach for digital forensic investigations, Digital Investigation 9 (2012) S69S79. URL: https://www.sciencedirect. com/science/article/pii/S174228761200031X. doi:10.1016/j.diin.2012.05.006. 

- [5] E. Casey, Standardization of forming and expressing preliminary evaluative opinions on digital evidence, Forensic Science International: Digital Investigation 32 (2020) 200888. URL: https://www.sciencedirect. com/science/article/pii/S1742287619303147. doi:https://doi.org/10.1016/j.fsidi.2019.200888. 

- [6] C. Hargreaves, A. Nelson, E. Casey, An abstract model for digital forensic analysis tools-a foundation for systematic error mitigation analysis, Forensic Science International: Digital Investigation 48 (2024) 301679. 

- [7] World Anti-Doping Agency, Cas 2020/o/6689 world anti-doping agency v. russian anti-doping agency, https://www.tas-cas.org/ fileadmin/user_upload/CAS_Award_6689.pdf, 2020. 

- [8] D.-O. Jaquet-Chiffelle, E. Casey, A formalized model of the Trace, Forensic Science International 327 (2021) 110941. doi:10.1016/j.forsciint.2021.110941 . 

- [9] J. Gruber, C. J. Hargreaves, F. C. Freiling, Contamination of digital evidence: Understanding an underexposed risk, Forensic Science International: Digital Investigation 44 (2023) 301501. URL: https://www.sciencedirect.com/science/article/pii/ S2666281723000021. doi:10.1016/j.fsidi.2023.301501 . 

- [10] M. W. Stevens, Unification of relative time frames for digital forensics, Digital Investigation 1 (2004) 225239. URL: https://www.sciencedirect.com/science/article/pii/ S174228760400057X. doi:10.1016/j.diin.2004.07.003. 

      - ternational: Digital Investigation 32 (2020) 300924. URL: https:// linkinghub.elsevier.com/retrieve/pii/S2666281720300196. doi:10.1016/j.fsidi.2020.300924. 

   - [22] J. Schneider, L. Dusel, B. Lorch, J. Drafz, F. Freiling, Prudent design principles for digital tampering experiments, Forensic Science International: Digital Investigation 40 (2022) 301334. URL: https:// linkinghub.elsevier.com/retrieve/pii/S2666281722000038. doi:10.1016/j.fsidi.2022.301334. 

   - [23] R. Andrade, Expose evidence of timestomping with the ntfs timestamp mismatch artifact, 2020. URL: https://www.magnetforensics. com/blog/expose-evidence-of-timestomping-with-thentfs-timestamp-mismatch-artifact-in-magnet-axiom-4-4/. 

   - [24] C. Neale, Fool me once: A systematic review of techniques to authenticate digital artefacts, Forensic Science International: Digital Investigation 45 (2023) 301516. URL: https://www.sciencedirect.com/science/article/pii/ S2666281723000173. doi:10.1016/j.fsidi.2023.301516. 

   - [25] A. Bilz, A forensic gold mine ii: Forensic analysis of signal messenger on windows 10, 2021. URL: https://www.alexbilz.com/post/202106-07-forensic-artifacts-signal-desktop/. 

   - [26] C. J. Hargreaves, Assessing the reliability of digital evidence from live investigations involving encryption., Ph.D. thesis, Cranfield University, UK, 2009. 

   - [27] S. Laiho, How to access the sam and security hives in the registry using the system account, 2020. URL: https://4sysops.com/ archives/how-to-access-the-sam-and-security-hives-inthe-registry-using-the-system-account/. 

   - [28] R. S. Ross, Guide for conducting risk assessments, National Institute of Standards and Technology (2012). 

   - [29] C. Hargreaves, H. Chivers, D. Titheridge, Windows vista and digital investigations, Digital Investigation 5 (2008) 3448. 

- [11] D. Palmbach, F. Breitinger, Artifacts for Detecting Timestamp Manipulation in NTFS on Windows and Their Reliability, Forensic Science International: Digital Investigation 32 (2020) 300920. doi:10.1016/j.fsidi.2020.300920 . 

- [12] M. Galhuber, R. Luh, Time for Truth: Forensic Analysis of NTFS Timestamps, in: Proceedings of the 16th International Conference on Availability, Reliability and Security, ARES 21, Association for Computing Machinery, New York, NY, USA, 2021, pp. 110. doi:10.1145/3465481.3470016 . 

- [13] J. Oh, S. Lee, H. Hwang, Forensic detection of timestamp manipulation for digital forensic investigation, IEEE Access (2024). 

- [14] D.-i. Jang, G.-J. Ahn, H. Hwang, K. Kim, Understanding anti-forensic techniques with timestamp manipulation, in: 2016 IEEE 17th International Conference on Information Reuse and Integration (IRI), IEEE, 2016, pp. 609614. 

- [15] E. Casey, Digital Stratigraphy: Contextual Analysis of File System Traces in Forensic Science, Journal of Forensic Sciences 63 (2018) 13831391. doi:10.1111/1556-4029.13722 , number: 5. 

- [16] J. Schneider, M. Eichhorn, L. M. Dreier, C. Hargreaves, Applying digital stratigraphy to the problem of recycled storage media, Forensic Science International: Digital Investigation 49 (2024) 301761. 

- [17] S. Y. Willassen, Finding Evidence of Antedating in Digital Investigations, in: 2008 Third International Conference on Availability, Reliability and Security, 2008, pp. 2632. doi:10.1109/ARES.2008.149. 

- [18] W.-h. Tse, Kenneth, Forensic analysis using FAT32 file cluster allocation patterns, Master of Philosophy, The University of Hong Kong, Pokfulam Road, Hong Kong SAR, 2011. URL: https://hdl. handle.net/10722/143258. doi:10.5353/th_b4660573, pages: 991032316259703414, b46605733. 

- [19] C. Vanini, C. J. Hargreaves, H. van Beek, F. Breitinger, Was the clock correct? exploring timestamp interpretation through time anchors for digital forensic event reconstruction, Forensic Science International: Digital Investigation 49 (2024) 301759. 

- [20] L. M. Dreier, C. Vanini, C. J. Hargreaves, F. Breitinger, F. Freiling, Beyond timestamps: Integrating implicit timing information into digital forensic timelines, Forensic Science International: Digital Investigation 49 (2024) 301755. 

- [21] J. Schneider, J. Wolf, F. Freiling, Tampering with Digital Evidence is Hard: The Case of Main Memory Images, Forensic Science In- 

9 

# SOURCE: 2504.18131v1.pdf

# SoK: Timeline based event reconstruction for digital forensics: Terminology, methodology, and current challenges 

Frank Breitinger<sup>a,</sup> , Hudan Studiawan<sup>b</sup> , Chris Hargreaves<sup>c</sup> 

_aInstitute of Computer Science, University of Augsburg, Augsburg, Germany bDepartment of Informatics, Institut Teknologi Sepuluh Nopember, Surabaya, Indonesia cDepartment of Computer Science, University of Oxford, Oxford, United Kingdom_ 

## **Abstract** 

Event reconstruction is a technique that examiners can use to attempt to infer past activities by analyzing digital artifacts. Despite its significance, the field suffers from fragmented research, with studies often focusing narrowly on aspects like timeline creation or tampering detection. This paper addresses the lack of a unified perspective by proposing a comprehensive framework for timelinebased event reconstruction, adapted from traditional forensic science models. We begin by harmonizing existing terminology and presenting a cohesive diagram that clarifies the relationships between key elements of the reconstruction process. Through a comprehensive literature survey, we classify and organize the main challenges, extending the discussion beyond common issues like data volume. Lastly, we highlight recent advancements and propose directions for future research, including specific research gaps. By providing a structured approach, key findings, and a clearer understanding of the underlying challenges, this work aims to strengthen the foundation of digital forensics. 

_Keywords:_ Event reconstruction, Timeline, Digital investigation, Methodology, Artifacts, Terminology, Framework, Challenges 

## **1. Introduction** 

Event reconstruction involves recreating past events by analyzing digital artifacts, allowing examiners to determine system activities and make informed conclusions about what occurred. While traditional forensic science benefits from a welldefined framework summarizing the field (Ribaux, 2023), event reconstruction in digital forensics is often discussed in fragmented terms focusing on tasks such as super timeline creation (Gujnsson, 2010; Metz et al., 2024), tampering detection (Palmbach & Breitinger, 2020; Studiawan & Sohel, 2021) or environmental peculiarities (Schatz et al., 2006). As a result, research has centered on these narrow aspects, leaving broader challenges underexplored or overlooked. The absence of a unified perspective has led to a proliferation of terms, making it difficult to discuss event reconstruction comprehensively or find relevant research, e.g., some studies use the term artifact (Harichandran et al., 2016), others refer to observable facets (Jaquet-Chiffelle & Casey, 2021). Terms such as events (Carrier & Spafford, 2004a), user actions, interactions, or clicks (Neasbitt et al., 2014) are inconsistently used in literature. 

_The three contributions:_ First, the article discusses concepts and definitions in timeline-based event reconstruction and integrates them into a new visual model (the timeline-based event reconstruction model or _TER-Model_ ), divided into four quadrants, integrating digital forensic timeline-based terminology and Ribaux (2014) model. Second, with this delineation, we provide a thorough discussion of the issues associated with 

> Corresponding author _Email addresses:_ `frank.breitinger@uni-a.de` (Frank Breitinger), `hudan@its.ac.id` (Hudan Studiawan), `christopher.hargreaves@cs.ox.ac.uk` (Chris Hargreaves) 

timeline-based event reconstruction. These issues can be used to evaluate event reconstructions and identify areas of uncertainty in the results. They can also be used to systematically identify weaknesses in the timeline generation and analysis techniques and contribute to a knowledge base of such weaknesses such as SOLVE-IT (Hargreaves et al., 2025). Third, we provide future research directions needed within each quadrant of the event reconstruction process. This paper is predominantly theoretical, aiming to harmonize timeline-based event reconstruction terminology, however, a practical illustration of the use of the model is available online<sup>1</sup> . 

_Not in scope:_ The identification of relevant devices (computer profiling, Marrington et al. (2007)), legal constraints or ethical issues (Losavio et al., 2015), technical challenges such as encryption, sophistication of crime (Karie & Venter, 2015), or very general challenges, e.g., that results must be reproducible and verifiable (Soltani & Seno, 2019). 

_Outline:_ The next section summarizes core works in event reconstruction which served as a foundation for this work. Subsequently, Sec. 3 presents terms and technology in existing literature and outlines the terminology used in this article. A contribution of this work is the TER-model which is developed and described in Sec. 4. Using the model, we identified challenges according to the methodology in Sec. 5 and organized the challenges for event reconstruction in the two main sections: Challenges stemming from environmental and process-related factors and Challenges stemming from deliberate interference, which are summarized as key findings in Sec. 8. Considering these, Sec. 9 provides a discussion and identifies specific research gaps. The final section concludes the paper. 

> 1 `https://github.com/chrishargreaves/TER-model-example` 

_Preprint submitted to DFRWS USA 2025_ 

_April 28, 2025_ 

## **2. Event reconstruction** 

Lee et al. (2001) and many others have discussed event reconstruction for physical crime scenes. Carrier & Spafford (2004a,b) were the first to define it as applied in digital forensics and presented an event-based investigation framework. Their work defines the basic terminology and introduces a formal process model that mirrors physical crime scene investigations but is tailored to the unique aspects of digital evidence. We borrow from this work as discussed in Sec. 3.1. 

Casey (2011)s work includes the practicalities of linking evidence to behaviors and motives. Casey emphasizes three core analysis types: (1) temporal which helps establish the timeline of events (the focus of this article), (2) relational which explores the connections between objects, people, and locations, clarifying how different elements of the crime are related, and (3) functional which assesses what was possible or impossible, such as determining how a system or tool was used in the crime. Chabot et al. (2015a) defines terminology based on existing works, outlines challenges, and evaluates existing approaches. However, the authors limit their challenges to the volume of data and data heterogeneity where this article provides a broader discussion. Our work complements these existing works by providing a new visual model and a thorough discussion of challenges and future research. 

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

- **Environment bounds:** The changes resulting from an event may be distributed across multiple locations, including cloud environments, resulting in fragmented evidence that is challenging to collect and analyze (Group et al., 2014; Joseph & Singh, 2019; Manral et al., 2019). 

Even with the cooperation of external service providers, data cannot be recovered, particularly when logging is explicitly disabled, as is often the case with many VPN services. 

## _6.2. Q2: Post-Event Period_ 

This period relates to the influence of time on the changes left behind after an event. 

## _6.2.1. Subsequent events impacting changes_ 

Over time the changes generated by the primary event are altered by subsequent events (referred to as intrinsic events by Jaquet-Chiffelle & Casey (2021), or evidence dynamics by Gruber et al. (2023)). 

- **Deletion:** Initial changes may disappear due to subsequent events. Examples are rotating logs (Sandvik et al., 2021), temporary files, routine cleanup tasks, or reboots. 

- **Alteration** / **overwriting:** Subsequent events can modify or replace existing data. For instance, Khan et al. (2007) mention that much of the application footprint is rewritten each time the application runs. Routine file operations, such as automatic backups or updates, may also overwrite metadata, configurations, or timestamps (Soltani et al., 2019). 

## _6.2.2. Aging and degradation_ 

Digital artifacts and physical devices are susceptible to degradation, affecting their reliability and accessibility. This degradation can manifest as file corruption, obsolescence of file types, or the deterioration of storage media. Furthermore, changes in software, file formats, or logging systems can introduce additional challenges. As schemas evolve, inconsistencies in log formats may emerge, complicating the process of reconciling older and newer data entries. Backward compatibility issues also arise when outdated systems or logs are incompatible with modern tools, requiring extra effort to ensure that historical data remains interpretable and consistent across different versions (Schuster, 2007). 

## _6.3. Q3: Timeline_ 

This third quadrant summarizes all timeline-related challenges. We decided to include the trans-boundary boxes, i.e., timeline generation (Q2-Q3) and timeline analysis (Q3-Q4), in this section as we think they are closer related to the timeline. 

## _6.3.1. Timeline generation_ 

Data comes from various systems, including traditional computing environments and a growing number of IoT devices, each with distinct structures, conventions, and formats (Patterson & Hargreaves, 2012; Mohammed et al., 2016). This increasing _heterogeneity_ of both data sources and devices causes several challenges. 

- **Artifact** / **timestamp extraction:** Extracting data presents an ongoing challenge, as tools must be continuously updated to accommodate new and evolving software (Horsman, 2019). The acquisition process can introduce alterations, particularly when conducted on live systems, such as during memory dumps (Soltani & Seno, 2017; Gruber et al., 2023). 

- **Normalization:** This involves converting diverse data types, such as logs, databases, and sensor outputs, into a standardized structure that enables comprehensive analysis (Han et al., 2020). This can be challenging due to different timestamp formats, timestamp resolutions, and timezone settings. Timestamp formats can also change over time, meaning timestamp normalization needs to be updated over time and handle older and newer formats. 

- **Contamination and process problems:** Evidence might be unintentionally modified during collection or handling, e.g., failing to use a write blocker (Gruber et al., 2023) or corrupt software, leading to data contamination. Similarly, lapses in maintaining a proper chain of custody can result in evidence being mishandled, misplaced, or questioned in terms of authenticity and reliability. 

- **Source combination:** Combining data from multiple sources to create a unified perception is challenging, especially when sources have different levels of reliability or granularity (Gmez et al., 2005; Levett et al., 2010). 

## _6.3.2. Tool capabilities and usage_ 

Balancing automated tools with manual analysis is essential yet challenging. While automation expedites the process, it may overlook nuances that a human analyst would catch (Klber et al., 2013) and can introduce various types of error (Hargreaves et al., 2024b). 

- **Usage challenges:** Incorrect settings or carelessness can lead to incorrect results. For example, errors in the configuration of the tools have been shown to result in inaccurate extractions of digital evidence, which can impact the credibility of the findings (Bhat et al., 2021). The transition to a new tool may lead to misinterpretation as tools may interpret/visualize data differently. Some features of tools also do not help in reducing chances of investigator misinterpretation (see Hargreaves et al. (2024b)), e.g., if a tool provides an automated result of a Google search occurring, this is easy to interpret the event occurring as a fact rather than Google search data being present. This is an event reconstruction process, with all the uncertainty that could be present, as discussed in Sec. 6.4. Tools can conflate facts with interpretation within their interfaces. 

- **Transparency:** Many tools operate as black-boxes making it unclear how artifacts are handled. Transparency of functionality is critical, as proprietary processes can influence assumptions or conclusions, leading to misinterpretation. 

7 

- **Handling volume:** Tools may have limits on the amount of data they can process or the complexity of queries, leading to unnoticed gaps in analysis, e.g., a tool limited to analyzing 5,000 files at once. Consequently, validation is essential, but challenging, given the rapid change of artifacts (Horsman, 2018; Arshad et al., 2018). 

- **AI-powered examination:** AI-powered tools introduce complexities regarding explainability and transparency, not just of the models but of training data. Recent approaches such as LLMs are also problematic due to their non-deterministic nature and in many cases opaque training data and processes. These tools can produce inaccurate or misleading outputs, such as AI-generated errors or hallucinations which can affect the analysis (Scanlon et al., 2023). 

Developers aiming to create tools should consider the seven criteria outlined by Chabot et al. (2015b), which provide a comprehensive framework for ensuring an efficient reconstruction tool. 

## _6.4. Q4: Decision Making_ 

Q4 involves the generation and testing of hypotheses based on the timeline. This is critical and Hargreaves (2009) goes as far as defining a digital investigation as a process that formulates and tests hypothesis using digital evidence with the prior stages facilitating this goal. Some areas of this are explored, e.g., timeline analysis, but others, such as hypothesis forming and testing are less frequently discussed. 

## _6.4.1. Timeline analysis_ 

Although the processing is mostly done using tools, this section highlights challenges originating from the processing of timeline entries. 

- **Volume of data:** The extensive amount of information (number of entries in the timeline) makes the analysis timeconsuming (Chabot et al., 2015a) and overloads examiners. Additionally, significant resources are needed to extract, process, and store this data, including computational power, storage capacity, and advanced data management tools (Quick & Choo, 2014). 

- **Aggregation, organization and visualization:** Techniques 

- such as combining related events into cohesive units (sometimes called high-level events or super events) (Buchholz & Falk, 2005; Kiernan & Terzi, 2009; Hargreaves & Patterson, 2012; Inglot & Liu, 2014; Raju et al., 2017) can streamline analysis but may result in the loss of granularity or context. Similarly, visualizations (Osborne & Turnbull, 2009) require consideration to ensure that they accurately represent the data without oversimplifying or distorting the information. The volume of the raw data can be a challenge to visualize and reduction of the data before visualization is meaningful may be necessary, e.g., Hargreaves & Patterson (2012). 

- **Correlation:** The process of establishing meaningful relation- 

- ships between disparate timelines entries is fraught with diffi culties, especially when data originates from various sources or formats (Schatz et al., 2006) or times across environments are not synchronized (Marangos et al., 2016). Detecting and validating these connections requires experience and meticulous attention (Amato et al., 2017). For example, incorrect 

handling of local time vs. UTC can disrupt the sequencing of events, particularly in global systems where data spans multiple time zones (Buchholz & Tjaden, 2007). Verifying data across different sources and formats is challenging but necessary to ensure the accuracy and completeness of the reconstructed timeline. 

## _6.4.2. Interpretation, trust and integrity_ 

Ensuring that data is accurate and trustworthy is fundamental (Neale et al., 2022). Determining which sources to trust and how to weigh them can significantly affect the reliability of the reconstruction. This challenge becomes even more pronounced when different sources report the same event but provide inconsistent or conflicting details, leading to uncertainty. 

- **Interpretation:** Investigators work with a static set of data which includes evidence and irrelevant information generated by subsequent activities or during investigative processes (Roux et al., 2022). Misinterpretation can arise from factors such as incorrect ordering, aggregation, or filtering of entries, leading to distortions in the reconstructed narrative but also from unawareness of an examiner, i.e., insufficient knowledge of an event or timestamp (Boyd & Forster, 2004). 

- **Untrusted internal sources:** The presence of anti-forensic tools (Conlan et al., 2016) or tampering indicators, such as manipulated timestamps or hidden data, raises suspicion about the authenticity of the evidence<sup>3</sup> . According to Neale (2023), detecting and addressing such tampering is crucial to maintaining trust in the evidence (more in Sec. 7). 

- **Untrusted external sources:** Combining data from external sources, such as cloud services, introduces additional challenges. When the integrity of these sources cannot be independently verified, especially due to possible alterations in transit or at rest, the reliability of the event reconstruction may be compromised (Battistoni et al., 2016). 

## _6.4.3. Knowledge and perception bias_ 

Investigators may interpret evidence differently based on their prior knowledge, experience, or expectations, which can lead to skewed interpretations of the data. Perception and decision bias may cause certain patterns or details to be overlooked. 

- **Artifact interpretation knowledge:** Previous knowledge may become outdated due to the release of a new operating system, or new version of an application (Horsman, 2019). Examiners may be unaware of certain behaviors (e.g., (Thierry & Mller, 2022) identified multiple unexpected and noncompliant behaviors of timestamps). Limitations in knowledge reduce the investigators ability to generate viable alternative hypotheses that would produce the same artifacts. 

- **Algorithmic bias:** Tools operate based on algorithms that might make certain assumptions or prioritize specific types of data, which can introduce biases into the reconstructed events (Jinad et al., 2024). For instance, an AI-powered tool may be biased due to unbalanced training data. 

> 3We decided to include this challenge here and not in Sec. 7 (deliberate interference) as the presence of these tools does not necessarily mean that they were executed. 

8 

- **Human bias:** Analysts may bring their own preconceptions into the analysis, influencing how they interpret and prioritize different events (Kang et al., 2013). This can lead to confirmation bias, where analysts might favor hypotheses that align with their pre-existing beliefs or expectations, unintentionally skewing the analysis (Kassin et al., 2013). 

## _6.4.4. Complexity in testing hypotheses_ 

Testing hypotheses against a timeline is complex, especially when considering all the aforementioned challenges. 

- **Multiple interpretations:** Evidence may be open to multiple interpretations, making it difficult to draw definitive conclusions and infer events from the past. This ambiguity can lead to varied interpretations of the same data, which impacts the ability to test hypotheses with certainty. Effective hypothesis testing must address temporal inaccuracies or manage the inherent uncertainty that arises from imperfect data such as log files (Latzo & Freiling, 2019). 

- **Defining error:** Hargreaves (2009) discusses that error in event reconstruction can be defined as the difference between the inferred history and the true history of the examined digital evidence. This error cannot necessarily be expressed as a definite value, e.g., _x_  _y_ , but can be expressed as uncertainty (possible error) in the inferred events, i.e., alternative possible hypothesized events that explain the current state of the examined digital evidence. Communicating these uncertainties transparently is vital to ensure that conclusions drawn are appropriately qualified and reflect the limitations of the available evidence. 

- **Environment manipulation:** It is possible to disable or tamper with logging mechanisms, preventing activities from being recorded. Similarly, security tools may be compromised or altered (MITRE, 2023). Decoys such as fake accounts or planted traps such as cleanup scripts may be used to further obscure activities. 

- **Anti-forensics and malware:** Adversaries may use software to obscure their actions. For instance, anti-forensic tools erase or alter evidence (Conlan et al., 2016) or rootkits and malware to cover access and manipulations to files and logs (Palmbach & Breitinger, 2020). Anonymization services such as VPNs and TOR hide the attackers origin, making it difficult to trace activities 

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

- Du, X., Hargreaves, C., Sheppard, J., Anda, F., Sayakkara, A., Le-Khac, N.A., & Scanlon, M. (2020a). SoK: Exploring the state of the art and the future potential of artificial intelligence in digital forensic investigation. In _Proceedings of the 15th International Conference on Availability, Reliability and Security_ (pp. 110). 

- Du, X., Le, Q., & Scanlon, M. (2020b). Automated artefact relevancy determination from artefact metadata and associated timeline events. In _2020 International Conference on Cyber Security and Protection of Digital Services (Cyber Security)_ (pp. 18). IEEE. 

- Fernndez-Fuentes, X., Pena, T. F., & Cabaleiro, J. C. (2022). Digital forensic analysis methodology for private browsing: Firefox and chrome on linux as a case study. _Computers_ & _Security_ , _115_ , 102626. 

- Galhuber, M., & Luh, R. (2021). Time for Truth: Forensic Analysis of NTFS Timestamps. In _Proceedings of the 16th International Conference on Availability, Reliability and Security_ ARES 21 (pp. 110). New York, NY, USA: Association for Computing Machinery. doi: `10.1145/3465481.3470016` . 

- Gladyshev, P., & Patel, A. (2004). Finite state machine approach to digital event reconstruction. _Digital Investigation_ , _1_ , 130149. doi: `10.1016/j.di in.2004.03.001` . 

- Gmez, R., Herrerias, J., & Mata, E. (2005). Using lamports logical clocks to consolidate log files from different sources. In _International Workshop on Innovative Internet Community Systems_ (pp. 126133). Springer. 

- Grajeda, C., Breitinger, F., & Baggili, I. (2017). Availability of datasets for digital forensicsand what is missing. _Digital Investigation_ , _22_ , S94S105. 

- Grajeda, C., Sanchez, L., Baggili, I., Clark, D., & Breitinger, F. (2018). Experience constructing the artifact genome project (agp): managing the domains knowledge one artifact at a time. _Digital Investigation_ , _26_ , S47S58. 

- Group, N. C. C. F. S. W. et al. (2014). _NIST cloud computing forensic science challenges_ . Technical Report National Institute of Standards and Technology. 

- Gruber, J., Hargreaves, C. J., & Freiling, F. C. (2023). Contamination of digital evidence: Understanding an underexposed risk. _Forensic Science International: Digital Investigation_ , _44_ , 301501. doi: `10.1016/j.fsidi.2023. 301501` . 

- Gujnsson, K. (2010). Mastering the super timeline with log2timeline. _SANS Institute_ , . 

- Hampton, N., & Baig, Z. A. (2016). Timestamp analysis for quality validation of network forensic data. In _Network and System Security: 10th International Conference, NSS 2016, Taipei, Taiwan, September 28-30, 2016, Proceedings 10_ (pp. 235248). Springer. 

- Han, J., Kim, J., & Lee, S. (2020). 5w1h-based expression for the effective sharing of information in digital forensic investigations. _arXiv preprint arXiv:2010.15711_ , . 

- Hargreaves, C., van Beek, H., & Casey, E. (2025). Solve-it: A proposed digital forensic knowledge base inspired by mitre att&ck. _Forensic Science International: Digital Investigation_ , _52_ , 301864. 

- Hargreaves, C., Breitinger, F., Dowthwaite, L., Webb, H., & Scanlon, M. (2024a). Dfpulse: The 2024 digital forensic practitioner survey. _Forensic Science International: Digital Investigation_ , _51_ , 301844. 

- Hargreaves, C., Nelson, A., & Casey, E. (2024b). An abstract model for digital forensic analysis tools-a foundation for systematic error mitigation analysis. _Forensic Science International: Digital Investigation_ , _48_ , 301679. doi: `10. 1016/j.fsidi.2023.301679` . 

- Hargreaves, C., & Patterson, J. (2012). An automated timeline reconstruction approach for digital forensic investigations. _Digital Investigation_ , _9_ , S69 S79. doi: `10.1016/j.diin.2012.05.006` . 

- Hargreaves, C. J. (2009). _Assessing the reliability of digital evidence from live investigations involving encryption._ . Ph.D. thesis Cranfield University, UK. 

- Harichandran, V. S., Walnycky, D., Baggili, I., & Breitinger, F. (2016). Cufa: A more formal definition for digital forensic artifacts. _Digital Investigation_ , _18_ , S125S137. 

- Henderson, G. (2009). _A Categorization of Computer Clocks_ . Technical Report Department of Computer Science, James Madison University. 

- Henseler, H., & van Beek, H. (2023). Chatgpt as a copilot for investigating digital evidence. In _Proceedings of the Third International Workshop on Artificial Intelligence and Intelligent Assistance for Legal Professionals in_ 

13 

_the Digital Workplace_ (pp. 5869). 

- Horsman, G. (2018). i couldnt find it your honour, it mustnt be there!tool errors, tool limitations and user error in digital forensics. _Science_ & _Justice_ , _58_ , 433440. 

- Horsman, G. (2019). Raiders of the lost artefacts: Championing the need for digital forensics research. _Forensic Science International: Reports_ , _1_ , 100003. 

- Inglot, B., & Liu, L. (2014). Enhanced timeline analysis for digital forensic investigations. _Information Security Journal: A Global Perspective_ , _23_ , 32 44. 

- Jahankhani, H., & Hosseinian-far, A. (2014). Digital forensics education, training and awareness. In _Cyber Crime and Cyber Terrorism Investigators Handbook_ (pp. 91100). Elsevier. doi: `10.1016/B978-0-12-8007433.00008-6` . 

- Jang, D.-i., Ahn, G.-J., Hwang, H., & Kim, K. (2016). Understanding antiforensic techniques with timestamp manipulation. In _2016 IEEE 17th International Conference on Information Reuse and Integration (IRI)_ (pp. 609 614). IEEE. 

- Jaquet-Chiffelle, D.-O., & Casey, E. (2021). A formalized model of the Trace. _Forensic Science International_ , _327_ , 110941. doi: `10.1016/j.forsciint. 2021.110941` . 

- Jarrett, A., & Choo, K.-K. R. (2021). The impact of automation and artificial intelligence on digital forensics. _Wiley Interdisciplinary Reviews: Forensic Science_ , _3_ , e1418. 

- Jinad, R., Gupta, K., Simsek, E., & Zhou, B. (2024). Bias and fairness in software and automation tools in digital forensics. _J. Surveill. Secur. Saf_ , _5_ , 1935. 

- Joseph, A., & Singh, K. J. (2019). Digital forensics in distributed environment. In _Cloud Security: Concepts, Methodologies, Tools, and Applications_ (pp. 11571177). IGI Global. 

- Jrgensen, J. P. (2021). Trace reconstruction in system logs for processing with process mining. _Procedia Computer Science_ , _180_ , 352357. 

- Kaart, M., & Laraghy, S. (2014). Android forensics: Interpretation of timestamps. _Digital Investigation_ , _11_ , 234248. doi: `10.1016/j.diin.2014. 05.001` . 

- Kang, J., Lee, S., & Lee, H. (2013). A digital forensic framework for automated user activity reconstruction. In _Information Security Practice and Experience: 9th International Conference, ISPEC 2013, Lanzhou, China, May 12-14, 2013. Proceedings 9_ (pp. 263277). Springer. 

- Karie, N. M., & Venter, H. S. (2015). Taxonomy of challenges for digital forensics. _Journal of Forensic Sciences_ , _60_ , 885893. 

- Kassin, S. M., Dror, I. E., & Kukucka, J. (2013). The forensic confirmation bias: Problems, perspectives, and proposed solutions. _Journal of applied research in memory and cognition_ , _2_ , 4252. 

- Kebande, V. R., & Venter, H. S. (2018). Novel digital forensic readiness technique in the cloud environment. _Australian Journal of Forensic Sciences_ , _50_ , 552591. doi: `10.1080/00450618.2016.1267797` . 

- Khan, M., Chatwin, C. R., & Young, R. C. (2007). A framework for postevent timeline reconstruction using neural networks. _Digital Investigation_ , _4_ , 146157. 

- Khan, M., & Wakeman, I. (2006). Machine learning for post-event timeline reconstruction. In _First Conference on Advances in Computer Security and Forensics, Liverpool, UK_ . Citeseer. 

- Khan, S., Gani, A., Wahab, A. W. A., Bagiwa, M. A., Shiraz, M., Khan, S. U., Buyya, R., & Zomaya, A. Y. (2016). Cloud log forensics: Foundations, state of the art, and future directions. _ACM Computing Surveys (CSUR)_ , _49_ , 142. doi: `10.1145/2906149` . 

- Kiernan, J., & Terzi, E. (2009). Eventsummarizer: a tool for summarizing large event sequences. In _Proceedings of the 12th International Conference on Extending Database Technology: Advances in Database Technology_ (pp. 11361139). 

- Kos, M., & El Fray, I. (2020). Securing event logs with blockchain for iot. In _International Conference on Computer Information Systems and Industrial Management_ (pp. 7787). Springer. doi: `10.1007/978-3-030-476793_7` . 

- Kwan, M., Chow, K.-P., Law, F., & Lai, P. (2008). Reasoning about evidence using bayesian networks. In _Advances in Digital Forensics IV 4_ (pp. 275 289). Springer. 

- Klber, S., Dewald, A., & Freiling, F. C. (2013). Forensic ApplicationFingerprinting Based on File System Metadata. In _2013 Seventh International Conference on IT Security Incident Management and IT Forensics_ (pp. 98112). doi: `10.1109/IMF.2013.20` . 

- Latzo, T., & Freiling, F. (2019). Characterizing the Limitations of Forensic Event Reconstruction Based on Log Files. In _2019 18th IEEE International_ 

_Conference On Trust, Security And Privacy In Computing And Communications_ / _13th IEEE International Conference On Big Data Science And Engineering (TrustCom_ / _BigDataSE)_ (pp. 466475). doi: `10.1109/TrustCom/B igDataSE.2019.00069` iSSN: 2324-9013. 

- Lee, H. C., Palmbach, T., & Miller, M. T. (2001). _Henry Lees crime scene handbook_ . Academic Press. 

- Levett, C. P., Jhumka, A., & Anand, S. S. (2010). Towards event ordering in digital forensics. In _Proceedings of the 12th ACM workshop on Multimedia and security_ (pp. 3542). 

- Losavio, M., Pastukov, P., & Polyakova, S. (2015). Cyber black box/event data recorder: legal and ethical perspectives and challenges with digital forensics. _Journal of Digital Forensics, Security and Law_ , _10_ , 4. 

- Lyle, J. R., Guttman, B., Butler, J. M., Sauerwein, K., Reed, C., & Lloyd, C. E. (2022). _Digital Investigation Techniques: A NIST Scientific Foundation Review_ . Technical Report National Institute of Standards and Technology. doi: `10.6028/NIST.IR.8354-draft` . 

- Malhotra, A., Cohen, I. E., Brakke, E., & Goldberg, S. (2015). Attacking the network time protocol. _Cryptology ePrint Archive_ , . 

- Manral, B., Somani, G., Choo, K.-K. R., Conti, M., & Gaur, M. S. (2019). A systematic survey on cloud forensics challenges, solutions, and future directions. _ACM Computing Surveys (CSUR)_ , _52_ , 138. 

- Marangos, N., Rizomiliotis, P., & Mitrou, L. (2016). Time synchronization: pivotal element in cloud forensics. _Security and Communication Networks_ , _9_ , 571582. 

- Markov, E., Sokol, P., & Kovcov, K. (2022). Detection of relevant digital evidence in the forensic timelines. In _2022 14th International Conference on Electronics, Computers and Artificial Intelligence (ECAI)_ (pp. 17). IEEE. 

- Marrington, A., Baggili, I., Mohay, G., & Clark, A. (2011). CAT Detect (Computer Activity Timeline Detection): A tool for detecting inconsistency in computer activity timelines. _Digital Investigation_ , _8_ , S52S61. 

- Marrington, A., Mohay, G., Clark, A., & Morarji, H. (2007). Event-based computer profiling for the forensic reconstruction of computer activity. _AusCERT 2007, IT-Security: Finding the Balance_ , (pp. 7187). 

- Metz, J., Gudjonsson, K., White, D. et al. (2024). log2timeline Plaso: Super timeline all the things. `https://github.com/log2timeline/plaso` . 

- Michelet, G., & Breitinger, F. (2024). Chatgpt, llama, can you write my report? an experiment on assisted digital forensics reports written using (local) large language models. _Forensic Science International: Digital Investigation_ , _48_ , 301683. 

- MITRE (2023). Impair defenses. `https://attack.mitre.org/technique s/T1562/` . 

- Mohammed, H., Clarke, N., & Li, F. (2016). An automated approach for digital forensic analysis of heterogeneous big data. _Journal of Digital Forensics, Security and Law_ , _11_ , 9. 

- Neale, C. (2023). Fool me once: A systematic review of techniques to authenticate digital artefacts. _Forensic Science International: Digital Investigation_ , _45_ , 301516. doi: `10.1016/j.fsidi.2023.301516` . 

- Neale, C., Kennedy, I., Price, B., Yu, Y., & Nuseibeh, B. (2022). The case for zero trust digital forensics. _Forensic Science International: Digital Investigation_ , _40_ , 301352. 

- Neasbitt, C., Perdisci, R., Li, K., & Nelms, T. (2014). Clickminer: Towards forensic reconstruction of user-browser interactions from network traces. In _Proceedings of the 2014 ACM SIGSAC Conference on Computer and Communications Security_ (pp. 12441255). 

- Nguyen, H. T. C., & Comuzzi, M. (2019). Event log reconstruction using autoencoders. In _Service-Oriented ComputingICSOC 2018 Workshops_ (pp. 335350). Springer. 

- Nordvik, R., & Axelsson, S. (2022). It is about timedo exfat implementations handle timestamps correctly? _Forensic Science International: Digital Investigation_ , _42_ , 301476. 

- Oh, J., Lee, S., & Hwang, H. (2022). Forensic recovery of file system metadata for digital forensic investigation. _IEEE Access_ , _10_ , 111591111606. 

- Osborne, G., & Turnbull, B. (2009). Enhancing computer forensics investigation through visualisation and data exploitation. In _2009 International Conference on Availability, Reliability and Security_ (pp. 10121017). IEEE. 

- Palmbach, D., & Breitinger, F. (2020). Artifacts for Detecting Timestamp Manipulation in NTFS on Windows and Their Reliability. _Forensic Science International: Digital Investigation_ , _32_ , 300920. doi: `10.1016/j.fsidi. 2020.300920` . 

- Patterson, J., & Hargreaves, C. J. (2012). The Potential for cross-drive analysis using automated digital forensic timelines. `https://dspace.lib.cranf ield.ac.uk/handle/1826/8088` . Accepted: 2014-01-23T05:01:12Z. 

- Quick, D., & Choo, K.-K. R. (2014). Impacts of increasing volume of digital forensic data: A survey and future research challenges. _Digital Investiga-_ 

14 

_tion_ , _11_ , 273294. 

- Raghavan, S., & Saran, H. (2013). Unitime: Timestamp interpretation engine for developing unified timelines. In _2013 8th International Workshop on Systematic Approaches to Digital Forensics Engineering (SADFE)_ (pp. 1 7). IEEE. 

- Raju, B. K., Gosala, N. B., & Geethakumari, G. (2017). Closer: applying aggregation for effective event reconstruction of cloud service logs. In _Proceedings of the 11th International Conference on Ubiquitous Information Management and Communication_ (pp. 18). 

- Reddy, K., & Venter, H. S. (2013). The architecture of a digital forensic readiness management system. _Computers_ & _security_ , _32_ , 7389. doi: `10.1016/j.cose.2012.09.008` . 

- Ribaux, O. (2014). _Police scientifique: le renseignement par la trace_ . Sciences forensiques. Lausanne: Presses polytechniques et universitaires romandes. 

- Ribaux, O. (2023). _De la police scientifique  la traologie: le renseignement par la trace_ . EPFL Press. 

- Rivera-Ortiz, F., & Pasquale, L. (2019). Towards automated logging for forensic-ready software systems. In _2019 IEEE 27th International Requirements Engineering Conference Workshops (REW)_ (pp. 157163). IEEE. doi: `10.1109/REW.2019.00033` . 

- Roux, C., Bucht, R., Crispino, F., De Forest, P., Lennard, C., Margot, P., Miranda, M. D., NicDaeid, N., Ribaux, O., Ross, A., & Willis, S. (2022). The Sydney declaration  Revisiting the essence of forensic science through its fundamental principles. _Forensic Science International_ , _332_ , 111182. doi: `10.1016/j.forsciint.2022.111182` . 

- Sachowski, J. (2019). _Implementing digital forensic readiness: From reactive to proactive process_ . CRC Press. doi: `10.1016/C2015-0-00701-8` . 

- Sandvik, J.-P., Franke, K., & rnes, A. (2021). Towards a generic approach of quantifying evidence volatility in resource constrained devices. _Digital Forensic Investigation of Internet of Things (IoT) Devices_ , (pp. 2145). 

- Sandvik, J.-P., & rnes, A. (2018). The reliability of clocks as digital evidence under low voltage conditions. _Digital Investigation_ , _24_ , S10S17. doi: `10. 1016/j.diin.2018.01.003` . 

- Scanlon, M., Breitinger, F., Hargreaves, C., Hilgert, J.-N., & Sheppard, J. (2023). ChatGPT for digital forensic investigation: The good, the bad, and the unknown. _Forensic Science International: Digital Investigation_ , _46_ , 301609. 

- Schatz, B., Mohay, G., & Clark, A. (2006). A correlation method for establishing provenance of timestamps in digital evidence. _Digital Investigation_ , _3_ , 98107. doi: `10.1016/j.diin.2006.06.009` . 

- Schneider, J., Dsel, L., Lorch, B., Drafz, J., & Freiling, F. (2022). Prudent design principles for digital tampering experiments. _Forensic Science International: Digital Investigation_ , _40_ , 301334. doi: `10.1016/j.fsidi.2022. 301334` . 

- Schneider, J., Eichhorn, M., Dreier, L. M., & Hargreaves, C. (2024). Applying digital stratigraphy to the problem of recycled storage media. _Forensic Science International: Digital Investigation_ , _49_ , 301761. 

- Schneider, J., Wolf, J., & Freiling, F. (2020). Tampering with Digital Evidence is Hard: The Case of Main Memory Images. _Forensic Science International: Digital Investigation_ , _32_ , 300924. doi: `10.1016/j.fsidi.2020.300924` . 

- Schuster, A. (2007). Introducing the microsoft vista event log file format. _Digital Investigation_ , _4_ , 6572. 

- Silalahi, S., Ahmad, T., & Studiawan, H. (2023a). Dfler: Drone flight log entity recognizer to support forensic investigation on drone device. _Software Impacts_ , _15_ , 100457. doi: `10.1016/j.simpa.2022.100457` . 

- Silalahi, S., Ahmad, T., & Studiawan, H. (2023b). Transformer-based named entity recognition on drone flight logs to support forensic investigation. _IEEE Access_ , _11_ , 32573274. doi: `10.1109/ACCESS.2023.3234605` . 

- Silalahi, S., Ahmad, T., & Studiawan, H. (2023c). Transformer-based sentiment analysis for anomaly detection on drone forensic timeline. In _2023 11th International Symposium on Digital Forensics and Security (ISDFS)_ (pp. 16). IEEE. doi: `ISDFS58141.2023.10131749` . 

- Soltani, S., Hosseini Seno, S. A., & sadoghi yazdi, H. (2019). Event Reconstruction using Temporal Pattern of File System Modification. _IET Information Security_ , _13_ . doi: `10.1049/iet-ifs.2018.5209` . 

- Soltani, S., & Seno, S. A. H. (2017). A survey on digital evidence collection and analysis. In _2017 7th International Conference on Computer and Knowledge Engineering (ICCKE)_ (pp. 247253). IEEE. 

- Soltani, S., & Seno, S. A. H. (2019). A formal model for event reconstruction in digital forensic investigation. _Digital Investigation_ , _30_ , 148160. doi: `10. 1016/j.diin.2019.07.006` . 

evolving systems. _Forensic Science International: Digital Investigation_ , _52_ , 301867. URL: `https://www.sciencedirect.com/science/arti cle/pii/S266628172500006X` . doi: `https://doi.org/10.1016/j.fs idi.2025.301867` . DFRWS EU 2025 - Selected Papers from the 12th Annual Digital Forensics Research Conference Europe. 

   - Sreya, E., Wadhwa, M. et al. (2023). Enhancing digital investigation: Leveraging chatgpt for evidence identification and analysis in digital forensics. In _2023 International Conference on Computing, Communication, and Intelligent Systems (ICCCIS)_ (pp. 733738). IEEE. 

   - Stevens, M. W. (2004). Unification of relative time frames for digital forensics. _Digital Investigation_ , _1_ , 225239. doi: `10.1016/j.diin.2004.07.003` . 

   - Studiawan, H. (2023). Event abstration in a forensic timeline. In _International Conference for Information and Communication Technologies_ (pp. 119129). Springer. 

   - Studiawan, H., Hasan, M. F., & Pratomo, B. A. (2023). Rule-based entity recognition for forensic timeline. In _2023 Conference on Information Communications Technology and Society (ICTAS)_ (pp. 16). IEEE. 

   - Studiawan, H., Payne, C., & Sohel, F. (2017). Graph clustering and anomaly detection of access control log for forensic purposes. _Digital Investigation_ , _21_ , 7687. 

   - Studiawan, H., & Sohel, F. (2021). Anomaly detection in a forensic timeline with deep autoencoders. _Journal of Information Security and Applications_ , _63_ , 103002. 

   - Studiawan, H., Sohel, F., & Payne, C. (2019). A survey on forensic investigation of operating system logs. _Digital Investigation_ , _29_ , 120. doi: `10.1016/j.diin.2019.02.005` . 

   - Studiawan, H., Sohel, F., & Payne, C. (2020a). Automatic event log abstraction to support forensic investigation. In _Proceedings of the Australasian Computer Science Week Multiconference_ (pp. 19). 

   - Studiawan, H., Sohel, F., & Payne, C. (2020b). Sentiment analysis in a forensic timeline with deep learning. _IEEE Access_ , _8_ , 6066460675. doi: `10.1109/ ACCESS.2020.2983435` . 

   - Thierry, A., & Mller, T. (2022). A systematic approach to understanding MACB timestamps on Unix-like systems. _Forensic Science International: Digital Investigation_ , _40_ , 301338. 

   - Turnbull, B., & Randhawa, S. (2015). Automated event and social network extraction from digital evidence sources with ontological mapping. _Digital Investigation_ , _13_ , 94106. doi: `10.1016/j.diin.2015.04.004` . 

   - Vanini, C., Breitinger, F., & Hargreaves, C. (2023). A discussion of sources and quality/reliability of events for timelines. Presentation at the Digital Forensics Research Conference 2023 (Bonn, Germany). 

   - Vanini, C., Gruber, J., Hargreaves, C., Benenson, Z., Freiling, F., & Breitinger, F. (2024a). Strategies and challenges of timestamp tampering for improved digital forensic event reconstruction (extended version). _arXiv preprint arXiv:2501.00175_ , . 

   - Vanini, C., Hargreaves, C. J., van Beek, H., & Breitinger, F. (2024b). Was the clock correct? Exploring timestamp interpretation through time anchors for digital forensic event reconstruction. _Forensic Science International: Digital Investigation_ , _49_ , 301759. doi: `10.1016/j.fsidi.2024.301759` . 

   - VMware (2008). Timekeeping in VMware virtual machines. https://www.cse.psu.edu/ buu1/teaching/spring06/papers/vmwaretiming.pdf. 

   - Weijters, A., & van der Aalst, W. M. (2001). Process mining: Discovering workflow models from event-based data. In _Belgium-Netherlands Conf. on Artificial Intelligence_ . 

   - Willassen, S. (2008a). Hypothesis-based investigation of digital timestamps. In I. Ray, & S. Shenoi (Eds.), _Advances in Digital Forensics IV IFIP  The International Federation for Information Processing_ (pp. 7586). Boston, MA: Springer US volume 285. doi: `10.1007/978-0-387-84927-0_7` . 

   - Willassen, S. Y. (2008b). Finding evidence of antedating in digital investigations. In _2008 Third International Conference on Availability, Reliability and Security_ (pp. 2632). doi: `10.1109/ARES.2008.149` . 

   - Willassen, S. Y. (2008c). Timestamp evidence correlation by model based clock hypothesis testing. In _Proceedings of the 1st International Conference on Forensic Applications and Techniques in Telecommunications, Information, and Multimedia and Workshop_ e-Forensics 08 (pp. 16). Brussels, BEL: ICST (Institute for Computer Sciences, Social-Informatics and Telecommunications Engineering). 

   - Xu, W., & Xu, D. (2022). Visualizing and reasoning about presentable digital forensic evidence with knowledge graphs. In _2022 19th Annual International Conference on Privacy, Security_ & _Trust (PST)_ (pp. 110). IEEE. 

- Song, S., Cao, Y., & Wang, J. (2016). Cleaning timestamps with temporal constraints. _Proceedings of the VLDB Endowment_ , _9_ , 708719. 

- Spichiger, H., & Adelstein, F. (2025). Preserving meaning of evidence from 

15 

# SOURCE: 2603.23996v1.pdf

# Forensic Implications of Localized AI: Artifact Analysis of Ollama, LM Studio, and llama.cpp 

Shariq Murtuza 

##### **Abstract** 

The proliferation of local Large Language Model (LLM) runners, such as Ollama, LM Studio and llama.cpp, presents a new challenge for digital forensics investigators. These tools enable users to deploy powerful AI models in an offline manner, creating a potential evidentiary blind spot for investigators. This work presents a systematic, cross platform forensic analysis of these popular local LLM clients. Through controlled experiments on Windows and Linux operating systems, we acquired and analyzed disk and memory artifacts, documenting installation footprints, configuration files, model caches, prompt histories and network activity. Our experiments uncovered a rich set of previously undocumented artifacts for each software, revealing significant differences in evidence persistence and location based on application architecture. Key findings include the recovery of plaintext prompt histories in structured JSON files, detailed model usage logs and unique file signatures suitable for forensic detection. This research provides a foundational corpus of digital evidence for local LLMs, offering forensic investigators reproducible methodologies, practical triage commands and analyse this new class of software. The findings have critical implications for user privacy, the admissibility of AI-related evidence and the development of anti-forensic techniques. 

**Keywords:** Digital Forensics, Large Language Models (LLM), Ollama, LM Studio, llama.cpp, Artifact Analysis, Evidence Recovery 

## **1 Introduction** 

The rapid evolution of Artificial Intelligence (AI), specially Large Language Models (LLMs) has introduced transformative capabilities across countless domains. While cloud based services like OpenAIs ChatGPT have captured mainstream attention, another parallel ecosystem of local LLM runners has emerged, enabling users to operate powerful models directly on their personal workstations [1, 2, 3]. Applications such as Ollama, LM Studio and the underlying llama.cpp engine prioritize privacy, offline functionality and user control, making them increasingly popular for both benign and potentially malicious activities. 

This shift towards localized AI processing presents a significant challenge for the digital forensics community. Malicious actors can leverage these tools to 

1 

generate harmful content, such as phishing emails or novel malware, process stolen data, or plan illicit activities, all while operating under a perceived cloak of privacy that circumvents the monitoring inherent in cloud services[7]. When a device containing such software is seized, investigators are faced with a critical problem: the digital artifacts created by these applications are largely undocumented. The locations of prompt histories, downloaded models, configuration files and activity logs are not standardized and remain unknown to most practitioners. This artifact gap hinders the ability of timeline reconstruction and reconstruction of user actions, establish intent, or attribute malicious activity to a suspect. 

The motivation for this research stems from this urgent operational need. As the use of local LLMs grows, a forensically sound and reproducible methodology for their examination is paramount. Without a systematic understanding of their digital footprint, critical evidence may be overlooked or misinterpreted, compromising both criminal investigations and corporate incident response efforts. The very features that make local LLMs attractive (privacy and offline operation) also make their forensic analysis both essential and challenging[9]. 

This paper addresses this critical gap by providing the first comprehensive, empirical forensic analysis of the most popular local LLM runners. Our work makes the following original contributions to the field of digital forensics: 

- To the best of our knowledge, this work presents the first systematic, cross platform forensic analysis of Ollama, LM Studio and llama.cpp on Windows and Linux operating systems. 

- This work presents a comprehensive corpus of digital artifacts, detailing their file paths, data formats, persistence levels and forensic value for reconstructing user activity. 

- Finally the legal and ethical implications of our findings are presented and connected to the technical recovery of artifacts like prompt histories to evidentiary standards, such as the Daubert standard and pressing privacy considerations. 

By giving this foundational knowledge, this work aims to equip forensic practitioners and researchers with the methodologies necessary to navigate this new and rapidly evolving domain of digital evidence. 

## **2 Background and Related Work** 

The intersection of artificial intelligence and digital forensics is a rapidly expanding field of study. Currently the focus of existing research has been mostly towards exploring artificial intelligence itself as a tool to aid investigators, instead of being the investigation focus. The next section presents existing technologies while identifying the critical gap that this work aims to cover. 

2 

_2.1 Digital Forensics of AI/ML Systems_ 

### **2.1 Digital Forensics of AI/ML Systems** 

Traditional research in the domain of AI and forensics intersection has typically focussed on the application of machine learning techniques to support and strengthen the investigative capabilities [11, 13]. Prior and current works presents the application of artificial intelligence for the automation of the analysis of vast artifacts, detecting network traffic anomalies, identifying unusual user behaviour or classifying digital evidence such as images or malware[10]. These applications aim to improve the efficiency and effectiveness of forensic examiners who face an ever increasing volume of data[14]. The underlying premise of such research is to apply AI as an analytical tool within established digital forensic frameworks, such as those proposed by the Digital Forensics Research Workshop (DFRWS) or the National Institute of Standards and Technology (NIST)[8]. While quite usefull, this research paradigm treats the AI system as a trusted assistant, not as a source of evidence itself [15, 20, 21]. 

### **2.2 Forensic Artifacts from Agentic and LLM Systems** 

In recent times, with the release of powerful Large Language Models (LLMs), the focus of researchers has been drawn towards their potential to be deployed in the digital forensics domain. Multiple studies has explored and tried to evaluate the potential of LLMs to be used as "investigative assistants"[7, 16, 17, 18]. Such task requires the LLMs to be able to summarize the case files, while analyzing textual evidence. More complex tasks involve the application of a LLM for authorship attribution in order to detect unique traits like age and gender using written text[19]. Other works have deeply focused on creating highly specialized domain specific models, such as ForensicLLM, which are finetuned on digital forensics related datasets to provide much more accurate responses by utilizing their context aware processing capabilities[9]. 

These works rely on trusting a large language model to make sensitive decisions, due to which such tools are often supplemented with human monitoring all the steps manually. Such tools or services may also require the subscription of an online LLM hosting service where the LLM is hosted. The tools then collect the data locally and send it to the cloud based service where the LLM is hosted for obtaining results. This can be a hurdle if the case data is confidential or sensitive and requires a high amount of discretion. Privacy is also a major concern and law enforcement cannot be allowed to upload such data to third party services[9]. 

Even if the language models are hosted locally in an offline setting, the models are highly prone to hallucination where they generate factually incorrect data. Other issues include the presence of inherent biases in the training data of the model, the process of complicated and unexplainable decision making process. Such privacy issues further fuels the adoption of local LLM runners. 

3 

_2.3 The Local LLM Ecosystem and the Forensic Gap_ 

### **2.3 The Local LLM Ecosystem and the Forensic Gap** 

This section discusses the current state of local LLM running tools and their forensic implications. The need for privacy, security and confidentiality often results in individuals selecting offline solutions albeit having lower capabilities over stronger and highly capable but third party hosted online solutions. The open source nature of these tools have further captured general interest resulting in rapid capability updation. With modern desktops and laptops being able to run and deploy language models as capable as GPT 3.5 without any external hardware addition. This has been majorly possible due to the efforts of the most powerful and popular open source C++ based inference engine named llama.cpp[3]. This inference engine is built with high optimizations and has become the defacto standard, with Ollama, LM Studio and almost every other local inference tool using llama.cpp at its core. Llama.cpp was originally designed for deployment on consumer grade CPU in an extremely efficient and resource aware manner but has not expanded to now utilize GPUs also, if available. 

llama.cpp has its own specialized file format called GPT-Generated Unified Format (GGUF) [4, 26, 69], which has now become an industry standard. Language models are typically distributed in a binary format packing model weights, metadata and quantization information into a single, portable file. Applications like Ollama and LM Studio have a user friendly graphical interfaces to facilitate interaction with the language models. Ollama and llama.cpp provide a server based API using client server paradigm to allow any application to interact with the locally hosted model [27]. LM Studio on the other hand is built as a standalone application that supports chatting via a graphical interface[28]. This rapid adoption of local LLM deployment softwares has created a large forensic gap. Extensive work has been already done for using large language models as assistants in digital forensics tasks, but there is a near complete absence of academic literature on the forensic analysis of local LLM runners. These offline first, secure assistants have become an evidentiary blind spot for forensic investigators. This push for local AI applications in forensics, has paradoxically created another new class of applications whose digital traces are unexplored and unidentified. To the best of our knowledge this work is the first to explore this avenue, aiming to lay the foundational analysis for evidence processing of these local first AI environments to ensure that cases involving these softwares can be investigated with the same rigor as other digital activity. 

## **3 Threat and Forensic Models** 

To base this research on ground reality, we create a set of different models to cover the possible local LLMs misuse. The threat scenarios guide our forensic approach which aims to be designed to be compliant with scientifically and legally established standards. 

4 

_3.1 Investigative Scenarios_ _<u>(Use</u> Case Models)_ 

### **3.1 Investigative Scenarios (Use Case Models)** 

The threat models are as follow: 

- **Insider Threat:** This scenario involves an employee using a local LLM on a corporate computer system to process (summarize, analyse or rephrasing etc.) confidential internal documents (source code, trade secretor, financial data etc.) and then exfiltrate it. A locally running LLM was chosen by the employee since it wont leave any network traces and would be nearly impossible to track. 

- **Malicious Content Generation:** In this threat scenario, a malicious actor has used a locally deployed LLM in order to make highly sophisticated phishing emails, create malware, generate fraud documents, or create disinformation for a social engineering campaign etc. In this scenario the forensic investigation aims to attribute the creation of this content to the suspects machine. 

- **Contraband Data Processing:** A suspect under investigation has allegedly used a locally deployed LLM to process illegal data. For example, summarization of stolen documents, trade secrets. Recovering the prompts and outputs is essential evidence. 

- **Attribution and Reconstruction:** Once an investigator discovers questionable documents and needs to further identify and link it with the suspects computer. This investigation shall then focusses detecting the specific software that was used to deploy the LLM locally, specific model used, if possible, then the configuration parameters used to infer from it and finally the most important, the exact sequence of textual prompts that generated the final output. 

### **3.2 Forensic Assumptions and Scope** 

This paper works under the following scope and fixed assumptions: 

- **Scope:** The forensic analysis is confined to the extraction of artifacts found on a host workstation having Windows or Linux operating system. Multi tenant, server based hosts, cloud based environments are out of our scope of work. These areas are highlighted as important areas for future work[29]. 

- **Assumptions:** This work assumes that the forensic investigator is already having the relevant permissions from the legal entities for full physical or logical acquisition of target systems memory and storage. The subject under investigation is not presumed to a highly capable state actor or a sophisticated individual employing advanced anti forensics techniques like full disk encryption, file shredding or using live bootable operating system. The impact of these mentioned techniques are discussed in section 9. Our 

5 

_3.3 Legal and Ethical Framework_ 

primary objective is to recreate the suspect who is under investigation for interactions with the LLM software. 

### **3.3 Legal and Ethical Framework** 

The novel nature of locally deployed language models and corresponding evidence requires deep and careful planning of legal and ethical principles to ensure that integrity and admissibility of the evidence remains unquestionable. 

- **Chain of Custody:** The investigator must keep the chain of custody maintained for each and every digital evidence. The investigator must record all the steps from the very (image acquiring) till the final analysis step of all the collected artifacts. This must be maintained to show that all the evidence is untampered [31, 32]. 

- **Evidence Admissibility:** Artifacts extracted from local LLMs must adhere to the established standards for reliability, such as the _Daubert_ standard in U.S. federal courts[9]. Key _Daubert_ factors involve whether the technique described can be tested/re tested, has prior identified and calculated error rates, has been peer reviewed and is accepted in the scientific community. The steps described in this work are reproducible and have quantitative results. This work itself has peer review nature which helps in satisfying these criteria. The recently proposed Federal Rule of Evidence 707, dealing with machine generated evidences, underscoring the need to demonstrate the strength of the forensic process that produced the output[33, 34]. The analysis of the configuration files and associated model metadata helped in identifying and laying out the "process" that resulted in generating a particular AI response. 

- **Privacy:** The recovery of a particular plaintext prompt is often associated with deep privacy concerns[22]. The chat logs can often have sensitive personal, medical, financial or even proprietary information that was given by the user under the impression of the communication being confidential. The investigator looking for evidence of a particular crime may come across such a scenario. The complication arising from this stems from the disruption of the "plain view" doctrine and puts an ethical obligation on the investigator to ensure extreme care in managing the data. The scope of analysis must remain strictly within the scope of the original obtained legal warrant[36]. 

## **4 Targeted Software and Deployment Scenarios** 

To ensure that the methodology presented here is reproducible and valid, this work uses the below specific versions of the software under study. They were installed on a clean, controlled virtual machine environments. The architectural 

6 

_4.1 Ollama_ 

differences present in these tools result in fundamental differences in the types and locations of forensic artifacts produced. 

Table 1: Experimental Software and Environment Configuration 

|**?**__`tab:software-env`__**?**<br>Software|Version|Operating System|Installation<br>Date|Installation Method|
|---|---|---|---|---|
|Ollama|v0.11.8|Windows 11 Pro<br>(23H2)|2025-08-29|`https:`<br>`//ollama.com/install.sh` | sh|
|Ollama|v0.11.8|Ubuntu 24.04 LTS|2025-08-29|`https:`<br>`//ollama.com/install.sh` | sh|
|LM Studio|0.3.24|Windows 11 Pro<br>(23H2)|2025-08-29|LM-Studio-setup-0.2.22.exe|
|LM Studio|0.3.24|Ubuntu 24.04 LTS|2025-08-29|LM_Studio-0.2.22.AppImage|
|llama.cpp|b6316<br>(Git)|Windows 11 Pro<br>(23H2)|2025-08-29|Download from Releases section|
|llama.cpp|b6316<br>(Git)|Ubuntu 24.04 LTS|2025-08-29|Download from Releases section|

### **4.1 Ollama** 

Ollama [2] is an easy to use opensource framework that enables easy and quick deployment of Large language models (LLMs). It is written in the Go programming language and it hides all complications associated with running a Large Language Model, dependencies and other configuration issues by providing a simple abstract interface to the user to interact with the language model. Central to Ollama is a client server architecture that has a server using the llama.cpp library to deploy a language model in a highly optimized manner on the CPU and GPU (if available). Ollama also provides a Command Line Interface (CLI) that interacts with this server. Multiple Graphical User Interfaces (GUI) are also available that connect with the Ollama server. Each Ollama model is associated with a modelfile whose functionality is to provide users with a way to customize and create newer models by defining or redefining network parameters such as temperature, top_p and system prompts. These models are not the exact complete models, instead they are Quantized versions of the corresponding original models. Quantization is a process in which the numerical precision of a model is reduced to decrease the models size and computational requirements. Different levels of quantization results in different sized models, with a smaller sized quantized variant being lesser capable than a quantization variant with a larger size. With these optimizations sophisticated models are able to run on consumer grade hardware directly. Ollama also provides a full REST API to enable seamless integration with different applications. All the data remains on the system that hosts the server including the models. This design makes it an important and ideal tool of deployment by users for offline purposes. 

Digital artefacts related with an Ollama installation are as follows: 

7 

_4.2 LM Studio_ 

- Since Ollama uses a client server model the language model is exposed as an API on the local network port (default 11434)[27]. All the interaction with the model happens via this port only[38]. 

- **Installation Footprint:** 

- ** Linux:** The official installation script (install.sh) makes a separate user named ollama (a system user) and a corresponding systemd service. The main Ollama executable binary is in /usr/local/bin, the model data is kept at /usr/share/ollama[37] and is available system wide. User specific data including the downloaded models and logs are stored by default in the users home directory at ~/.ollama [40]. 

   - **Windows:** The installer binary puts the application files in the current users local application data folder located at C:\Users\<username>\AppData\Local\ollama [43, 46, 47]. The models and logs are kept at C:\Users\<username>\.ollama [39, 40]. The user can also install the application at a custom location by using the command line flag (/DIR=) to give the installation directory [41, 42, 44, 45, 48]. 

### **4.2 LM Studio** 

LM Studio is another alternative desktop application built upon the Electron framework[27]. Electron bundles a web based user interface typically made using HTML, CSS, JavaScript along with a backend process into a single executable. In the case of LM Studio a llama.cpp inference engine is included. LM Studio provides GUI based model discovering, downloading and chatting. It also provides an OpenAI compatible API server for programmatic access[28, 56, 57]. LM Studio also supports inferencing via GGUF model files. 

- **Installation Footprint:** 

- ** Linux:** The software is distributed as an .AppImage file. An AppImage file is a self sufficient way to distribute software. It includes all the required files and libraries that are mounted when the application is executed. When executed, there is no traditional installation. The required data directories are created within the users home folder. Major forensic artifacts are present in ~/.lmstudio/ and ~/.config/LM Studio[49, 50, 51, 53]. 

   - **Windows:** Like typical softwares, a standard .exe installer is provided to install. It has the vast majority of its operational data, including the crucial model cache and conversation logs, under the users profile at %USERPROFILE%\.cache\LM Studio\ and %USERPROFILE%\.lmstudio\[54]. 

### **4.3 llama.cpp** 

llama.cpp is a community driven open source C++ software library built for extremely efficient, local inference of large language models (originally built 

8 

for LLaMA but now supports almost every model. It exploits the GGML tensor library (also open source) that performs highly optimized computation based on the hardware (including CPUs and GPUs). The framework supports quantization levels (from 1.5 bit to 8 bit) enabling large models to be deployed on systems with a not so high configuration (often as low as 6GB RAM). llama.cpp performs text tokenization, inferencing using next token sampling and finally detokenization via the language model (in GGUF file format [70, 71, 72]), which bundles weights, tokenizer and metadata for quick loading and deployment. Llama.cpp also features real time token streaming, hybrid CPU+GPU inference, speculative decoding for speed and OpenAI compatible APIs. All these features make them the ideal solution for privacy focused, offline deployment without external dependencies like Python or CUDA frameworks. 

- **Architecture:** Written in C/C++, originally llama.cpp is not intended to be used directly as a user facing application. Instead it is meant to act like a command line based tool that is supposed to have some kind of front end like Ollama or LM Studio. The core design focuses on highly optimized performance LLM text inference with minimal dependencies[5]. Llama.cpp is distributed as source code and meant to be compiled, however compiled binaries are also released[24]. 

- **Installation Footprint:** There is no standard installation path. The tool and all the corresponding files (main, llama-cli) are present wherever the user downloaded and compiled the source code repository[5]. Forensically interesting artifacts are created in the same directory from which the tool is executed. The model files in the GGUF format, are usually placed by the user in a manually created models subdirectory within the project folder[5]. The absence of a standardized footprint causes difficulties for investigators. 

## **5 Methodology** 

We used a detailed, rigorous and forensically sound procedure to identify, generate and then analyze digital artifacts from the selected softwares. The process is designed to be reproducible so that the findings can be reproduced and verified which is very important for legal admissibility[9]. 

### **5.1 Forensic Acquisition** 

To establish a clean baseline environment and ensure proper and clear artifacts attribution, a differential analysis approach was used to compare before and after states. 

- **Disk Imaging:** For both the operating systems (Windows 11, Ubuntu 24.04) we made a base virtual machine (VM). A complete bit by bit disk image of this base state was made using dd for Linux and FTK 

9 

_5.2 Instrumentation and Data Generation_ 

Imager for Windows. Then the target software was installed and specific models were downloaded. After this a second disk image was captured. These comparative images allowed for precise identification of all files and identifying the system changes introduced by the software. 

- **Memory Acquisition:** Volatile memory or the RAM (Random Access Memory) often has artifacts that will typically not be written back to the disk. Examples include the user given in memory prompts or transient configuration data. While chatting with the LLM sessions, live memory captures were simultaneously performed using industry standard tools: Linux Memory Extractor (LiME) for Linux and WinPmem for Windows. 

- **OS Artifact Collection:** Standard host based artifacts were acquired to correlate the findings from the application specific data. This included shell history files (.bash_history, .zsh_history), PowerShell console history, Windows Prefetch files (.pf) and the Application Compatibility Cache (Shimcache). 

### **5.2 Instrumentation and Data Generation** 

To analyze and map the behavior of each application and to ensure that a consistent set of evidentiary data is generated across the experiments, we used a combination of system monitoring and scripted interactions. 

- **Process Monitoring:** System level tracing tools were used to monitor file system, registry (on Windows) and process activity during installation, model downloads and chat sessions. Process Monitor (ProcMon) from Sysinternals was used on Windows, while strace was used on Linux. These tools gave a real time log of every file read, written and modified by the applications. 

- **Network Capture:** For network traffic monitoring we used Wireshark to capture all the passing network traffic from the test VMs. This helped us to analyze and know telemetry, update checks, model download communications or any other network activity of forensic interest[38]. 

- **Controlled Data Generation:** A predecided, fixed script of user interactions was performed on each platform and tool. This included downloading specific models, running a series of ten distinct prompts and then deleting some of the generated artifacts. The prompts included benign questions ("How to make cake?"), requests for code generation ("Write a Python script to list files in a directory") and the inclusion of unique keywords (e.g., "FORENSIC_KEYWORD_12345") to facilitate later searching and data carving. All actions were properly recorded in a timestamped experiment log. 

10 

_5.3 Analysis Procedures_ 

### **5.3 Analysis Procedures** 

The collected data was then analyzed using a combination of tools. 

- **Integrity Verification:** All acquired disk images and key evidence files were hashed using the SHA 256 algorithm. These hashes were verified throughout the analysis process to ensure data integrity and maintain a valid chain of custody. 

- **Forensic Tooling:** Analysis of the disk images was conducted using leading open source digital forensic platform Autopsy. These tools were used for file system navigation, keyword searching and carving for deleted files. 

- **Specialized Analysis:** For application related artifacts case to case based specialized tools were used. SQLite databases were examined using the Foxton SQLite Examiner which can parse freelists and Write Ahead Logs (WAL) to recover deleted or uncommitted records[58]. Custom Python scripts were developed using the GGUF library to parse the metadata and structure of GGUF model files[60]. JSON formatted chat logs were parsed programmatically to extract conversations and metadata. 

- **Chain of Custody:** A formal chain of custody log was maintained for all evidentiary items. This document recorded every individual who handled the evidence, the date and time of transfer and the actions performed, adhering to best practices to ensure the evidences admissibility[31]. 

## **6 Artifact Analysis** 

This section presents the core findings. A detailed breakdown of the forensic artifacts created by Ollama, LM Studio and llama.cpp is given with the locations. The architectural differences between these tools result in distinct evidentiary footprints, with varying levels of richness, persistence and ease of recovery. For each artifact, we detail its location, format, forensic value and volatility. 

### **6.1 Ollama Artifacts** 

Ollamas client server architecture creates a centralized set of artifacts inside a hidden _.ollama_ directory in the users profile. Ollama is also distributed as a dockerized container for rapid deployment. In case of a container based deployment the _.ollama_ directory of the container is stored in the _/root_ directory. It is mapped to the _/var/lib/docker/volumes/ollama/_data_ location on the host system. [6] 

- **Model Manifests** 

** Location:** /.ollama/models/manifests/registry.ollama.ai/library/<model>/<tag> 

** Format:** JSON. 

11 

_6.1 Ollama Artifacts_ 

   - **Forensic Value:** These files are critical for proving which specific models and versions a user has downloaded. Each manifest contains metadata about the model, including a list of SHA 256 hashes corresponding to the models layers (blobs). This allows an investigator to confirm the exact composition of a model on the system[27]. 

   - **Volatility:** Persistent. These files remain on disk until the model is explicitly removed via ollama rm. 

- **Model Blobs (Layers)** 

** Location:** ~/.ollama/models/blobs/sha256-<hash> 

   - **Format:** Binary data. 

   - **Forensic Value:** These are the actual data layers of the LLMs. While their content is not human readable, their presence, verified by matching their SHA 256 hash with a manifest file, proves that a specific model was present on the machine. Hashing these files can be part of a signature based detection strategy[27]. 

   - **Volatility:** Persistent. Blobs are content addressable and may be shared across multiple models. They are only deleted when no manifest references them. 

- **Server Logs** 

   - **Location:** Default: ~/.ollama/logs/server.log. This can be redirected by the user at runtime (e.g., ollama serve > /path/to/logfile.log 2>&1)[61, 62, 63]. 

   - **Format:** Plain text. 

   - **Forensic Value:** Highly valuable for reconstructing a timeline of activity. Logs can contain timestamps for server startup/shutdown, model loading events, API requests from clients and, if verbose logging is enabled (OLLAMA_DEBUG=true), potentially the full text of user prompts and model responses[27]. 

   - **Volatility:** Semi persistent. The log file can be easily deleted by the user. Its location can be changed, making it harder to find. 

- **CLI History** 

** Location:** ~/.ollama/history 

** Format:** Plain text, one entry per line. 

- **Forensic Value:** Provides direct, plaintext evidence of user prompts entered via the ollama run command. This is a crucial artifact for understanding user intent. However, it does not capture interactions made through the API or third party GUI clients. 

- **Volatility:** Persistent, but only captures one mode of interaction and can be deleted. 

12 

_6.2 LMStudio Artifacts_ 

####  **Configuration** 

- **Location:** No single configuration file. Configuration is primarily managed through environment variables (e.g., OLLAMA_MODELS, OLLAMA_HOST) set in shell profiles (.bashrc, .zshrc) or, on Linux, in the systemd service file (/etc/systemd/system/ollama.service)[40]. 

- **Format:** N/A (environment variables, .ini style service files). 

- **Forensic Value:** Critical for identifying non default configurations. An OLLAMA_MODELS variable will point to a custom storage location for models, which an investigator must examine. An OLLAMA_HOST variable might indicate the server was bound to a public network interface. 

- **Volatility:** Persistent. 

### **6.2 LMStudio Artifacts** 

As a feature rich Electron application, LM Studio creates the most structured and comprehensive set of forensic artifacts, making it the most forensically revealing of the tools analyzed. The most important file is ~.lmstudio-homepointer, which is a small text file created by LM Studio application. It stores the absolute path to the applications home data directory which is ~ /.lmstudio or ~ /.cache/lm-studio on Linux/Mac, or ~%USERPROFILE%\.lmstudio on Windows. Inside this folder the following artefacts are stored. 

####  **Chat History** 

** Location:** `~/.lmstudio/conversations/<session_id>.json` (macOS / Linux) and `%USERPROFILE%\.lmstudio\conversations\<session_id>.json` (Windows). 

- **Format:** Structured JSON. 

- **Forensic Value:** This is the crown jewel artifact. Each JSON file represents a single chat session and contains a complete, timestamped record of the conversation, including user prompts, AI responses, the model used, and configuration presets applied[64, 65, 66]. Analysis of these files allows for a near-perfect reconstruction of the users interactions. The format appears to be an internal data structure but is human-readable and programmatically parsable[64]. Importantly, the `<session_id>` component of the filename is a **Unix epoch timestamp encoded in milliseconds** , providing a precise creation timestamp for each session that can be correlated with external artefacts such as proxy logs, browser history, or Windows Event Logs during a DFIR investigation[64]. 

- **Volatility:** Persistent. These files remain until manually deleted. Due to their structured nature, they are highly recoverable from unallocated space. Note that LM Studio does not delete model folder 

13 

_6.2 LMStudio Artifacts_ 

remnants from the filesystem when a model is removed via the UI, so orphaned session directories may persist even after apparent user clean-up[36]. 

- **Model Cache** 

   - **Location:** The correct dual-path structure (Windows example) is: 

      - (or a user-configured alternative path) 

The equivalent on macOS / Linux replaces `%USERPROFILE%\` with `~/` . (Path varies slightly by OS, see Section 4.2.) 

   - **Format:** GGUF model weight files within a nested `<publisher>/<repo-name>` directory structure; the hub sub-tree contains small JSON manifests and configuration shards alongside the weight files. 

   - **Forensic Value:** The directory path itself ( `<publisher>/<repo-name>` ) provides valuable metadata about the models origin from the Hugging Face Hub, even if the user renames the `.gguf` file[54]. The split introduced in v0.3.16 means an investigator must examine _both_ sub-trees; hub metadata directories may persist on disk even after the GGUF weight file has been deleted[36]. 

   - **Volatility:** Persistent. 

- **Configuration Presets** 

   - **Location:** `~/.lmstudio/hub/presets/` (synced / community presets) or `~/.lmstudio/config-presets/` (user-defined presets). A drafts sub-directory at `~/.lmstudio/.internal/config-presets-drafts/` stores in-progress or unsaved preset configurations. 

   - **Format:** JSON ( `.preset.json` ). 

** Forensic Value:** These files store user-defined or downloaded model configurations, such as the system prompt, temperature, context length, and GPU offload settings[52]. They reveal how a user tailored a models behaviour for specific tasks, which can be indicative of intent. The presence of drafts in `config-presets-drafts/` may expose experimental or discarded configurations not visible in the main UI. 

   - **Volatility:** Persistent. 

- **Application and Server Logs** 

   - **Location:** Two distinct log sources exist: 

      1. **Persistent server logs:** `~/.lmstudio/server-logs/YYYY-MM/` : automatically written per calendar month. 

14 

_6.2 LMStudio Artifacts_ 

      2. **Live inference stream:** accessible via the CLI command `lms log stream` [30]: volatile unless redirected to a file. 

   - **Format:** Persistent logs are timestamped, structured text files; the live stream is a real-time text output[30]. 

   - **Forensic Value:** The persistent server logs record every API request served by LM Studios local HTTP server, including endpoint calls ( `/v1/chat/completions` , `/v1/models` , etc.), timestamps, model load/unload events, and client IP addresses if the server was accessed from other devices. The month-partitioned directory structure provides a direct timeline of application activity. The live inference stream reveals the _exact_ , fully formatted prompt sent to the inference engine _after_ prompt templating has been applied, which may differ from the raw user input recorded in the chat history  this distinction is significant for forensic reconstruction of model instructions[30]. 

   - **Volatility:** Server logs are **persistent** and month-partitioned. The live stream is highly volatile and is not persisted unless the user explicitly redirects output to a file. 

- **RAG Pipeline Cache** 

#### ** Location:** 

      -  `~/.lmstudio/.internal/retrieval-sessions/` : active RAG session state 

      -  `~/.lmstudio/.internal/cached-rag-pipeline-chunks/` : chunked and vectorised document representations 

      -  `~/.lmstudio/.internal/parsed-documents-cache/` : raw text extracted from uploaded files (PDF, DOCX, etc.) 

   - **Format:** Internal binary or serialised vector format (chunks); parsed document cache may contain extractable plain text. 

   - **Forensic Value:** When a user employs the Chat with Documents feature (Retrieval Augmented Generation, RAG [74]), LM Studio processes and caches documents across these three directories[55]. Analysis can reveal fragments or full copies of external documents the user was interacting with, even if the original documents have been deleted. The `parsed-documents-cache/` sub-directory is particularly valuable as it may contain human-readable extracted text. The bundled embedding model used for vectorisation ( `nomic-embed-text-v1.5-GGUF` , stored at `.internal/bundled-models/nomic-ai/` ) provides context for interpreting the chunk format. 

   - **Volatility:** Semi-persistent. It is a cache that can be cleared, but often persists across sessions and can grow to significant size with heavy document use. 

- **API Prediction History** 

15 

_6.2 LMStudio Artifacts_ 

- **Location:** `~/.lmstudio/.internal/api-prediction-history/packs/` 

- **Format:** Binary pack files; individual pack files can exceed 500 MB under heavy usage[10]. 

- **Forensic Value:** This directory captures every inference request processed by LM Studio, including _programmatic_ API calls made by external scripts, `curl` commands, or Python SDKs  artefacts that are _not_ recorded in the chat UIs conversation history. This makes it an indispensable source for detecting automated or scripted model usage beyond the graphical interface. 

- **Volatility:** Persistent. Pack files accumulate over time. A corrupted or oversized pack file is known to cause HTTP 500 errors on the local API server, which itself is a forensic indicator of sustained, high-volume API usage[10]. 

####  **Credentials Store** 

#### ** Location:** `~/.lmstudio/credentials/` and `~/.lmstudio/.internal/lms-key-2` [12] 

** Format:** Key files; may be stored in plaintext or lightly encoded form. The `lms-key-2` file stores the CLI authentication key used for LM Studio Hub access[12]. The `credentials/` directory may additionally contain tokens for integrated external services such as Hugging Face. 

- **Forensic Value:** Highest-sensitivity artefacts in the directory tree. These files should be examined for plaintext or base64-encoded secrets. Their presence confirms the user authenticated with LM Studio Hub or an external model repository. The `lms login` command uses asymmetric key pairs ( `key-id` , `public-key` , `private-key` ) for CI-style authentication, meaning key material may reside here in exportable form[35]. 

- **Volatility:** Persistent. Credentials persist until the user explicitly logs out or deletes the files. 

####  **User-Uploaded Files** 

** Location:** `~/.lmstudio/user-files/` 

- **Format:** Original file formats as uploaded by the user (PDF, TXT, DOCX, source code, etc.). 

- **Forensic Value:** Contains files the user attached to chat sessions or fed into the RAG pipeline. The presence and modification timestamps of files here directly corroborate when external documents were introduced into the models context. These files may persist even after the originating chat session is deleted. 

- **Volatility:** Persistent. 

16 

_6.3 llama.cpp Artifacts_ 

### **6.3 llama.cpp Artifacts** 

By design, llama.cpp is a minimalist tool and consequently, its forensic footprint is the most ephemeral and challenging to analyze. 

- **Model Files** 

** Location:** User defined. Typically in a ./models/ subdirectory relative to the executable[5]. 

** Format:** GGUF. 

- **Forensic Value:** The presence of .gguf files is the primary indicator that LLM activity may have occurred. Analysis of the GGUF file itself is the main source of evidence (see 6.4). 

** Volatility:** Persistent. 

- **Command Line History** 

** Location:** Standard shell history files (~/.bash_history, ~/.zsh_history, PowerShell history). 

   - **Format:** Plain text. 

   - **Forensic Value:** This is the most critical artifact for llama.cpp. The full command line used to launch llama-cli or main contains the path to the model, all generation parameters (temperature, top-p, etc.) and often the initial prompt itself (if passed with the -p flag)[23]. 

   - **Volatility:** Highly volatile. Shell history is often limited in size, can be disabled, or can be easily cleared by a user. 

- **Memory Artifacts** 

   - **Location:** System RAM. 

   - **Format:** Raw memory strings. 

   - **Forensic Value:** During execution, the prompt text, model weights and generated output exist in the processs memory space. A live memory capture or analysis of a pagefile/swap file may be the _only_ way to recover a prompt that was not logged or passed via the command line (e.g., in interactive mode)[67, 68]. 

   - **Volatility:** Extremely volatile. Lost upon process termination or system shutdown. 

### **6.4 Cross Cutting Artifacts (GGUF and SQLite)** 

Two file formats are common across the local LLM ecosystem and warrant special attention. 

- **GGUF File Analysis:** The GGUF format is central to llama.cpp and the models used by Ollama and LM Studio. Using Python libraries like gguf and pygguf, an investigator can parse these binary files[60]. 

17 

   - **Forensic Value:** The GGUF header and metadata section contain a wealth of information[25]. This includes the models architecture (e.g., llama, qwen), parameter count, quantization level (e.g., Q4_K_M), context length, embedding length and the full tokenizer vocabulary. This data can be used to precisely fingerprint a model and understand its capabilities, which is crucial for verifying if a given output could have been produced by a specific model file found on a system. 

- **SQLite Database Analysis:** While not used by the core applications in their default state, many popular front ends and related tools in the ecosystem use SQLite databases for storing user settings, chat histories, or vector embeddings for RAG[73]. 

   - **Forensic Value:** Standard forensic techniques for SQLite are highly applicable [76]. Tools can analyze the main database file, but also the rollback journal (-journal) and Write-Ahead Log (-wal) files to recover transient data. Furthermore, carving for deleted records within the database files freelists and unallocated space can recover previously deleted chat messages or configuration settings, providing evidence a user thought they had removed[59]. 

The architectural design of each tool client server for Ollama, monolithic Electron for LM Studio and minimalist CLI for llama.cpp fundamentally dictates the nature and persistence of the evidentiary trail. This creates a clear hierarchy of forensic richness. User friendly applications like LM Studio, designed for convenience, generate more structured and persistent artifacts. In contrast, the ephemeral traces left by a command line tool like llama.cpp make it more difficult to investigate post facto. This suggests that a suspects choice of tool can itself be an indicator of their technical sophistication and potential awareness of forensic countermeasures, a "meta artifact" that can inform the overall investigative strategy. 

## **7 Experiments and Results** 

To validate and quantify the findings from our artifact analysis, we conducted a series of controlled experiments. These experiments were designed to simulate common investigative challenges: reconstructing user interactions and determining the persistence of evidence. The results provide objective metrics on the forensic utility of the identified artifacts. 

### **User Interaction Reconstruction** 

This experiment aimed to determine the extent to which a users conversation with an LLM could be recovered from disk based artifacts after a normal system shutdown. 

18 

- **Experimental Setup:** On each of the six test environments (3 tools x 2 OSes), we executed a standardized script of 10 prompts. The prompts ranged from simple questions to code generation requests and contained unique keywords. After the session, the VM was shut down cleanly. 

- **Procedure:** The "after" disk image of each VM was mounted and analyzed. We searched for the primary prompt/chat history artifacts identified in Section 6: ~/.ollama/history for Ollama, ~/.lmstudio/conversations/*.json for LM Studio and shell history files for llama.cpp. 

- **Results:** 

- ** LM Studio:** For both the operating systems, 100% of the 10 prompts and their corresponding model responses were recovered with full fidelity from the JSON chat logs. Timestamps, model identifiers and session configurations were also fully intact. 

   - **Ollama:** On both the OSes, 100% of the prompts entered via the ollama run CLI were recovered from the ~/.ollama/history file. However, this file only contains the users input, not the models output. Prompts sent via an API client were not logged in this file. 

   - **llama.cpp:** Recovery was dependent on the shell. On Linux 100% of the commands, including prompts passed with the -p flag, were recovered from .bash_history or .zsh_history. On Windows, PowerShell history successfully captured the commands. In interactive mode (-i), no prompts were logged to the shell history, resulting in 0% recovery from disk. This highlights the critical importance of memory forensics for llama.cpp. 

deletion, simulating a non expert users attempt to cover their tracks. 

### **7.1 Summary of Forensic Artifacts** 

The following table synthesizes the findings from our analysis and experiments, providing a comprehensive, at a glance reference for forensic practitioners. It ranks artifacts based on their forensic value and persistence, helping investigators prioritize their efforts during an examination. 

**Table 2: Summary of Key Forensic Artifacts by Product and Operating System** 

|**Product**|**OS**|**Artifact Category**|**Default Path (User Profl**|
|---|---|---|---|
|**Ollama**|All|Model Manifests|~/.ollama/models/manifests/|
|All|Model Blobs|~/.ollama/models/blobs/|Binary|
|All|CLI History|~/.ollama/history|Plain Text|
|All|Server Logs|~/.ollama/logs/server.log|Plain Text|
|Linux|Confguration|/etc/systemd/system/ollama.service|INI|
|**LM Studio**|All|**Chat History**|~/.lmstudio/conversations/|

19 

|All|Model Cache|~/.lmstudio/models/|GGUF|
|---|---|---|---|
|All|Confg Presets|~/.lmstudio/confg-presets/|JSON|
|All|RAG Cache|~/.lmstudio/.session_cache|Binary|
|All|Application Logs|N/A (Live Stream)|Text Stream|
|**llama.cpp**|All|CLI History<br>|Shell History (.bash_history, etc.)|
|All|Model Files|User defned (e.g., ./models/)|GGUF|
|All|Memory|System RAM / Pagefle|Raw Strings|

Table 3: Summary of Key Forensic Artifacts by Product and Operating System 

**?** __ `tab:forensic-artifacts` __ **? Product OS** 

|**Product **|**OS**|**Artifact**<br>**Category**|**Default Path**<br>**(User Profle**<br>**Relative)**|**Format**|**Persistence**|**Forensic**<br>**Value**<br>**(15)**|**Remarks**|
|---|---|---|---|---|---|---|---|
|**Ollama**|All|Model<br>Manifests|~/.ollama/models/man<br>|ifests/<br>JSON<br>|Persistent|4|Proves which<br>models/versions were<br>downloaded.|
||All|Model Blobs|~/.ollama/models/blob|s/Binary|Persistent|3|Confrms presence of<br>model layers via<br>hashing.|
||All|CLI History|~/.ollama/history|Plain<br>Text|Persistent|5|Plaintext record of<br>`ollama run` prompts.|
||All|Server Logs|~/.ollama/logs/server.l|ogPlain<br>Text|Semi-<br>Persistent|4|Records server activity;<br>can be<br>redirected/deleted.|
||Linux|Confguration|/etc/systemd/system/|ollama.serv<br>INI|ice<br>Persistent|3|Reveals non-default<br>paths or network<br>settings.|
|**LM**<br>**Studio**|All|**Chat**<br>**History**|~/.lmstudio/conversati|ons/<br>**JSON**|**Persistent **|**5**<br>**(Critical)**|**Complete,**<br>**timestamped user/AI**<br>**conversation logs.**|
||All|Model<br>Cache|~/.lmstudio/models/|GGUF|Persistent|4|Stores models; path<br>reveals Hugging Face<br>origin.|
||All|Confg<br>Presets|~/.lmstudio/confg-<br>presets/|JSON|Persistent|4|Shows user-defned<br>model parameters and<br>intent.|
||All|RAG Cache|~/.lmstudio/.session_c|ache<br>Binary|Semi-<br>Persistent|4|Contains fragments of<br>documents used in RAG.|
||All|Application<br>Logs|N/A (Live Stream)|Text<br>Stream|Volatile|5|`lms log stream` shows<br>fnal formatted prompt.|
|**llama.cp**|**p**All|CLI History|Shell History<br>(`.bash_history`, etc.)|Plain<br>Text|Volatile|5<br>(Critical)|Often the only record of<br>prompts and<br>parameters.|
||All|Model Files|User-defned (e.g.,<br>`./models/`)|GGUF|Persistent|3|Proves presence of<br>models; metadata is key.|
||All|Memory|System<br>RAM / Pagefle|Raw<br>Strings|Extremely<br>Volatile|5<br>(Critical)|May be the only source<br>for interactive mode<br>prompts.|

_Note: ~ refers to the users home directory (/home/<user> on Linux and C:\Users\<user> on Windows)._ 

20 

## **8 Discussion** 

Our findings have significant implications for digital forensic investigators, legal professionals and software vendors. The massive adoption of local LLMs has created a new and complex evidentiary landscape that poses multiple challenges to traditional investigative methods while simultaneously offering unprecedented insight into user intent. This section discusses these implications, explores potential anti forensic techniques and countermeasures and provides recommendations for stakeholders. 

### **8.1 Implications for Investigators** 

Our analysis demonstrates that local LLMs are a double edged sword for digital forensics. On one hand, they represent a new vector for malicious activity that can be conducted offline, away from the purview of network monitoring. On the other hand, when artifacts are present, they provide an exceptionally rich source of evidence regarding a users state of mind, intent and actions. 

The primary takeaway is that the forensic strategy must be tailored to the specific tool in use. The architectural differences between Ollama, LM Studio and llama.cpp are not trivial and they create a clear hierarchy of evidentiary persistence. An investigator examining a system with LM Studio can expect to find structured, persistent chat logs that are relatively easy to parse and recover[64]. In contrast, an investigation involving llama.cpp may yield no persistent prompt history on disk, making live memory acquisition and analysis of shell history paramount[23]. Investigators must therefore be trained to first identify the specific runner being used and then apply the appropriate analytical workflow as outlined in this paper. 

### **8.2 Anti Forensic Risks and Countermeasures** 

A forensically literate user can take several steps to obstruct or evade analysis of their local LLM activity. 

- **Anti Forensic Techniques:** 

- ** Path Obfuscation:** A user can set the OLLAMA_MODELS environment variable or use LM Studios settings to store models and data on an external or encrypted volume, evading searches of default directories[40]. 

   - **Ephemeral Execution:** Tools like llama.cpp or Ollama can be run from a temporary directory or a live USB, leaving minimal traces on the host machines primary storage. 

   - **Logging Evasion:** Ollamas server logging can be disabled or redirected to /dev/null on Unix like systems. For LM Studio, a user can simply delete the JSON chat logs from the cache directory. 

21 

_8.3 Vendor Recommendations and Future Work_ 

   - **Tool Selection:** As our findings show, a sophisticated actor would likely choose llama.cpp over LM Studio precisely because it generates fewer persistent artifacts. 

- **Investigative Countermeasures:** 

- ** Signature Based Detection:** The YARA rules and triage commands can be developed to find artifacts regardless of their location. Searching for GGUF file headers (GGUF) across an entire disk image can identify model files even in non standard paths[77, 78, 79]. 

   - **Memory Forensics:** Live memory analysis remains the most effective countermeasure against ephemeral execution and interactive prompts. Searching a memory dump for strings related to model loading or prompt templates can reveal activity that was never written to disk. 

   - **System Level Artifacts:** Even if application level logs are deleted, OS artifacts can provide crucial leads. Windows Prefetch files can show that lm studio.exe or ollama.exe was executed. Shell history can capture the commands used to launch llama.cpp or change environment variables. 

### **8.3 Vendor Recommendations and Future Work** 

To improve the forensic readiness of these tools, especially in corporate or regulated environments, we offer the following recommendations for vendors: 

- **Standardized, Structured Logging:** We recommend that vendors like Ollama and LM Studio implement a robust, standardized logging framework. This should be an opt in feature for enterprise or forensic use, creating a single, secure log file that records all key events: user authentication, model loading, API requests, full prompt text and generated responses, all with reliable timestamps. This would be invaluable for incident response. 

- **Log Integrity:** To prevent tampering, these logs could incorporate cryptographic integrity checks, such as chaining log entries with hashes, similar to a blockchain. 

Our work has several limitations that open avenues for future research. We only partially investigated containerized deployments (Ollama running in Docker), which introduces layers of abstraction that complicate forensic analysis[29, 80]. The forensic traces left by model finetuning and the use of Retrieval Augmented Generation (RAG) with large, external knowledge bases also warrant dedicated study[81, 82]. Finally, the growing ecosystem of third party web front ends for these tools (Open WebUI [75]) creates additional artifacts that need to be cataloged. 

22 

_8.4 Privacy and Admissibility in the Age of Local AI_ 

### **8.4 Privacy and Admissibility in the Age of Local AI** 

The ability to recover a users complete, verbatim interactions with an LLM raises profound legal and ethical questions. These prompt histories can be more revealing than a private diary, capturing a users brainstorming, sensitive queries and unrefined thoughts. This places a heavy burden on the legal system to balance the needs of an investigation with an individuals right to privacy[22]. The specificity of search warrants, as discussed in Section 3.3, becomes non negotiable. 

Furthermore, for evidence derived from these systems to be admissible, it must be presented in a reliable and understandable manner. An investigator cannot simply present a generated text as evidence. It must be prepared to use the artifacts we have identified model manifests, GGUF metadata, configuration presets to explain the process by which that text was generated. This aligns with the principles of the _Daubert_ standard and the proposed FRE 707, which demand that the proponent of machine generated evidence demonstrate the validity of the underlying process[33]. Our research provides the first map to the artifacts needed to build that foundational argument. 

## **9 Conclusion** 

This paper has conducted the first systematic and empirical forensic analysis of the leading local Large Language Model runners: Ollama, LM Studio and llama.cpp. We have demonstrated that while these applications are designed with privacy and offline use in mind, they create a rich and varied trail of digital evidence across Windows and Linux platforms. The architectural choices of their developers have resulted in a clear hierarchy of forensic utility, with user friendly GUI applications like LM Studio producing highly structured and persistent artifacts, while minimalist command line tools like llama.cpp leave more ephemeral traces. 

Our research provides a foundational methodology for this new domain of digital forensics. We have established a comprehensive corpus of artifacts, from model caches and manifests to plaintext chat histories and configuration files. We have validated the forensic value of these artifacts through controlled experiments. 

The rise of local AI represents a paradigm shift and the digital forensics field must adapt accordingly. The techniques and findings presented in this paper are a critical first step, equipping investigators to pull back the curtain on these private AI environments. By integrating these methodologies into standard operating procedures, the forensic community can ensure that evidence from local LLM systems is identified, recovered and presented in a manner that is both scientifically rigorous and legally sound, upholding justice in an increasingly intelligent world. 

23 

_REFERENCES_ 

## **References** 

- `1` [1] LM Studio - Local AI on your computer. https://lmstudio.ai/ (Accessed: January 8, 2026). 

- `1.1` [2] Ollama. https://ollama.com/ (Accessed: January 8, 2026). 

- `1.2` [3] GitHub - ggml-org/llama.cpp: LLM inference in C/C++ (no date). https://github.com/ggml-org/llama.cpp (Accessed: January 8, 2026). 

- `1.3` [4] Mucci, T. (2025, November 17). GGUF versus GGML. IBM. https://www.ibm.com/think/topics/gguf-versus-ggml 

- `2` [5] How to Use llama.cpp to Run LLaMA Models Locally - Codecademy, accessed August 8, 2025, _https://www.codecademy.com/article/llama-cpp_ 

- <u>`3`</u> [6] Where Are Ollama Models Stored on Mac? - BytePlus, accessed August 8, 2025, _https://www.byteplus.com/en/topic/418089_ 

- `4` [7] Yin, Z., Wang, Z., Xu, W., Zhuang, J., Mozumder, P., Smith, A. and Zhang, W., 2025. Digital Forensics in the Age of Large Language Models. arXiv preprint arXiv:2504.02963. 

- <u>`5`</u> [8] Exploring the Potential of Large Language Models for Improving Digital Forensic Investigation Efficiency - ResearchGate, accessed August 8, 2025, 

- <u>`6`</u> [9] ForensicLLM: A local large language model for digital forensics | DFRWS, accessed August 8, 2025, _https://dfrws.org/wpcontent/uploads/2025/03/ForensicLLM-A-local-large-languagemod_2025_Forensic-Science-International-.pdf_ 

- <u>`7`</u> [10] A comprehensive study of Cybercrime and Digital Forensics through Machine Learning and AI | Al-Rafidain Journal of Engineering Sciences, accessed August 8, 2025, _https://rjes.iq/index.php/rjes/article/view/168_ 

- <u>`8`</u> [11] The Use of Machine Learning in Digital Forensics: Review Paper - Atlantis Press, accessed August 8, 2025, _https://www.atlantispress.com/article/125984186.pdf_ 

- <u>`9`</u> [12] AI-Enhanced Digital Forensics: Automated Techniques for Efficient Investigation and Evidence Collection | Journal of Electrical Systems, accessed August 8, 2025, _https://journal.esrgroups.org/jes/article/view/766_ 

- `10` [13] Using Micro-Services and Artificial Intelligence to Analyze Images in Criminal Evidences - DIGITAL FORENSIC RESEARCH CONFERENCE, accessed August 8, 2025, _https://dfrws.org/wpcontent/uploads/2021/09/2021-usa-paper-41-using_micro-_ 

   - _services_and_artificial_intelligence_to_analyze_images_in_criminal_evidences.pdf_ 

24 

_REFERENCES_ 

- `11` [14] Murtuza, Shariq, Robin Verma, Jayaprakash Govindaraj, and Gaurav Gupta. "A tool for extracting static and volatile forensic artifacts of windows 8. x apps." In _IFIP International Conference on Digital Forensics_ , pp. 305-320. Cham: Springer International Publishing, 2015. 

- `12` [15] The Future of Artificial Intelligence (AI) Applications in Forensics - RAIS Conferences, accessed August 8, 2025, _https://rais.education/wpcontent/uploads/2025/05/0523.pdf_ 

- `13` [16] Large Language Models in Modern Forensic Investigations: Harnessing the Power of Generative Artificial Intelligence in Crime Resolution and Suspect Identification - Zenodo, accessed August 8, 2025, _https://zenodo.org/records/14825697_ 

- `14` [17] Leveraging LLMs for Memory Forensics: A Comparative Analysis of Malware Detection, accessed August 8, 2025, 

- `15` [18] Murtuza, Shariq. "Scout: Leveraging Large Language Models for Rapid Digital Evidence Discovery." arXiv preprint arXiv:2507.18478 (2025). 

- `17` [19] Exploring the potential of large language models for author profiling tasks in digital text forensics - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/exploring-the-potential-of-largelanguage-models-for-author-profiling-tasks-in-digital-text-forensics/_ 

- `18` [20] Deep Reasoning and Large Context Windows: Next-Generation AI in Digital Forensic Investigations - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/deep-reasoning-and-large-contextwindows-next-generation-ai-in-digital-forensic-investigations/_ 

- `19` [21] ForensicLLM: A Local Large Language Model for Digital Forensics - DFRWS, accessed August 8, 2025, _https://dfrws.org/presentation/forensicllm-a-locallarge-language-model-for-digital-forensics/_ 

- `20` [22] The Ethical Implications of AI in Forensic Science, accessed August 8, 2025, _https://forensicscienceacademy.org/blog/f/the-ethical-implications-ofai-in-forensic-science_ 

- `21` [23] llama.cpp - Qwen - Read the Docs, accessed August 8, 2025, _https://qwen.readthedocs.io/en/latest/run_locally/llama.cpp.html_ 

- `22` [24] llama.cpp - Wikipedia, accessed August 8, 2025, _https://en.wikipedia.org/wiki/Llama.cpp_ 

- `23` [25] GPT-Generated Unified Format - NATO, accessed August 8, 2025, _https://nhqc3s.hq.nato.int/apps/DCRA_Report/id29d4122b072148f5aaf4882ecc5d963c/elements/id673b71b3cfc34434a68a55988f9a6354.html_ 

25 

_REFERENCES_ 

- `24` [26] What is GGUF? A Beginners Guide - Shep Bryan, accessed August 8, 2025, _https://www.shepbryan.com/blog/what-is-gguf_ 

- `25` [27] Analysis of Ollama Architecture and Conversation Processing Flow for AI LLM Tool, accessed August 8, 2025, _https://medium.com/@rifewang/analysis-of-ollama-architecture-andconversation-processing-flow-for-ai-llm-tool-ead4b9f40975_ 

- `26` [28] About LM Studio | LM Studio Docs, accessed August 8, 2025, _https://lmstudio.ai/docs_ 

- `28` [29] What is Container Forensics and Incident Response? - Sysdig, accessed August 8, 2025, _https://www.sysdig.com/learn-cloud-native/what-is-containerforensics-and-incident-response_ 

- `29` [30] Best practices for performing forensics on containers. | Google Cloud Blog, accessed August 8, 2025, _https://cloud.google.com/blog/products/containerskubernetes/best-practices-for-performing-forensics-on-containers_ 

- <u>`30`</u> [31] Best Practices for Maintaining Chain of Custody for Digital Evidence - Vidizmo, accessed August 8, 2025, _https://vidizmo.ai/blog/chain-of-custodyfor-digital-evidence_ 

- <u>`31`</u> [32] What is the Chain of Custody in Digital Forensics? - Champlain College Online, accessed August 8, 2025, _https://online.champlain.edu/blog/chaincustody-digital-forensics_ 

- <u>`32`</u> [33] Artificial Intelligence and the Law: Expert Witness Issues in 2025 - Forensis Group, accessed August 8, 2025, _https://www.forensisgroup.com/resources/expert-legal-witnessblog/artificial-intelligence-law_ 

- <u>`33`</u> [34] Jessica Kerbel and Leonard Dietzen, III, New AI Rule, Old Standard: Proposed Federal Rule of Evidence 707 Aims to Apply Daubert Standard to AI-Generated Evidence, _RumbergerKirk_ , June 20, 2025, accessed August 8, 2025, _https://www.rumberger.com/insights/new-ai-rule-old-standardproposed-federal-rule-of-evidence-707-aims-to-apply-daubert-standard-to-aigenerated-evidence/_ 

- <u>`35`</u> [35] Legal and Ethical Challenges in Digital Forensics Investigations - ResearchGate, accessed August 8, 2025, 

- <u>`36`</u> [36] AI & the courts: Judicial and legal ethics issues | National Center for State Courts, accessed August 8, 2025, _https://www.ncsc.org/resources-courts/aicourts-judicial-and-legal-ethics-issues_ 

- <u>`37`</u> [37] How to install Ollama for fine-tuning machine learning models efficiently - Hostinger, accessed August 8, 2025, _https://www.hostinger.com/tutorials/how-to-install-ollama_ 

26 

_REFERENCES_ 

- <u>`38`</u> [38] How to Easily Share LM studio API Online - Pinggy, accessed August 8, 2025, _https://pinggy.io/blog/lm_studio/_ 

- <u>`39`</u> [39] Linux installer default path  Issue #2361  ollama/ollama - GitHub, accessed August 8, 2025, _https://github.com/ollama/ollama/issues/2361_ 

- `40` [40] Exploring the Local Location of Ollama Models on WSL2  - dasarpAI, accessed August 8, 2025, _https://main--dasarpai.netlify.app/dsblog/exploringollama-models-location-on-wsl2/_ 

- `41` [41] How to change where OLLAMA models are saved on linux - Stack Overflow, accessed August 8, 2025, _https://stackoverflow.com/questions/79444743/how-to-change-whereollama-models-are-saved-on-linux_ 

- `42` [42] where is everything?  Issue #733  ollama/ollama - GitHub, accessed August 8, 2025, _https://github.com/ollama/ollama/issues/733_ 

- `43` [43] Unlock the Secrets of Ollamas File Structure for AI Mastery - Geeky Gadgets, accessed August 8, 2025, _https://www.geeky-gadgets.com/unlockthe-secrets-of-ollamas-file-structure-for-ai-mastery/_ 

- `44` [44] Ollama Models Location on macOS - BytePlus, accessed August 8, 2025, _https://www.byteplus.com/en/topic/418101_ 

- `45` [45] Change Ollama Model Directory - GitHub Gist, accessed August 8, 2025, _https://gist.github.com/duonghuuphuc/8339df20b2efcb55bb41941b896a5d8d_ 

- `46` [46] Where would I find the model Files on my Mac? - ollama - Reddit, accessed August 8, 2025, 

- `47` [47] File path of Models? : r/ollama - Reddit, accessed August 8, 2025, _https://www.reddit.com/r/ollama/comments/1cl1lxy/file_path_of_models/_ 

- `48` [48] Windows install path  Issue #2938 - GitHub, accessed August 8, 2025, _https://github.com/ollama/ollama/issues/2938_ 

- `49` [49] How to Install LM Studio - A Graphical Application for Running Large Language Models (LLMs) | Vultr Docs, accessed August 8, 2025, _https://docs.vultr.com/how-to-install-lm-studio-a-graphicalapplication-for-running-large-language-models-llms_ 

- <u>`50`</u> [50] How I install LM Studio 0.3.2 on Ubuntu Studio 24.04 linux | DimensionQuest - Burkes Blog!, accessed August 8, 2025, _https://dimensionquest.net/2024/09/how-i-install-lm-studio-0.3.2-onubuntu-studio-24.04-linux/_ 

- <u>`51`</u> [51] How to Install LM Studio to Run LLMs Offline in Linux, accessed August 8, 2025, _https://www.tecmint.com/lm-studio-run-llms-linux/_ 

27 

_REFERENCES_ 

- <u>`52`</u> [52] ~/.cache isnt a good place to put configuration files and binaries  Issue #230 - GitHub, accessed August 8, 2025, _https://github.com/lmstudioai/lmstudio-bug-tracker/issues/230_ 

- <u>`53`</u> [53] How to Install LM Studio on macOS: A Quick Guide, accessed August 8, 2025, _https://www.metriccoders.com/post/how-to-install-lm-studio-onmacos-a-quick-guide_ 

- <u>`54`</u> [54] Re-use already downloaded models? : r/LMStudio - Reddit, accessed August 8, 2025, _https://www.reddit.com/r/LMStudio/comments/18y9tnd/reuse_already_downloaded_models/_ 

- <u>`55`</u> [55] Session Cache  Issue #536  lmstudio-ai/lmstudio-bug-tracker - GitHub, accessed August 8, 2025, _https://github.com/lmstudio-ai/lmstudio-bugtracker/issues/536_ 

- <u>`63`</u> [56] How to run a local AI model on your computer with LM Studio | The Neuron, accessed August 8, 2025, _https://www.theneuron.ai/explainer-articles/howto-run-a-local-ai-model-on-your-computer-with-lm-studio_ 

- <u>`64`</u> [57] How to Enable External Access to LM Studio: A Complete Guide - Practical.kr, accessed August 8, 2025, _http://practical.kr/?p=848_ 

- <u>`65`</u> [58] SQLite Examiner - Free SQLite viewer software - Foxton Forensics, accessed August 8, 2025, _https://www.foxtonforensics.com/sqlite-database-examiner/_ 

- <u>`66`</u> [59] Forensic Analysis of SQLite Databases: Free Lists, Write Ahead Log, Unallocated Space and Carving - Belkasoft, accessed August 8, 2025, _https://belkasoft.com/sqlite-analysis_ 

- <u>`67`</u> [60] gguf  PyPI, accessed August 8, 2025, _https://pypi.org/project/gguf/_ 

- <u>`68`</u> [61] Logging your AI events (from Ollama) in Bronto, accessed August 8, 2025, _https://www.bronto.io/blog/logging-events-from-ollama_ 

- <u>`69`</u> [62] Ollama - ITS Documentation - University of Michigan, accessed August 8, 2025, _https://documentation.its.umich.edu/arc-software/ollama_ 

- <u>`70`</u> [63] Does anyone know how to change where your models are saved on linux? : r/ollama, accessed August 8, 2025, 

- <u>`71`</u> [64] Loading chat history from LM Studio?  Issue #152 - GitHub, accessed August 8, 2025, _https://github.com/lmstudio-ai/lmstudio.js/issues/152_ 

- <u>`72`</u> [65] Delete all history  Issue #80  lmstudio-ai/lmstudio-bug-tracker - GitHub, accessed August 8, 2025, _https://github.com/lmstudio-ai/lmstudio-bugtracker/issues/80_ 

- <u>`73`</u> [66] Importing and Sharing | LM Studio Docs, accessed August 8, 2025, _https://lmstudio.ai/docs/app/presets/import_ 

28 

_REFERENCES_ 

- <u>`76`</u> [67] Security Overview  ggml-org/llama.cpp - GitHub, accessed August 8, 2025, _https://github.com/ggml-org/llama.cpp/security_ 

- <u>`77`</u> [68] Delving deep into Llama.cpp and exploiting Llama.cpps Heap Maze, from Heap-Overflow to Remote-Code Execution. - Retr0s Register, accessed August 8, 2025, _https://retr0.blog/blog/llama-rpc-rce_ 

- <u>`78`</u> [69] GGUF - Hugging Face, accessed August 8, 2025, _https://huggingface.co/docs/transformers/gguf_ 

- <u>`79`</u> [70] gguf-parser - piwheels, accessed August 8, 2025, _https://www.piwheels.org/project/gguf-parser/_ 

- <u>`80`</u> [71] 99991/pygguf: GGUF parser in Python - GitHub, accessed August 8, 2025, _https://github.com/99991/pygguf_ 

- <u>`81`</u> [72] GGUF - Hugging Face, accessed August 8, 2025, _https://huggingface.co/docs/hub/gguf_ 

- <u>`82`</u> [73] Generating SQL for SQLite using Ollama, ChromaDB - Vanna.AI Documentation, accessed August 8, 2025, _https://vanna.ai/docs/sqlite-ollamachromadb/_ 

- <u>`83`</u> [74] A completely local RAG: .NET Langchain, SQLite and Ollama with no API keys required. | by John Kane | Medium, accessed August 8, 2025, _https://medium.com/@johnkane24/a-completely-local-rag-netlangchain-sqlite-and-ollama-with-no-api-keys-required-d36c53652f00_ 

- <u>`84`</u> [75] Features | Open WebUI, accessed August 8, 2025, _https://docs.openwebui.com/features/_ 

- <u>`85`</u> [76] SQLite Forensic Corpus - Digital Corpora, accessed August 8, 2025, _https://digitalcorpora.org/corpora/sql/sqlite-forensic-corpus/_ 

- <u>`86`</u> [77] Writing YARA rules yara 4.4.0 documentation, accessed August 8, 2025, _https://yara.readthedocs.io/en/stable/writingrules.html_ 

- <u>`87`</u> [78] YARA Rules Guide: Learning this Malware Research Tool - Varonis, accessed August 8, 2025, _https://www.varonis.com/blog/yara-rules_ 

- <u>`88`</u> [79] What is YARA rules? | Netenrich Fundamentals, accessed August 8, 2025, _https://netenrich.com/fundamentals/yara-rules_ 

- <u>`89`</u> [80] Inside the Container: Advanced Forensics for Kubernetes and CloudNative Threats | by Balasubramanya C | Medium, accessed August 8, 2025, _https://medium.com/@balasubramanya.c/inside-the-containeradvanced-forensics-for-kubernetes-and-cloud-native-threats-7cf601408aba_ 

- <u>`90`</u> [81] What is RAG (Retrieval-Augmented Generation)? - AWS, accessed August 8, 2025, _https://aws.amazon.com/what-is/retrieval-augmented-generation/_ 

29 

_REFERENCES_ 

- <u>`91`</u> [82] What is retrieval-augmented generation (RAG)? - IBM Research, accessed August 8, 2025, _https://research.ibm.com/blog/retrieval-augmentedgeneration-RAG_ 

30 

# SOURCE: 2603.29364v1.pdf

# **Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications** 

JIACHENG WANG, Nanyang Technological University, Singapore WEIHONG QIN, Jilin University, China JIALING HE, Chongqing University, China CHANGYUAN ZHAO, Nanyang Technological University, Singapore DUSIT NIYATO, Nanyang Technological University, Singapore TAO XIANG, Chongqing University, China 

This survey examines intelligent forensics in next-generation mobile networks, arguing that future wireless security must move beyond real-time detection toward accountable post-incident reconstruction. Unlike traditional digital forensics, wireless investigations rely on short-lived, distributed, and heterogeneous evidence, including radio waveforms, channel measurements, device-side artifacts, and network telemetry, affected by calibration, timing uncertainty, privacy constraints, and adversarial manipulation. To address this limitation, this paper develops an evidence-centric framework that treats wireless measurements as first-class forensic artifacts and organizes the field through a unified taxonomy spanning physical-layer, device-layer, networklayer, and cross-layer forensics. We further systematize the forensic workflow into readiness and preservationby-design, acquisition, correlation and analysis, and reporting and reproducibility, while comparing the complementary roles of traditional methods and artificial intelligence-assisted techniques. Subsequently, we review major application areas, including anomaly discovery, attribution, provenance and localization, authenticity verification, and timeline reconstruction. Finally, we identify key open challenges, including domain shift, resource-aware evidence capture, and the benefits and admissibility risks of generative evidence. Overall, this paper positions wireless forensics as a foundational capability for trustworthy, auditable, and reproducible security in next-generation wireless systems. Readers can understand and streamline wireless forensics processes for specific applications, such as low-altitude wireless networks, vehicular communications, and edge general intelligence. 

CCS Concepts:  **General and reference**  **Surveys and overviews** ;  **Networks**  **Mobile networks** ;  **Computing methodologies**  **Artificial intelligence** . 

Additional Key Words and Phrases: Wireless networking; forensics and security 

## **1 INTRODUCTION** 

## **1.1 Background and Motivation** 

Forensics refers to the disciplined practice of identifying, collecting, examining, and reporting evidence so that conclusions are reproducible, verifiable, and, when necessary, admissible [1]. In modern digital infrastructure, incidents rarely stay within a single component; rather, they span software stacks, cloud services, identity systems, and network control functions, where traces may be fragmented or overwritten. Hence, beyond remediation, the key task is forensic reconstruction: determining what happened and why from preserved artifacts rather than assumptions. The Capital One incident disclosed in 2019 illustrates this challenge. Public reports indicate that the attacker exploited a web application vulnerability, then obtained cloud credentials and accessed cloud storage services, causing unauthorized exposure of customer related data [2]. Investigators therefore had to correlate heterogeneous evidence, including application and security alerts, identity and access traces, and cloud audit and storage access logs, to reconstruct the intrusion path, establish the 

Authors Contact Information: Jiacheng Wang, jiacheng.wang@ntu.edu.sg, NTU, Singapore; Weihong Qin, qinwh25@mails.jlu.edu.cn, Jilin University, Changchun, China; Jialing He, hejialing@cqu.edu.cn, Chongqing University, Chongqing, China; Changyuan Zhao, zhao0441@e.ntu.edu.sg, NTU, Singapore; Dusit Niyato, dniyato@ntu.edu.sg, NTU, Singapore; Tao Xiang, txiang@cqu.edu.cn, Chongqing University, Chongqing, China. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:2 

timing and scope of data access, and justify the conclusions [3]. More broadly, this case shows that complex systems require accountable reconstruction, not only real time alarms. 

This need on intelligent wireless forensics is becoming more significant in wireless communication systems. Future wireless networks are programmable service platforms that connect heterogeneous devices and couple radio access with edge computing under strict latency and reliability constraints. The same infrastructure may simultaneously support consumer traffic, industrial telemetry, vehicular safety messages, and sensing related bit streams, raising the demand for accountable post incident analysis. Forensic capability is especially important because wireless evidence can directly affect cyber physical control, which is distributed across operators, cloud and edge providers, and devices, and is further complicated by softwareized architectures such as cloud native cores, network slicing, and disaggregated radio access networks [4]. Hence, wireless forensics is increasingly needed for incident response, operational accountability, safety validation, and policy refinement [5]. In addition, wireless communication systems are becoming a foundational component of critical digital infrastructure, supporting industrial automation, autonomous transportation, smart cities, and large-scale internet of things (IoT) deployments. As these systems evolve toward sixth generation (6G) architectures, they increasingly integrate programmable networks, edge intelligence, and heterogeneous wireless devices operating across multiple layers of the communication stack. While these advances enable unprecedented connectivity and flexibility, they also introduce new security vulnerabilities and attack surfaces that extend beyond traditional network boundaries. Consequently, securing wireless systems now requires not only detecting attacks and mitigating threats in real time, but also reconstructing incidents with sufficient evidence to understand how attacks unfolded and how defenses should be improved. Meanwhile, wireless makes forensic reconstruction harder than many wired settings. Relevant traces are often ephemeral; evidence is inherently multi-layer, spanning radio features, channel dynamics, timing relations, and protocol bit streams; encryption reduces semantic visibility; and telemetry is fragmented across base stations, roadside units, access points, user devices, and edge controllers [6]. From a security perspective, incident reconstruction in wireless networks is particularly challenging because decisive evidence frequently resides in transient radio signals, distributed device artifacts, and multi-layer telemetry. Radio waveforms, channel measurements, and device-level state transitions may collectively encode critical clues about malicious activity, yet these signals are often short-lived and difficult to preserve. 

Therefore, wireless forensics workflows must address short-lived observables, environmental uncertainty, distributed evidence and time alignment, scale and resource limits, and adversarial manipulation. Conventional security monitoring and intrusion detection remain indispensable for real-time defense in wireless networks [7], but they do not provide preserved, provenance-aware, and cross-vantage evidence for post incident scrutiny [8]. Traditional digital forensics is also less effective when decisive wireless traces are short lived, distributed, and constrained by resource and privacy limits [6]. These gaps motivate wireless intelligent forensics, which complements existing defenses through preservation by design, resource aware evidence capture, and auditable, learning assisted analysis that can cope with uncertainty, mobility, and heterogeneous hardware [9]. By systematically capturing and correlating radio observations with device- and network-layer telemetry, wireless intelligent forensics enables accountable reconstruction of attacks, attribution of malicious transmissions, and validation of security claims under explicit uncertainty assumptions. Guided by this motivation, this survey analyzes wireless evidence sources, forensic workflows, analysis methods, evaluation practices, and case studies across 6G and beyond. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:3 

## **1.2 Related Work** 

Existing review papers related to this survey can be grouped into digital/network forensics and IoT/mobile/wireless security, both of which provide important foundations. 

**_1) Digital and Network Security:_** Many surveys establish the general foundations of digital forensics. For example, [10] reviews recurring themes such as evidence acquisition, analysis pipelines, tool support, scalability, and standardization. From the network perspective, [11] surveys network forensic frameworks and emphasizes systematic collection, multi-source correlation, and trustworthy reporting, while [12] provides a taxonomy and discusses attribution under partial observability and large-scale heterogeneous trace correlation. Related surveys further examine operational limits on preservation and processing: [13] highlights prioritization and data reduction under growing forensic data volume, and [14] motivates Digital Forensics as a Service for scalable processing and shared tooling. In cloud settings, [15] analyzes limited physical access, multi-tenancy, and dependence on provider logs and APIs, and [16] organizes cloud forensic artifacts and challenges across investigation stages. Overall, these works clarify workflows, scalability, and cross-domain evidence, but rarely treat radio and link-layer measurements as first-class evidence or address channel uncertainty and mobility. 

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

held-out set. This mathematically forces ambiguous observations to trigger multiple hypotheses with coverage-controlled validity rather than overconfident single-label attribution. 

## **2.3 Device-layer Forensics** 

Device-layer wireless forensics focuses on endpoint components that terminate RF links and execute wireless protocols, including baseband processors, radio interface layers, and so forth. Extracting these artifacts links over-the-air physical-layer phenomena to concrete device-state transitions [42]. 

**_1) Evidence Types:_** Device-layer wireless evidence encompasses artifacts generated by the endpoint radio protocol stack. The primary categories involve baseband firmware interfaces and radio interface layers execution states. These artifacts explain cellular networks and wireless anomalies beyond standard operating system logs. Concrete studies operationalize the extraction of these specialized wireless artifacts. Investigating baseband and firmware interfaces, the work in [43] demonstrates that radio interface layers binaries can be mined to recover opaque vendor-proprietary command semantics. The authors implement BaseMirror using bidirectional taint analysis seeded from baseband interaction system application programming interface (API). The system resolves C++ virtual calls to recover indirect call targets and filters out non-baseband channels via associated system paths. This provides investigators with crucial device-layer explanations for physicallayer cellular behaviors. Highlighting radio interface layers execution states, the work in [44] demonstrates that inserting monitors at the cellular modem boundary yields actionable evidence about control path abuse. The authors encode wireless attack signatures as configurable rules and parse protocol data units to extract fields such as protocol identifier and data coding scheme. The system correlates this message context to distinguish various cellular attack classes. 

**_2) Supported Forensic Claims:_** Device-layer wireless evidence primarily supports three specialized forensic claims. Radio stack behavioral attribution connects network-layer signaling events to explicit device-layer baseband execution states. Hardware-based identity integrity ensures cryptographic materials and subscriber identity modules remain uncompromised. Baseband protocol conformance validates cellular modem adherence to standard state machine rules against vulnerability-driven deviations. These explicit boundaries critically anchor physical-layer phenomena to verifiable device-layer execution states [45]. Concrete studies operationalize these device-layer forensic claims using highly specific technical methodologies. Highlighting hardwarebacked identity integrity, the survey in [46] analyzes remote attestation across trusted execution 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:8 

environments (TEEs). This process extracts a cryptographic hash of the application code measurement. The device-layer system cryptographically signs these claims to form verifier-checkable evidence. Investigating baseband protocol conformance, the work in [47] introduces long term evolution (LTE)-based approach to dynamically test network-layer control-plane procedures. The authors utilize open-source software to generate crafted stimuli including invalid plain requests and replayed messages. A decision tree classifies device-layer baseband behaviors by tracking radio link failures from device-side logs. One attack vector skips the radio resource control (RRC) Security Mode Command. This omission forces the baseband to establish a data radio bearer via a plain RRC Connection Reconfiguration. 

**_3) Limitations and Adversarial Threats:_** Device-layer forensics suffers from severe structural constraints encompassing privileged mediation isolating radio stack artifacts from main application processors, vendor-specific firmware opacity obscuring internal cellular state machines, and critical visibility gaps preventing physical-layer anomalies from propagating into device-layer operating system logs [48]. Adversaries actively exploit these vulnerabilities through log evasion injecting physical-layer binary payloads into subscriber identity modules to completely bypass logging and baseband state desynchronization utilizing crafted network-layer stimuli to force anomalous modem states without triggering application-layer alarms [49]. Concrete analyses illustrate these weaponized constraints. Addressing log evasion, the system in [50] exploits transmitting noninteractive physical-layer binary messages via short message service channels to execute commands directly on subscriber identity modules. The intercept of calls from the Android radio interface layer successfully mitigates 19 distinct attacks and 11 malware samples with only a 1% hourly battery overhead. Investigating baseband state desynchronization, the analysis in [47] utilizes malformed physical-layer paging requests to force device-layer basebands into silently dropping radio resource control connections. Processing these rejections internally without upper-layer operating system notification directly enables completely untraceable denial of service operations. 

**_4) Mitigations and Cross-checks:_** Device-layer mitigation maintains endpoint evidence availability despite baseband opacity through three primary strategies. Privilege-boundary interception captures communication between the application processor and the cellular modem before baseband obfuscation [50]. Diagnostic telemetry extraction leverages proprietary interfaces to expose physical-layer radio messages to device-layer auditors. Cross-layer state reconciliation validates isolated device-layer baseband narratives against independent network-layer signaling traces. These practices ground forensic conclusions in reproducible wireless artifacts [51]. Addressing diagnostic telemetry extraction, the system in [52] interfaces directly with the device-layer chipset diagnostic mode. The software emulates an external logger within the operating system user space via virtual hardware paths. Investigators utilize this side channel to pull raw hexadecimal logs directly from the baseband interface. Decoding these binary streams extracts granular physical-layer payloads tracking radio resource control and non-access stratum state transitions. This verifiable ground truth prevents attackers from hiding physical-layer manipulations behind opaque device-layer firmware boundaries. 

## **2.4 Network-layer Forensics** 

Network-layer forensics utilizes network-visible traces to reconstruct wireless incidents. Packet, flow, and control-plane observations establish interaction structure even when payload visibility is limited. The evidential value of these traces relies on capture provenance. Vantage points, sampling policies, and timestamping accuracy strictly shape the claim boundary [53]. This dependency is critical in modern wireless architectures. 

**_1) Evidence Types:_** Network-layer forensics utilizes packet traces, flow records, control-plane logs, and management telemetry to reconstruct wireless incidents. Collection provenance including 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:9 

<!-- Start of picture text -->
Mitigations &<br>cross-checks:<br><!-- End of picture text -->

<!-- Start of picture text -->
Limits / failure modes:<br> Privilege isolation<br> Firmware opacity<br> Cross-layer visibility gap<br><!-- End of picture text -->

<!-- Start of picture text -->
Anti-forensics & adversarial<br>manipulation:<br> Subscriber identity module<br>/baseband log bypass<br> Baseband state desync<br><!-- End of picture text -->

<!-- Start of picture text -->
 Baseband state desync  Hardware trust anchors<br>securely isolate forensic<br>memory acquisitions.<br>Evidence type Baseband firmware  Supported forensic claims  Memory state fuzzing uncovers hidden<br>Internal radio protocol  interfaces proprietary baseband<br>primary evidentiary surface stack interfaces  semanticsvendor cmds  diag   Radio-stack behavioral   Radio interface vulnerabilities.<br>attribution interception mitigates<br>Baseband Radio Interface Layer RIL execution states  Hardware- malicious non-interactive<br>identity moduleSubscriber  Firmware communication traces  protocol data unit context  monitorsinter-process   Baseband backed identity integrityprotocol   Physical-logical binary payloads.verification explicitly connects phenomena to endpoint states.<br>Visibility depends on privilege boundary & acquisition mode conformance<br>command/diag<br>semantics<br>runtime<br>control-path state<br><!-- End of picture text -->

Fig. 3. Device-layer forensics framework for radio-stack evidence and defensible claims. The framework depicts endpoint radio-stack evidence under privilege boundaries and acquisition provenance, and summarizes the supported forensic claims, representative limitations, adversarial manipulation, and key mitigation and cross-check measures [50, 51]. 

vantage points, sampling configurations, and transformation histories strictly bounds independent forensic verification [26]. Concrete case studies operationalize these evidence categories. Addressing packet traces, the system in [54] partitions encrypted network-layer traffic into 5-tuple flows to construct traffic interaction graphs. By linking intra-burst packets and inter-burst sequences, a graph neural network classifier trains on decentralized application measurements to support structural reconstruction claims without application-layer payload inspection. Investigating control-plane logs, the framework in [55] instruments fifth generation (5G) core network functions to capture service-based architecture messages outside transport-layer encryption boundaries. Aggregating these plaintext records builds call flow provenance graphs that explicitly track transformations across subscription permanent identifiers, subscription concealed identifiers, 5G globally unique temporary identifiers, internet protocol (IP) addresses, and network function identities. This state tracking enables precise cross-entity correlation where passive network-layer monitors fail. 

**_2) Supported Forensic Claims:_** Network-layer evidence primarily supports four specialized forensic claims encompassing encrypted service identification, flow-signature intrusion detection, virtualized event timeline reconstruction, and control-plane correctness analysis. These verifications require strict provenance data to definitively bind network-layer symptoms to specific device-layer or application-layer activities [56]. Concrete studies operationalize these claims using highly specific methodologies. Addressing intrusion detection, the system in [57] compiles Snort rule semantics into predicates over IP flow information export features. Matching exported flow records instead of raw packets increases system throughput from 2.5 Gbit/s to 9.5 Gbit/s while fully preserving signature auditability. Investigating timeline reconstruction in virtualized 5G environments, the framework in [58] instruments containerized core components to collect operating-system provenance events. Orchestrating these labeled scenarios generates structured provenance graphs to enable replayable cause-effect reconstruction across dispersed network-layer services. 

**_3) Limitations and Adversarial Threats:_** Network-layer forensics suffers from structural incompleteness via sampled flow exports and limited packet captures, estimation uncertainty requiring strict error boundaries for telemetry metrics, and semantic ambiguity decoupling network-layer routing tuples from application-layer payloads via encryption and address translation [59]. Adversaries weaponize these vulnerabilities through traffic obfuscation manipulating packet directions and timing patterns to defeat flow analysis. Attackers simultaneously utilize learning-pipeline 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:10 

<!-- Start of picture text -->
Network-layer Forensics<br>Anti-forensics & adversarial<br>manipulation:<br> Traffic obfuscationLearning-pipeline attacks cross-checks:Mitigations &<br> Control-plane deception  Sketch error bounding<br> Sensing disruption calculates query-specific<br>telemetry uncertainty<br>budgets.<br>Evidence type Supported forensic   Control-plane<br>claims provenance graphs track<br> Packet traces / flow records  Encrypted/aggregated  complex core identifier transformations.<br>service ID  Adversarial flow training<br> Control-plane  Interaction graph  Intrusion detection and  hardens network<br>& signaling  policy enforcement classifiers against<br>logs  Timeline and impact   backdoor triggers.Cross-entity tracking<br> Management  scope enables precise protocol<br>telemetry Provenance graph  Control-plane deviation analysis correlation bypassing passive monitors.<br><!-- End of picture text -->

<!-- Start of picture text -->
<br><br><br><br><!-- End of picture text -->

<!-- Start of picture text -->
 Monitoring<br>points / taps<br> Routers &<br>middleboxes<br>Access & core<br>components<br> Management<br>plane<br><!-- End of picture text -->

Fig. 4. The network-layer forensics framework for provenance-aware traffic and signaling analysis. Depict infrastructure evidence interpreted under explicit capture provenance, and summarize the supported forensic claims, representative failure modes, adversarial manipulation, and mitigation and cross check measures based on provenance preservation, uncertainty aware reporting, and data plane and control plane reconciliation [67]. poisoning to inject stealthy triggers into network-layer classifiers [60]. Concrete studies illustrate these severe vulnerabilities. Addressing structural incompleteness, the system in [61] utilizes multistage filters to identify heavy hitters using constrained static random access memory (SRAM). Hashing collisions inherently induce false positive and negative trade-offs during flow promotion. Investigating learning-pipeline poisoning, the research in [62] embeds explanation-guided backdoors into flow classifiers. Injecting specific packet-length sequences forces models to misclassify malicious flows as benign without impacting clean traffic, explicitly demanding rigorous training provenance verification for model-based network forensics. 

**_4) Mitigations and Cross-checks:_** Network-layer mitigation maintains evidence availability and interpretability under strict privacy constraints through three primary strategies. Privacypreserving telemetry export balances network-layer visibility with data minimization at capture points. Uncertainty-aware reporting mathematically forces machine learning inferences to output calibrated confidence intervals. Cross-layer state reconciliation validates isolated network-layer traces against independent control-plane narratives [63]. These strategies collectively ensure that defensible conclusions rest on preserved artifacts with known provenance [64]. Concrete systems illustrate how these mitigation categories strengthen network-layer evidence. Addressing privacy-preserving telemetry, the work in [65] proposes the in-band network telemetry (INT)based system. The authors formulate telemetry item selection as a 0-1 Knapsack optimization problem. They implement this logic to selectively export high-value metadata while suppressing sensitive attributes. This approach maintains network-layer monitoring accuracy above 95% while reducing bandwidth overhead by approximately 40%. Highlighting uncertainty-aware reporting, the study in [66] addresses the overconfidence of standard flow classifiers. The system applies Bayesian approximation techniques including Monte Carlo dropout and flipout to quantify epistemic uncertainty in intrusion detection. Establishing a threshold on predictive entropy allows the system to distinguish known attack patterns from out-of-distribution anomalies. Filtering predictions based on these uncertainty scores significantly increases network-layer evidence reliability. 

## **2.5 Cross-layer Fusion** 

Cross-layer wireless forensics combines over-the-air measurements, device-side artifacts, and infrastructure telemetry to reconstruct incidents and support verifiable claims. This is important because evidence from a single layer is often incomplete, whereas cross-layer alignment can 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:11 

strengthen reconstruction by linking identity, timing, and context across heterogeneous logs and measurements [68]. 

**_1) Evidence Types and Supported Claims:_** Cross-layer investigations extract hybrid evidentiary artifacts encompassing behavioral consistency metrics and multi-modal feature vectors [69]. These integrated artifacts explicitly support specialized forensic claims encompassing identity-physical binding and operational provenance validation [68]. Identity-physical binding strictly verifies claimed device-layer identities against actual physical-layer RF sources. Operational provenance validation confirms network-layer sessions and application-layer payloads originate from physically legitimate transmitters rather than spoofed nodes. Concrete studies operationalize these integrated cross-layer verifications. Addressing identity-physical binding, the system in [70] dynamically superimposes physical-layer authentication signals as covert tags onto source message waveforms by adaptively adjusting tag parameters according to real-time wireless channel states. Blindly extracting these superimposed features from noisy receptions via hypothesis testing frameworks achieves a 98% identification rate under a -15 dB signal-to-noise ratio. This explicitly binds physical-layer transmission provenance to claimed device-layer identities without requiring application-layer cryptographic keys. Investigating operational provenance validation, the framework in [71] superimposes physical-layer authentication tags onto device-layer Wi-Fi frame modulation constellations. Utilizing tag-encoding functions minimizes application-layer source message modification ratios while maintaining legitimate wireless link bit error rates within normal tolerances. This transparent integration successfully neutralizes device-layer mimicry attacks without disrupting standard network-layer processing or upper-layer communications [72]. 

**_2) Limitations and Adversarial Threats:_** Cross-layer forensic reliability suffers from severe decoupling factors encompassing environment-induced feature drift, encryption-induced opacity, and inherent semantic gaps [73]. Adversaries actively exploit these vulnerabilities through generative feature fabrication synthesizing physical-layer signatures to match legitimate device-layer identities, environmental reshaping cloning spatial fingerprints, and adversarial evasion forcing fusion model misclassification on malicious network-layer streams to disrupt required inter-layer correlations despite single-layer plausibility [74]. Concrete studies illustrate these weaponized cross-layer boundaries. Addressing encryption-induced opacity, the analysis in [75] demonstrates modern transport layer security (TLS) 1.3 protocols fully encrypting ClientHello messages and obfuscating handshake patterns via zero round trip time session resumption to completely decorrelate physical-layer traffic shapes from application-layer semantics. Investigating generative feature fabrication, the research in [76] evaluates attackers utilizing Wasserstein GANs to mimic legitimate physical-layer phase difference distributions. Calculating anomaly scores from discriminator feature residual errors and image reconstruction errors successfully detects synthetic spoofs perfectly aligning with legitimate device-layer training data. 

**_3) Mitigations and Cross-checks:_** Cross-layer mitigation counters decoupling threats through active challenge-response verification injecting probes to elicit unforgeable physical-layer responses, robust adversarial training hardening fusion models against gradient-based perturbations, and semantics-aware traffic recovery mining statistical flow correlations to reconstruct encrypted application-layer contexts [67]. Concrete studies operationalize these cross-layer defenses. Addressing active verification, the protocol in [77] utilizes zero-sum game strategies commanding device-layer drones to traverse specific waypoints. Matching measured physical-layer channel gain statistics against unique location multipath profiles successfully establishes authenticity. This multi-round spatial challenge forces attackers to solve intractable inverse problems to completely neutralize mimicry attacks. Combating adversarial manipulation, the framework in [78] integrates phase flipping data augmentation to simulate diverse physical-layer signal distortions. Employing 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:12 

soft label training strategies to smooth decision boundaries directly improves classification accuracy by 18.12% against optimization-based Carlini-Wagner attacks. 

## **2.6 Lessons Learned** 

Synthesizing these evidentiary dimensions reveals fundamental challenges in bridging severe cross-layer semantic gaps. Physical-layer investigations provide hardware-grounded truth bypassing upper-layer network spoofing. This fundamental advantage remains severely fragile under environmental mobility requiring picosecond-level synchronization and extensive provenance metadata [79]. Device-layer mitigations expose stealthy baseband desynchronization attacks utilizing hardware-backed trust anchors. This deep visibility faces extreme vendor dependence requiring reverse-engineered diagnostic interfaces instead of standardized forensic telemetry interfaces [80]. Network-layer synthesis successfully contextualizes raw baseband samples into actionable protocol signaling sequences. Payload encryption inherently restricts this network visibility strictly to control-plane metadata [79]. Producing defensible and replayable wireless evidence ultimately demands overcoming opaque hardware processing delays to guarantee strict cross-layer temporal alignment. 

## **3 WIRELESS FORENSICS WORKFLOW: TRADITIONAL VS. AI-BASED METHODS 3.1 Forensic Readiness & Preservation-by-design** 

**_1) Traditional Approaches:_** Traditional forensic readiness and preservation-by-design aim to ensure that a wireless system can produce evidence that is preservable, attributable, and reproducible [81]. For example, the authors in [82] introduce a readiness-as-a-service view, which organizes readiness into three process groups and further refines them into 11 processes covering policy design, forensic classification, asset identification, and so forth. They propose centralized forensic logging, conversion of backups into logical image files, chained hash-based integrity protection, and a capability maturity model integration (CMMI)-style evaluation model for continuous improvement. Besides, another direction strengthens preservation infrastructure by securing how evidence is stored and accessed. In [83], the authors propose a digital-forensic-readiness storage model that combines representational state transfer (REST)-based ingestion, metadata sanitization, integrity hashing, encryption, randomized filenames, read-only access, and two-factor authentication (2FA)-protected download to reduce tampering and unauthorized access. In [84], the authors study how to trigger forensic evidence collection in software-defined networks by integrating an evolved digital forensics and incident response architecture with an unusual-traffic detector and an unexpected-behavior detector. 

**_2) AI-based Approaches:_** Traditional triggers often rely on static intrusion detection system (IDS) alerts which are prone to high false-positive rates in dynamic 5G/6G environments. Learned triggers utilize machine learning (ML) and signal processing to recognize subtle behavioral or physical anomalies as precursors to an attack. The study in [85] uses Channel State Information (CSI) as a passive sensor and highlights the AI part through data-driven anomaly recognition and motion-pattern discrimination for radio-silent drones. Specifically, it combines Gaussian mixture model-based anomaly detection with short-time Fourier Transform (STFT)-based time-frequency analysis, template matching for drone-specific shiftingmoving patterns, and fixed-frequency enhancement for propeller-spinning signatures, thereby turning raw CSI fluctuations into an intelligent physical-layer trigger. Experimentally, the power spectrum (PS) and time-frequency distribution (TFD) modules achieve 96.8% and 93.5% true positive rates even at 10 m, while the overall system reaches 95.65% at 5 m and still maintains 60% at 7 m. Beyond physical sensing, system-level intelligence is required to manage the trade-off between forensic visibility and resource 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:13 

consumption. The study in [86] introduces an AI-based autonomic forensic-readiness design for drones, where a loop dynamically adjusts reporting interval and geolocation precision according to contextual risk. By combining blockchain-based tamper-proof storage with a prediction-driven, locality-sensitive approximation mechanism, it preserves verification correctness while reducing communication cost. In a case with 100 simulated flight paths, the method achieves up to 46% reduction in transmitted geolocation digits with zero verification error, using a 15 s sampling baseline. Moreover, integrating forensic intelligence into wireless infrastructure allows for a more centralized and proactive readiness posture. The study in [87] details the design of a forensic-ready Wi-Fi access point (AP) to secure heterogeneous IoT ecosystems. In this architecture, the AP serves as a centralized coordinator for evidence provenance, analyzing cross-device traffic to identify coordinated botnet behaviors. By offloading the learned trigger logic to the infrastructure layer, the framework ensures the capture of distributed attack traces that individual, resource-constrained IoT nodes are unable to record independently. 

Besides using AI to detect and interpret forensic-relevant events, another emerging direction is adaptive retention, where AI is used to transition from rigid periodic deletion to dynamic, valueaware lifecycle management. From terminal-side, resource-constrained nodes such as IoT devices and edge servers often struggle with the storage of comprehensive forensic logs. The study in [88] addresses these storage bottlenecks by pushing lightweight attack-detection models directly onto resource-constrained IoT/edge devices, so that forensic readiness is triggered locally through realtime classification rather than cloud-side analysis. Specifically, the proposed framework evaluates 9 ML/deep learning(DL) models for on-device multiclass attack recognition and then uses the detected attack type to drive evidence collection and preservation. The results show that AI can remain both accurate and lightweight in this setting, with multiclass accuracy reaching 99.60% on CICIoT2023 dataset and 99.98% on IoT-23 dataset, highlighting that the main contribution lies in resource-aware on-edge intelligence for forensic readiness. From network perspective, a more specialized advancement involves integrating intelligence within the network infrastructure to manage evidence persistence across various layers. For instance, on the access layer, the authors in [89] propose to extract traffic features over tunable aggregation windows and feeds them to pre-trained machine learning classifiers, while jointly optimizing the window length, the number of features, and the quantization bits under storage and central processing unit (CPU) constraints. The framework models forensic accuracy, storage cost, and computational load, and then solves a nonlinear optimization problem to allocate resources across device identification, human activity recognition from encrypted traffic, and smart-speaker interaction detection. Experiments on a Raspberry Pi 3B+ show that even under a 1 KB/s storage limit and a 10% CPU budget, the system still maintains 84.68% minimum accuracy with 30 active tasks. Such work shows that the main value of AI in adaptive retention lies not only in later classification, but also in deciding how much traffic evidence should be preserved, at what granularity, and for which forensic task under tight edge-side constraints. 

## **3.2 Acquisition** 

Evidence acquisition links physical wireless incidents to digital forensic reconstruction by collecting heterogeneous traces with integrity and provenance preserved. This section reviews sensor infrastructure, deterministic collection policies, and emerging AI-driven context-aware capture. 

**_1) Tools and Placements:_** At the extraction stage, wireless forensics relies on specialized tools to access measurements that are normally hidden by commodity radio stacks. For instance, In Wi-Fi systems, the Intel 5300 CSI tool first exposed per-packet channel state through modified firmware, but only for 30 grouped subcarriers over 20/40 MHz channels, which limits delay and multipath resolution [90]. After that, Nexmon pushes this deeper into Broadcom devices by using 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:14 

Table 2. Representative studies on forensic readiness and preservation-by-design. 

|**Category**|**Ref.**|**Focus**|**Core idea**|**Takeaway**|
|---|---|---|---|---|
|Traditional<br>aroaches|[82]|Readiness<br>workfow|Readiness-as-a-service with policy design, forensic<br>classifcation, asset identifcation, centralized<br>logging, and chained integrity protection.|Systematic readiness modeling, but<br>limited adaptivity to dynamic<br>wireless environments.|
|pp|[83]|Secure<br>preservation|REST-based evidence ingestion with sanitization,<br>hashing, encryption, randomized flenames,<br>read-onlyaccess, and 2FA.|Strong storage-side protection with<br>low idle overhead, but large-fle<br>latencyremains noticeable.|
||[84]|Evidence<br>triage|SDN-based triggered evidence collection using<br>unusual-trafc and unexpected-behavior detectors.|Avoids store-everything logging and<br>achieves about 97.3% accuracy/F1,<br>but depends on detectorquality.|
|AI-based|[85]|Physical-layer<br>trigger|CSI-based anomaly recognition with GMM/EM,<br>STFT, and drone-pattern matching for radio-silent<br>drone detection.|Learns subtle physical-layer triggers;<br>detection is strong at short range<br>but degrades with distance.|
|approaches|[86]|Autonomic<br>readiness|Risk-aware adjustment of reporting interval and<br>geolocation precision, combined with<br>tamper-resistant storage.|Reduces reporting cost by up to 46%<br>with zero verifcation error, but<br>stronger integrity lowers<br>throughput.|
||[87]|Infrastructure-<br>side readiness|A forensic-ready Wi-Fi AP centrally coordinates<br>provenance and cross-device trafc analysis in<br>heterogeneous IoT.|Captures distributed attack traces<br>beyond individual IoT nodes, but is<br>mainlyAP-centric.|
||[88]|On-device<br>retention trigger|Lightweight ML/DL models run on IoT/edge devices<br>to trigger local evidence collection after attack<br>recognition.|Enables accurate and lightweight<br>edge-side readiness, but depends on<br>modelgeneralization.|
||[89]|Adaptive<br>retention|Joint optimization of aggregation window, feature<br>number, and quantization bits under storage and<br>CPU constraints.|Supports value-aware evidence<br>persistence under tight edge<br>budgets, at the cost of higher<br>optimization complexity.|

mechanisms such as flashpatch-based read-only memory (ROM) redirection, thereby enabling monitor mode, raw frame injection, and programmable firmware behavior on commodity phones and IoT hardware [91]. In cellular networks, low-level measurements can also be extracted from commodity smartphones. The study in [52] achieves this by using the modems diagnostic interface to collect raw binary logs and decode fine-grained wireless messages, while incurring modest overhead, with runtime cost of about 17% CPU and 30 MB memory. Beyond capability, reliable evidence collection depends on how tools are placed. For instance, the authors in [92] model channeldependent capture probability explicitly and shows that assigning multiple sniffers to the same channel can improve packet-capture reliability under fading, hardware failure, and other imperfectmonitoring conditions. For wireless local area network (WLAN), the authors in [93] shows that sniffer placement should be optimized where faults are likely to occur, rather than only where AP coverage exists. This is because anomalies such as hidden terminals, often need multiple correlated viewpoints for reliable diagnosis. In its design, sniffers are placed near clients and configured to report physical-layer summaries, such as noise-floor samples, to a central inference engine every 10 s for joint analysis. 

**_2) Policy-based Acquisition:_** While sensors provide the physical capability for evidence capture, acquisition policies define the operational logic governing their activation and depth. Traditional forensic readiness models formalize collection procedures as repeat- able service functions based on predefined trigger rules [82]. For instance, authors in [81] propose a trigger-based provenance collection pipeline for low-power IoT networks, where suspicious link-layer conditions activate selective evidence logging instead of continuous full-trace retention. It is implemented on a 16-node network with 1 user datagram protoco (UDP) server, 15 clients, and 4 hopping channels, and is evaluated under 4 jamming variants and 3 synchronization attacks. Once triggered, the system stores the resulting graphs, and derives a Link-IoT dataset with 8 raw and 15 derived features for later analysis. To mitigate storage exhaustion in high-speed links, policy-based designs often employ tiered sampling strategies that balance storage footprints with reconstruction accuracy [94]. Building on this idea, the authors in [95] extend tiered retention to microservice observability by adaptively switching among metrics, logs, and traces according to forensic value. They use a game-theoretic model of malicious users and forensic-ready microservices, together with a runtime monitor-analyze-plan-execute (MAPE) loop, to decide when higher-fidelity evidence is worth 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:15 

preserving for subsequent event analysis. On the 41-microservice TrainTicket benchmark [96], across 9 uncertainty scenarios with 10%90% malicious users, the approach improves F-measure by 26.97%42.50% over sampling observability and by 3.1%38.96% over full observability. This result shows that policy-based acquisition can move beyond static sampling and instead preserve high-fidelity evidence. 

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

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:19 

RF fingerprinting likewise shows that performance is inseparable from collection, preprocessing, training, and deployment choices [132, 133]. Therefore, the analytical pipeline should be preserved as a versioned forensic artifact rather than treated as invisible infrastructure. 

Machine-readable manifests and replayable execution bundles are practical mechanisms for this goal. Manifests can enumerate sensor identities, clock references, observation geometry, decoding parameters, model hashes, and intermediate outputs, while containerized runtimes, executable notebooks, and scripted replay packages allow third parties to regenerate figures, rerun fusion pipelines, and test sensitivity to thresholds or alignment assumptions. For example, SigMF provides such metadata structure for RF recordings, and platforms such as POWDER, Colosseum, and Sionna illustrate replayable wireless experimentation under controlled conditions [113, 134136]. For AI-assisted workflows especially, such re-execution artifacts are essential, which is consistent with the broader emphasis of datasheets for datasets and model cards on documenting dataset lineage, evaluation conditions, and model behavior [137, 138]. 

**_3) AI-assisted Reporting and Human Review:_** As wireless forensic pipelines increasingly incorporate learned models, reporting must distinguish among observed measurements, extracted features, inferred associations, and final human-endorsed conclusions. An anomaly score, class label, or causal ranking should never be presented as self-sufficient evidence. Instead, reports should explain why the preferred interpretation is more credible than competing alternatives and disclose the calibration limits of the underlying model. This requirement is supported both by evidence that neural confidence scores are often miscalibrated and by documentation frameworks arguing that model outputs must be contextualized rather than reported in isolation [131, 138]. Supporting materials such as saliency maps, attention traces, feature attribution, and counterfactual comparisons may assist interpretation, but only when their provenance and uncertainty are explicitly bounded [139, 140]. 

According to the above analysis, human review remains indispensable. Analysts must determine whether a model relied on mechanism-relevant evidence or on incidental correlates tied to vendor, site, or collection procedure. This is particularly important in RF fingerprinting, where prior studies show that failures often originate from data collection, preprocessing, and deployment mismatch rather than model architecture alone [132, 133]. Reports should thus preserve analyst annotations, hypothesis revisions, and adjudication decisions alongside automated outputs, maintaining an auditable boundary between machine suggestion and forensic conclusion. In high-stakes settings, AIassisted reporting should not replace expert judgment, but make the relationship among preserved evidence, learned inference, and human reasoning open to independent scrutiny [130, 138]. 

## **3.5 Lessons Learned** 

Based on the above presented content, the comparison between traditional and AI-based methods shows that wireless forensics should evolve toward a hybrid workflow rather than treating the two paradigms as substitutes. Traditional methods remain indispensable because they provide explicit evidence-preservation logic, interpretable correlation paths, and reproducible reporting structures; for example, forensic-readiness models in [82] explicitly define evidence preservation and management procedures, and provenance-graph-based analysis in [114] supports auditable crosslayer correlation and structured attribution. However, their effectiveness is often limited by partial observability, encrypted traffic, identifier churn, mobility, and large-scale heterogeneity. AI-based methods improve this situation by enabling adaptive triggering, uncertainty-aware acquisition, multimodal fusion, causal hypothesis ranking, and evidence reconstruction under resource and visibility constraints; for example, the work in [85] uses CSI-driven learning to proactively detect radio-silent drones, and the study in [86] dynamically adapts evidence collection for forensic-ready drone services under resource constraints. However, AI also introduces new risks, including dataset 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:20 

<!-- Start of picture text -->
Reproducible  Explicit uncertainty information<br>pipeline & re-<br>execution artifacts<br>Missing observations<br>Execution  Confidence bounds & Coverage Limits Rejected eypotheses<br>manifests<br>Condensed analytical views<br>Versioned<br>code &<br>model state<br>Protocol-state<br>Calibration &  Cross-layer timelines Provenance graphs summaries<br>synchronizat<br>ion files<br>Preserved raw evidence<br>Replayable<br>runtime<br>bundles Packet captures &<br>Raw RF / CSI traces baseband Logs Control-plane records<br><!-- End of picture text -->

<!-- Start of picture text -->
Contextualized<br>machine outputs<br><!-- End of picture text -->

<!-- Start of picture text -->
boundary<br><!-- End of picture text -->

Fig. 7. The framework for turning analyzed wireless evidence into reviewable and replayable forensic conclusions. It summarizes a versioned computational pathway and how calibrated reporting, replayable execution bundles. It also presents the AI-assisted reporting with an auditable machine-to-human review boundary support independent scrutiny, transferable interpretation, and human-endorsed forensic conclusions [133]. 

bias, poor calibration, adversarial manipulation, and reduced interpretability [53, 131]. Therefore, future wireless forensics should combine the procedural rigor of traditional methods with the adaptivity of AI-based methods; the former provides the foundation for trust, provenance, and reproducibility, while the latter enhances efficiency, resilience, and analytical reach under complex wireless conditions. 

## **4 FORENSIC APPLICATIONS** 

This section analyzes the application of forensics in different wireless domains. At the same time, we provide some use cases to further demonstrate the specific use of forensics. 

## **4.1 Detection & Anomaly Discovery** 

Detection and anomaly discovery is typically the first always-on forensic function in a wireless system. Beyond raising alarms, it must operate under limited storage/visibility by deciding _what_ evidence to preserve and _where_ to investigate next. Concretely, it should localize anomalies to suspicious time windows, frequency regions, cells, or links, and retain sufficient artifacts so later stages can reproduce the finding and test alternative explanations. 

**_1) Radio and spectrum based anomaly discovery:_** Many anomalies are visible only at the radio interface (e.g., jamming, unexpected emitters, abnormal spectral occupancy). Since exhaustive labeling is impractical, unsupervised or weakly supervised learning is widely used to learn normality and flag deviations [141]. A shared feature space can make heterogeneous sensors comparable, for example, in [142], the authors build a common feature space across sensors and identify anomalies as outliers, with a closed loop that incorporates expert feedback. Technically, a global encoder with sensor-specific decoders forms an adversarial-autoencoderbased detector, producing anomaly scores from reconstruction/discriminator losses. To make results actionable and shareable, they perform interactive clustering on learned features and then apply semi-supervised outlier detection with label propagation, separating sparse-but-normal events from frequent-but-illegal events using limited feedback. Evaluations on 4 sensors over 56 days show gains over baselines such as one class support vector machine (SVM), and also indicate thresholding remains difficult without a semi-supervised phase. Similarly, reconstruction-based detectors can be implemented directly on spectrogram evidence. In [143], received signals are converted to short-time Fourier Transform 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:21 

spectrogram images, then a modified GAN augmented with an encoder enables spectrogram reconstruction. Anomalies are scored using a weighted combination of pixel-level reconstruction error and discriminator loss, with the threshold derived from normal training scores. Simulations show consistent gains over [141], improving detection sensitivity by up to 10 dB. 

**_2) KPI based anomaly discovery for service accountability:_** In modern wireless systems, many incidents first appear as _service_ anomalies (e.g., throughput drops, handover failure bursts, control plane instability) rather than raw spectral changes [144]. Thus, forensic anomaly discovery for service accountability often relies on multi KPI time series collected by base stations (e.g., reference signal received power, access, scheduling, link robustness) [145]. Since these KPIs are high-dimensional and strongly influenced by external factors, modeling periodicity and context is critical. For example, [146] proposes an unsupervised detector combining time-series decomposition with an long short-term memory (LSTM)-based GAN. It removes seasonal components to train an LSTM-GAN on normal-only data so the generator captures normal KPI dynamics, and at inference performs an inverse search to find generator inputs that best reproduce an observed KPI window; the reconstruction residual is used as the anomaly score. On the CellPAD KPI dataset, it outperforms ARIMA and TCN-AE, achieving F1/AUPRC of 0.94/0.99 on sudden-drop point anomalies and 0.91/0.96 on segment anomalies. 

**_3) Rogue infrastructure and identity anomalies:_** Rogue infrastructure denotes adversarial radios impersonating legitimate entities (e.g., fake base stations, deceptive access points) to extract identifiers or enable follow-on manipulation. Forensic anomaly discovery must convert these shortlived interactions into interpretable evidences (e.g., trace segments) that can be corroborated across layers and stakeholders. For example, protocol- and trace-level artifacts enable end-user detection. In [147], an on-device learning framework detects fake base stations and multi-step attacks from cellular protocol traces: first, a packet-level stateful LSTM with attention flags suspicious packets by modeling long-range sequence dependencies; Then, the traces are represented as directed graphs of packet transitions, and graph-based matching/learning identifies reshaped or previously unseen attacks. It reports 96% detection accuracy with 2.96% false positives, 86% accuracy on unseen variants, and a lightweight footprint of about 835 KB memory with power below 2 mW. Moreover, cross-user aggregation and external consistency checks strengthen rogue-infrastructure evidence. For instance, the crowdsourced system in [148] has user equipments (UEs) upload lightweight message/context logs, then the backend correlates evidence using complementary high-precision detectors: unusually strong serving-cell received signal strength indicator (RSSI), invalid base station (BS) identifier syntax, an short message service (SMS)-content bag-of-words SVM, and BSWiFi inconsistency checks (inferring location from nearby WiFi and comparing with carrier coverage). It unions these detectors for conservative fake base station (FBS) attribution and performs localization, reporting 4.7% suspicious messages attributable to the FBS and median localization accuracy of 11 m. 

## **4.2 Attribution & Identification** 

Infrastructure-side attribution aims to determine which service, slice, or application class was responsible for the observed wireless behavior using only infrastructure traces, such as radio access network (RAN) controllers [149]. The goal is to produce labels that support accountability, triage, and later reconstruction when payload inspection is infeasible due to privacy constraints. 

**_1) Infrastructure-side attribution in edge networks:_** A first line of work performs attribution close to the radio edge by mapping RAN telemetry to service labels that can be logged and replayed. In [150], the framework closes the loop from traffic/slice classification to physical resource block optimization, avoiding packet-level inspection. It releases an Open-RAN-compliant KPI dataset (about 2.83 GB from 447 minutes of real 5G traffic) and trains slice classifiers on sliding windows 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:22 

that stack consecutive KPI snapshots; each snapshot contains 17 KPI features and a new decision is produced every 250 ms. The classifier reaches up to 99% offline classification accuracy and up to 92% online classification accuracy for specific slices, enabling near-real-time UE reassignment across slices. A second line attributes responsibility using core/edge visibility into flows, service functions, and protocol metadata. Taking an example, for encrypted user-plane traffic, [151] models each bi-directional flow as a graph: packets are nodes with raw byte sequence features, edges encode packet order and inter-arrival time, and flow-level statistics are added as global attributes. A Graph Netsstyle message-passing pipeline avoids fixed-length padding and achieves 97.56% accuracy on the ISCXVPN2016 VPN subset, outperforming 96.00% raw-bytes-only and 95% fixed-length convolutional neural network (CNN)/attention-LSTM baselines. 

**_2) Signal-native device identification:_** Signal-native identification links a transmission to a specific user device using signal-level artifacts when higher-layer identifiers are missing, spoofed, or encrypted. A representative approach treats raw I/Q (or closely related low-level bytes) as the primary evidence object. In [152], an encrypted web traffic classification pipeline keeps only ClientHello/ServerHello handshake header bytes while masking fields (e.g., cipher info) to avoid server-name lookup effects. Each flow is encoded as a three-channel time-series so hundreds of packets are modeled without exploding input size, and standard flow statistics from a flow meter are appended. These features are processed by a tripartite network and dense layers with strong dropout, achieving about 95.6% accuracy/F1, outperforming 91% of CNN/CNN-LSTM baselines trained on raw TLS bytes. Beyond general radios, identification at scale appears in back-scatter systems. In practical scenarios, because large labeled datasets are rarely available per device and signatures drift across days or power cycles, robustness hinges on transfer and label efficiency. The study in [153] shows that preprocessing choices directly affect performance and applies adversarial domain adaptation to improve cross-day device classification accuracy from 8.41% to 43.17% on one dataset and from 25.98% to 65.24% on another, reducing recollection cost. 

## **4.3 Provenance & Localization** 

While infrastructure-side attribution labels the responsible service or traffic class from edge-visible traces, provenance and localization emphasize the _spatial_ evidence chain: each location/trajectory claim must be tied to underlying wireless measurements so results remain reproducible and defensible. In wireless environments, location evidence is often inferred from noisy, transient RSS and CSI observations collected across distributed receivers [154]. Thus, this section focuses on extracting auditable spatial evidence to localize the transmitter/source area, reconstruct trajectories, and report confidence regions suitable for dispute resolution. 

**_1) Provenance-aware localization and source finding:_** Provenance-aware localization and source finding seeks where a suspicious transmission originated while keeping the source claim verifiable from preserved evidence. For example, [155] rasterizes received signal strength (RSS), samples into a grid and applies a U-shaped network (U-Net) to output a continuous likelihood heatmap over candidate transmitter positions. It then thresholds the heatmap and uses connected components to produce discrete estimates with explicit missed-detection and false-alarm accounting. Reported localization error ranges from 0.7 m to 12.4 m, with runtime about 114 ms, fitting preservethen-reconstruct workflows. Complementary to purely data-driven inference, [156] augments a standard path-loss model with a lightweight learned correction to keep localization interpretable while adapting to mismatch. Using about 10,000 received-power observations over a 1 km<sup>2</sup> area and only the 15 strongest observations, the augmented model can approach the CramrRao bound in the path-loss scenario, offering a reference for evidentiary soundness when only partial high-value measurements are preserved. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:23 

**_2) Trajectory reconstruction with confidence regions:_** Beyond localization, trajectory reconstruction must describe how the target moves and how certain each step is, so investigators can report a path with defensible confidence regions. A practical approach is to treat confidence as a first-class output and propagate it over time. In [157], a cooperative tracking pipeline uses a teacher-student Bayesian neural network to predict location and uncertainty, and injects learned epistemic uncertainty into the multi base station tracking update via the likelihood function. This down-weights out-of-distribution channel conditions and sparse training regions, achieving 46 cm median error and _<_ 1 m error in 87% of cases, outperforming temporal CNN baselines. Reported epistemic uncertainty is about 10 cm in dense regions, rises to about 50 cm in sparse regions, and exceeds 2 m in extreme low-data areas, enabling admissible uncertainty bounds for trajectory reporting. Moreover, for lightweight uncertainty quantification, [158] applies Monte Carlo dropout to deep learning based mmWave multiple-input multiple-output (MIMO) localization as a lowcomplexity approximation to Bayesian neural network inference, explicitly producing confidence interval bounds. It runs many stochastic forward passes to obtain predictive mean/variance and constructs a confidence ellipse scaled by the chi square distribution. Simulations use a 28-GHz urban ray-tracing model, a beamforming fingerprint codebook with __ = 32, and a dense 401  401 grid; uncertainty uses 1,000 Monte Carlo dropout samples per position with dropout 0.2. Empirically, ellipse shape depends on maximum received power, becoming eccentric in low-power regions and tightening at higher power, explaining why some trajectory segments are inherently less defensible despite plausible point estimates. Overall, trajectory reconstruction is more forensic-friendly when each update includes a calibrated confidence region and the system flags when that region becomes unreliable. 

**_3) Case study:_** In indoor non-line-of-sight (NLoS) investigations, the forensic objective is to convert time-stamped CSI into defensible spatiotemporal evidence of where a non-cooperative moving target was likely located within a time window [159]. To support provenance-aware localization, the method first stabilizes evidence formation and then derives physical parameters. A dedicated reference channel captures the direct signal and suppresses interference and phase errors in surveillance channels, thereby revealing target-induced reflections. Next, a packet-aggregation two-dimensional matrix pencil method jointly estimates path-length-change-rate and absolute time of flight (ToF) under low signal-to-noise ratio (SNR). Presence is detected from changes in the ToF distribution, and localization is obtained through geometric reasoning: multi-antenna ToF measurements form ellipse constraints whose intersection determines the target position. The prototype is built on IEEE 802.11ac. A directional-antenna transmitter operates on channel 161 (5.805 GHz) with 80 MHz bandwidth at 600 Hz, while a Broadcom 4366C0 receiver extracts CSI using Nexmon [160]. The setup includes one directional reference channel and two omni surveillance channels, with offline MATLAB processing on a workstation. Reported timing shows that detection and localization can be updated at about one-second granularity in the tested configuration. 

The brick wall scenario in Fig. 8 provides several forensic indicators. Fig. 8a shows measurable shifts in estimated ToF and path-length-change-rate between the empty room and different target locations, indicating that CSI logs contain usable NLoS evidence after proper preprocessing. Fig. 8b shows that ToF estimation remains effective and improves as more packets are aggregated, confirming higher evidence fidelity under low SNR. Fig. 8c further shows localization improvement with 10, 20, 30, and 40 packets, yielding median errors of 2.53 m, 2.05 m, 1.90 m, and 1.78 m, respectively. Additional results in [159] show that narrower bandwidth degrades localization, whereas more packets improve it, making the tradeoff among measurement cost, update rate, and uncertainty explicit. Overall, this case study highlights a practical forensic pattern for provenance-aware localization: reliable location claims require controlled evidence formation, physically meaningful intermediate parameters such as ToF, and a geometric reporting process that can be replayed. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:24 

<!-- Start of picture text -->
0.70.60.50.40.30.2 Target absenceAppears at location 1Appears at location 2Appears at location 3 0.81 0.81 40 packets30 packets20 packets10 packets<br>0.10 0.6 0.6<br>-30 -20 -10 0 ToF (ns)10 20 30 40<br>0.75 Target absence 0.4 0.4<br>0.5 Appears at location 1Appears at location 2Appears at location 3 0.2 10 packets20 packets30 packets 0.2<br>0.25 40 packets<br>0-2.5 -2 -1.5 -1Path length change rate (m/s)-0.5 0 0.5 1 1.5 2 2.5 00 2 ToF estimation error (ns)4 6 840 packets 10 00 Localization error (m)Localization error (m)1 2 3 4 5 6<br>(a) (b) (c)<br>Probability Cumulative distribution function<br>Probability<br>Cumulative distribution function<br><!-- End of picture text -->

Fig. 8. The results of the case for provenance-aware detection and localization in wireless forensics [159]. (a) Parameter distribution comparison. (b) The ToF estimation accuracy. (c) The localization accuracy. 

## **4.4 Authenticity and Anti-forgery** 

Authenticity and anti-forgery are prerequisites for using wireless traces as defensible forensic evidence. Authenticity asks whether preserved evidence can be credibly bound to a claimed transmitter, time window, and operating context, while anti-forgery asks whether an adversary could manufacture or alter those evidences to create a misleading narrative without being detected [161]. This becomes critical when encryption blocks payload inspection and evidence is increasingly signal-native and distributed, motivating physical-layer tags/watermarks for validation. 

**_1) Authentication at the signal level:_** A common method is to embed an authentication tag into the physical waveform so verification remains possible even under payload encryption [162]. In tagbased physical-layer authentication, the transmitter superimposes a structured tag and the receiver verifies it using shared secrets and statistical tests, producing a lightweight authenticity hook that can be preserved for later forensic checking [163]. For example, [164] proposes encoded tag-based physical-layer authentication for single block (ET-SB) and multiple blocks (ET-MB): ET-SB uses the polarity relationship of two sequential tag symbols to build a binary mapping vector that decides whether each message symbol is tagged or unchanged, while ET-MB extends the mapping across multiple tag blocks to improve robustness at low SNR while increasing compatibility. Receiver verification is formalized as NeymanPearson hypothesis testing [165], with an optimal test statistic tailored to partial superimposition (unlike classic statistics that assume tagging everywhere). They report compatibility improvements of 15.63% and 23.45%, and security gains of 6 dB and 12 dB at SNR 10 dB versus the baseline that superimposes the tag on the entire modulated message block. However, a practical barrier is that some tag-based designs require broadcasting tag parameters or tuning them empirically, which can create overhead and leakage opportunities. Compared with the baseline using channel characteristics instead of distributing keys to generate authentication tags [166], the overhead gap grows from 0.25 KBytes to 4.75 KBytes and the latency gap from 0.46 ms to 7.64 ms as message transmissions increase. Third, [162] leverages the RIS as an authenticity actuator: RIS configurations deliberately reshape the channel into a verifiable pattern. Results are reported as receiver operating characteristic (ROC) curves for false-alarm probabilities from 10<sup>1</sup> to 10<sup>4</sup> ; when false-alarm is pushed below 10<sup>3</sup> , detection probability can drop below 0.92, which is useful for forensics because admissible authenticity claims are tied to an explicit operating point. 

**_2) Replay-resistant validation for wireless evidence:_** Replay can forge convincing wireless evidence without synthesizing signals, so replay resistance is essential for forensic validation. For instance, [167] proposes a challenge response physical layer authentication scheme over partially controllable channels. The receiver uses a controllable propagation component, such as RIS, to issue a physical challenge by randomly selecting a new RIS configuration before transmission. A packet is accepted only if the CSI estimated from it matches the expected channel state information under that configuration, so a previously recorded waveform becomes invalid after reconfiguration. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:25 

With an RIS of 80/100 elements, they apply a likelihood-ratio test and evaluate misdetection at target false-alarm probabilities from 10<sup>4</sup> to 10<sup>2</sup> , showing that stronger RIS randomization reduces spoofing success but also lowers spectral efficiency. They report that reducing spectral efficiency by about 3 bits/s/Hz can push misdetection to roughly 10<sup>5</sup> 10<sup>3</sup> , while the no-randomization baseline stays close to one under strong attacker assumptions. 

**_3) Case Study:_** In ISAC systems, CSI from routine communications enables sensing but also exposes physical environments to illegitimate nodes, creating an authenticity and anti-forgery tension: legitimate sensing must remain reliable/auditable while unauthorized receivers may conduct covert surveillance or manipulate outcomes. A practical authenticity carrier is the pilot. Instead of a standardized pilot, the transmitter forges it by embedding extra side information (a safeguarding signal) [168].Accordingly, the method forges a new pilot by generating a protection signal and modulating it onto the pilot. It includes three modules: (i) a discrete conditional diffusion model generates an activation graph for ISAC link/node selection conditioned on device deployment and user location to reduce sensing network cost; (ii) a continuous conditional diffusion model generates a safeguarding signal conditioned on the activated links/nodes and the legitimate node; (iii) the safeguarding signal is embedded into the pilot amplitude to mask user-induced CSI fluctuations. Further speaking, the evaluation uses a software-defined-radio testbed (USRP N321 with external clock and GNU Radio Tx/Rx chains) in two indoor scenarios (office room for training/evaluation and meeting room for evaluation). Using accuracy degradation rate on AF-ACT [169], ABLSTM [170], PhaseAnti [171], and CeHAR [172] over five actions, Fig. 9(a) reports average accuracy degradation rate (ADR) of 0.82, 0.79, 0.74, and 0.70, showing forged pilots make CSI an authorization-bound artifact. 

<!-- Start of picture text -->
0.9 AF-ACT ABLSTM PhaseAnti CeHAR 0.89 ABLSTM AF-ACT<br>0.850.8 0.810.83 0.830.850.8 0.820.84 0.790.82 0.78 0.850.81 0.824 0.83 0.821 PhaseAnti0.811 0.817 CeHAR0.821<br>0.75 0.75 0.75 0.74 0.77 0.794 0.781 0.801 0.81 0.796 0.8<br>0.7 0.71 0.72 0.710.680.72 0.68 0.73 0.736 0.722 0.720.69 0.73 0.717 0.72<br>0.65 0.65 0.69 0.702 0.71 0.71 0.7 0.7<br>0.65<br>0.6 100 200 300 400 500 600<br>WK WH ST SQ FL Packet transmission rate (packets/second)<br>(a) (b)<br>Average ADR<br>ADR<br><!-- End of picture text -->

Fig. 9. Results of the case for authenticity and anti-forgery in wireless forensics [168]. (a) Accuracy degradation rate of 5 activities for different systems. WK indicates walking, WH is waving hand, ST is sitting, SQ means squatting, and FL represents falling. (b) The impact of communication speed on ADR performance. 

We also investigate the impact of packet transmission rate on our method. As Fig. 9(b) shows, the average ADR of each system remains nearly stable across different transmission rates. For instance, when the transmission rate increases from 100 to 600, the ADRs of AF-ACT are 0.79, 0.78, 0.80, 0.81, 0.80, and 0.80. This capability is vital for anti-forgery, as a practical attacker may try to manipulate the system by operating at a different traffic rate to recover stable sensing features. In this regard, a key authenticity takeaway is to enforce authenticity at the signal interface: authentic sensing evidence should be a CSI stream recoverable only via an authorized replay procedure, not something any listener can derive from the same over-the-air pilot [173]. Therefore, forensic readiness should preserve and report the authorization-bound decoding context as firstclass evidence, storing sensing outputs together with provenance fields (capture descriptors, tool versions, calibration/synchronization status) so a third party can replay authorized decoding and reject forged or replayed outputs lacking a valid integrity and provenance chain. 

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

- [20] Boris Danev, Davide Zanetti, and Srdjan Capkun. 2012. On physical-layer identification of wireless devices. _ACM Comput. Surv._ 45, 1 (2012), 129. 

- [21] Qiang Xu et al. 2015. Device fingerprinting in wireless networks: Challenges and opportunities. _IEEE Commun. Surv. Tutorials_ 18, 1 (2015), 94104. 

- [22] Anu Jagannath et al. 2022. A comprehensive survey on radio frequency (RF) fingerprinting: Traditional approaches, deep learning, and open challenges. _Computer Networks_ 219 (2022), 109455. 

- [23] Robert Mitchell and Ray Chen. 2014. A survey of intrusion detection in wireless network applications. _Computer Communications_ 42 (2014), 123. 

- [24] Van Der Heijden et al. 2018. Survey on misbehavior detection in cooperative intelligent transportation systems. _IEEE Commun. Surv. Tutorials_ 21, 1 (2018), 779811. 

- [25] Maria Stoyanova et al. 2020. A Survey on the Internet of Things (IoT) Forensics: Challenges, Approaches, and Open Issues. _IEEE Commun. Surv. Tutorials_ 22, 2 (2020), 11911221. 

- [26] James R. Lyle et al. 2022. _Digital Investigation Techniques: A NIST Scientific Foundation Review_ . Technical Report NIST IR 8354. National Institute of Standards and Technology. 

- [27] Zhaoyang Han et al. 2023. Smart optimization solution for channel access attack defense under UAV-aided heterogeneous network. _IEEE Internet Things J._ 10, 21 (2023), 1889018897. 

- [28] Anu Jagannath et al. 2022. A Comprehensive Survey on Radio Frequency (RF) Fingerprinting: Traditional Approaches, Deep Learning, and Open Challenges. _Computer Networks_ 219 (2022), 109455. 

- [29] Ben Hilburn et al. 2017. SigMF: The Signal Metadata Format. _Proceedings of the GNU Radio Conference (GRCon)_ (2017). Available via GNU Radio Conference Proceedings. 

- [30] Daniel Halperin, Wenjun Hu, Anmol Sheth, and David Wetherall. 2011. Tool Release: Gathering 802.11n Traces with Channel State Information. _ACM SIGCOMM Computer Communication Review_ 41, 1 (2011), 53. 

- [31] Vladimir Brik et al. 2008. Wireless Device Identification with Radiometric Signatures. In _Proceedings of the 14th ACM International Conference on Mobile Computing and Networking_ . Association for Computing Machinery, 116127. 

- [32] Luca Arcangeloni et al. 2023. Detection of Jamming Attacks via Source Separation and Causal Inference. _IEEE Trans. Commun._ 71, 8 (2023), 47934806. https://doi.org/10.1109/TCOMM.2023.3281467 

- [33] Tianyi Zhao, Shamik Sarkar, Enes Krijestorac, and Danijela Cabric. 2024. GAN-RXA: A Practical Scalable Solution to Receiver-Agnostic Transmitter Fingerprinting. _arXiv preprint_ (2024). arXiv:2303.14312 [eess.SP] arXiv:2303.14312. 

- [34] Naeimeh Soltanieh et al. 2020. A Review of Radio Frequency Fingerprinting Techniques. _IEEE Journal of Radio Frequency Identification_ 4, 3 (2020), 222233. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:30 

- [35] Junqing Zhang et al. 2025. Physical Layer-Based Device Fingerprinting for Wireless Security: From Theory to Practice. _IEEE T INF FOREN SEC._ 20 (2025), 52965325. https://doi.org/10.1109/TIFS.2025.3570118 

- [36] Yanfeng Qu et al. 2026. Secure and privacy-preserving issues in integrated sensing and communication-enabled wireless networks: a survey. _EURASIP Journal on Advances in Signal Processing_ 2026, 1 (2026), 4. 

- [37] zkan Ylmaz and Mehmet Akif Yazc. 2022. The Effect of Ambient Temperature On Device Classification Based On Radio Frequency Fingerprint Recognition. _Sakarya University Journal of Computer and Information Sciences_ 5 (08 2022), 233245. 

- [38] Zijie Tang et al. 2024. RF Domain Backdoor Attack on Signal Classification Via Stealthy Trigger. _IEEE Trans. Mob. Comput._ (2024). https://doi.org/10.1109/TMC.2024.3404341 Early Access. 

- [39] Ning Xie, Shilian Li, and Zhongwen Tan. 2021. A Survey of Physical-Layer Authentication in Wireless Communications. _IEEE Commun. Surv. Tutorials_ 23, 1 (2021), 282310. https://doi.org/10.1109/COMST.2020.3042188 

- [40] Jason M. Merlo et al. 2023. Wireless Picosecond Time Synchronization for Distributed Antenna Arrays. _IEEE T Microw Theory_ 71, 4 (2023), 17201731. https://doi.org/10.1109/TMTT.2022.3227878 

- [41] Junwei Ma et al. 2024. Navigating Uncertainty: Ambiguity Quantification in Fingerprinting-based Indoor Localization. In _Proceedings of the 2024 IEEE Annual Congress on Artificial Intelligence of Things_ . IEEE. 

- [42] Konstantia Barmpatsalou, Tiago Cruz, Edmundo Monteiro, and Paulo Simoes. 2018. Current and future trends in mobile device forensics: A survey. _ACM Comput. Surv._ 51, 3 (2018), 131. 

- [43] Wenqiang Li, Haohuang Wen, and Zhiqiang Lin. 2024. BaseMirror: Automatic Reverse Engineering of Baseband Commands from Androids Radio Interface Layer. In _Proceedings of the 2024 on ACM SIGSAC Conference on Computer and Communications Security (CCS 2024)_ . ACM, 23112325. https://doi.org/10.1145/3658644.3690254 

- [44] Haohuang Wen et al. 2023. Thwarting Smartphone SMS Attacks at the Radio Interface Layer. In _Network and Distributed System Security Symposium (NDSS)_ . The Internet Society. https://doi.org/10.14722/ndss.2023.24432 

- [45] Antonio Muoz. 2024. Cracking the Core: Hardware Vulnerabilities in Android Devices Unveiled. _Electronics_ 13, 21 (2024), 4269. 

- [46] Jmes Mntrey, Christian Gttel, Marcelo Pasin, Pascal Felber, and Valerio Schiavoni. 2022. An exploratory study of attestation mechanisms for trusted execution environments. _arXiv preprint arXiv:2204.06790_ (2022). 

- [47] Hongil Kim et al. 2019. Touching the Untouchables: Dynamic Security Analysis of the LTE Control Plane. In _2019 IEEE Symposium on Security and Privacy (SP)_ . IEEE, 11531168. https://doi.org/10.1109/SP.2019.00038 

- [48] Rick Ayers, Sam Brothers, and Wayne Jansen. 2014. _Guidelines on Mobile Device Forensics_ . NIST Special Publication 800-101 Revision 1. National Institute of Standards and Technology. https://doi.org/10.6028/NIST.SP.800-101r1 

- [49] Grant Hernandez et al. 2022. FirmWire: Transparent Dynamic Analysis for Cellular Baseband Firmware. In _Proceedings of the Network and Distributed System Security Symposium (NDSS)_ . 

- [50] Haohuang Wen et al. 2023. Thwarting Smartphone SMS Attacks at the Radio Interface Layer. In _Network and Distributed System Security Symposium (NDSS)_ . 

- [51] 2015. Information technology  Security techniques  Incident investigation principles and processes. https: //www.iso.org/standard/44407.html 

- [52] Yuanjie Li et al. 2016. MobileInsight: Extracting and Analyzing Cellular Network Information on Smartphones. In _Proceedings of the 22nd Annual International Conference on Mobile Computing and Networking (MobiCom)_ . 

- [53] Syed Rizvi et al. 2022. Application of Artificial Intelligence to Network Forensics: Survey, Challenges and Future Directions. _IEEE Access_ 10 (2022), 110362110384. https://doi.org/10.1109/ACCESS.2022.3214086 

- [54] Meng Shen et al. 2021. Accurate Decentralized Application Identification via Encrypted Traffic Analysis Using Graph Neural Networks. _IEEE T INF FOREN SEC._ 16 (2021), 23672380. https://doi.org/10.1109/TIFS.2021.3059654 

- [55] Harsh Sanjay Pacherkar and Guanhua Yan. 2024. PROV5GC: Hardening 5G Core Network Security with Attack Detection and Attribution Based on Provenance Graphs. In _Proceedings of the 17th ACM Conference on Security and Privacy in Wireless and Mobile Networks (WiSec)_ . ACM. https://doi.org/10.1145/3643833.3656129 

- [56] John Sheppard et al. 2022. Artificial Intelligence for Network Forensics: A Survey. _IEEE Access_ 10 (2022), 113687113717. https://doi.org/10.1109/ACCESS.2022.3214506 

- [57] Felix Erlacher and Falko Dressler. 2022. On High-Speed Flow-based Intrusion Detection using Snort-compatible Signatures. _IEEE Trans. Dependable Secure Comput._ 19, 2 (2022), 11901205. https://doi.org/10.1109/TDSC.2020.3026747 

- [58] Amr Abouelkhair et al. 2024. 5GProvGen: 5G Provenance Dataset Generation Framework. In _2024 20th International Conference on Network and Service Management (CNSM)_ . IFIP. 

- [59] Rick Hofstede et al. 2014. Flow Monitoring Explained: From Packet Capture to Data Analysis with NetFlow and IPFIX. _IEEE Commun. Surv. Tutorials_ 16, 4 (2014), 20372064. https://doi.org/10.1109/COMST.2014.2321898 

- [60] Daniel Klischies et al. 2025. BaseBridge: Bridging the Gap Between Over-the-Air and Emulation Testing for Cellular Baseband Firmware. In _IEEE Symposium on Security and Privacy (SP) 2025_ . IEEE, 11011119. https://doi.org/10.1109/ SP61157.2025.00142 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:31 

- [61] Cristian Estan et al. 2004. Building a Better NetFlow. In _Proceedings of the ACM SIGCOMM Conference_ . 245256. https://doi.org/10.1145/1015467.1015495 

- [62] Giorgio Severi et al. 2023. Poisoning Network Flow Classifiers. In _Proceedings of the 39th Annual Computer Security Applications Conference (ACSAC 23)_ . ACM, Austin, TX, USA, 337351. https://doi.org/10.1145/3627106.3627123 

- [63] Suleman Khan et al. 2016. Network forensics: Review, taxonomy, and open challenges. _Journal of Network and Computer Applications_ 66 (2016), 214235. https://doi.org/10.1016/j.jnca.2016.03.005 

- [64] Jim R. Lyle et al. 2022. _Digital Investigation Techniques: A NIST Scientific Foundation Review_ . Tech. Rep. NIST IR 8354. National Institute of Standards and Technology. https://doi.org/10.6028/NIST.IR.8354 

- [65] Zhaojin Xu, Zhaoming Lu, and Zuqing Zhu. 2024. Information-Sensitive In-Band Network Telemetry in P4-Based Programmable Data Plane. _IEEE/ACM Transactions on Networking_ 32, 1 (2024), 568581. 

- [66] Jacopo Talpini, Fabio Sartori, and Marco Savi. 2024. Enhancing trustworthiness in ML-based network intrusion detection with uncertainty quantification. _Journal of Reliable Intelligent Environments_ 10, 4 (2024), 501520. https: //doi.org/10.1007/s40860-024-00238-8 

- [67] Zhuoying Duan et al. 2025. Adaptive Strategies in Enhancing Physical Layer Security: A Comprehensive Survey. _ACM Comput. Surv._ 57, 7 (2025), 136. 

- [68] Jie Xiong and Kyle Jamieson. 2013. Securearray: Improving wifi security with fine-grained physical-layer information. In _Proceedings of the 19th annual international conference on Mobile computing & networking_ . 441452. 

- [69] Ruizhe Yao et al. 2021. Intrusion Detection System in the Advanced Metering Infrastructure: A Cross-Layer FeatureFusion CNN-LSTM-Based Approach. _Sensors_ 21, 2 (2021), 626. 

- [70] Chen Wang, Mingrui Sha, Wei Xiong, Ning Xie, Rui Mao, Peichang Zhang, and Lei Huang. 2024. Blind Tag-Based Physical-Layer Authentication. _IEEE/ACM Transactions on Networking_ 32, 1 (2024), 47354748. 

- [71] Ning Xie et al. 2022. Physical Layer Authentication With High Compatibility Using an Encoding Approach. _IEEE Trans. Commun._ 70, 12 (2022), 82708285. 

- [72] Ning Zhang et al. 2020. Physical-Layer Authentication for Internet of Things via WFRFT-Based Gaussian Tag Embedding. _IEEE Internet Things J._ 7, 9 (2020), 90019010. 

- [73] Mamyr Altaibek et al. 2025. A Survey of Cross-Layer Security for Resource-Constrained IoT Devices. _Applied Sciences_ 15, 17 (2025), 9691. 

- [74] Damilola Adesina et al. 2023. Adversarial Machine Learning in Wireless Communications Using RF Data: A Review. _IEEE Access_ 25 (2023), 77100. 

- [75] Jiuxing Zhou, Wei Fu, Wei Hu, Zhihong Sun, Tao He, and Zhihong Zhang. 2024. Challenges and Advances in Analyzing TLS 1.3-Encrypted Traffic: A Comprehensive Survey. _Electronics_ 13, 20 (2024), 4000. 

- [76] Wei Xie et al. 2024. A Novel PHY-Layer Spoofing Attack Detection Scheme Based on WGAN-Encoder Model. _IEEE T INF FOREN SEC._ 19 (2024), 86168629. 

- [77] Mattia Piana, Francesco Ardizzon, and Stefano Tomasin. 2025. Challenge-Response to Authenticate Drone Communications: A Game Theoretic Approach. _IEEE T INF FOREN SEC._ 20 (2025), 48904903. 

- [78] Zhida Bao et al. 2025. PFRTF: A Robust Training Framework to Counter Adversarial Attacks in Signal Classification for Next-G Consumer Electronics. _IEEE Trans. Consum. Electron._ 71, 1 (Feb 2025), 12351248. 

- [79] Jihao Xin et al. 2024. High-Precision Time Difference of Arrival Estimation Method Based on Phase Measurement. _Remote Sensing_ 16, 7 (2024), 1197. 

- [80] Yiming Zhang et al. 2025. SCRUTINIZER Towards Secure Forensics on Compromised TrustZone. In _Proceedings of the Network and Distributed System Security Symposium_ . 

- [81] Lakshminarayana Sadineni, Emmanuel S Pilli, and Ramesh Babu Battula. 2021. Ready-iot: A novel forensic readiness model for internet of things. In _2021 IEEE 7th World Forum on Internet of Things (WF-IoT)_ . IEEE, 8994. 

- [82] Sung Jin Lee and Gi Bum Kim. 2021. K-FFRaaS: A Generic Model for Financial Forensic Readiness as a Service in Korea. _IEEE Access_ 9 (2021), 130094130110. 

- [83] Avinash Singh, Richard Adeyemi Ikuesan, and Hein Venter. 2022. Secure storage model for digital forensic readiness. _IEEE Access_ 10 (2022), 1946919480. 

- [84] Mara B Jimnez et al. 2024. A filtering model for evidence gathering in an sdn-oriented digital forensic and incident response context. _IEEE Access_ 12 (2024), 7579275808. 

- [85] Jiangyi Deng et al. 2024. Dr. Defender: Proactive Detection of Autopilot Drones Based on CSI. _IEEE T INF FOREN SEC._ 19 (2024), 194206. 

- [86] Yijun Yu et al. 2019. LiveBox: A Self-Adaptive Forensic-Ready Service for Drones. _IEEE Access_ 7 (2019), 148401148412. 

- [87] Fabio Palmese et al. 2023. Designing a Forensic-Ready Wi-Fi Access Point for the Internet of Things. _IEEE Internet Things J._ 10, 23 (2023), 2068620702. 

- [88] Syed Rizvi et al. 2024. Pushing Network Forensic Readiness to the Edge: A Resource Constrained Artificial Intelligence Based Methodology. In _2024 Cyber Research Conference - Ireland (Cyber-RCI)_ . 18. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:32 

- [89] Fabio Palmese et al. 2025. Resource Optimization for Evidence Collection and Preservation in IoT Forensics-Ready Access Points. _IEEE Trans. Netw. Serv. Manag._ 22, 5 (2025), 44954508. 

- [90] Daniel Halperin, Wenjun Hu, Anmol Sheth, and David Wetherall. 2011. Tool release: Gathering 802.11 n traces with channel state information. _ACM SIGCOMM computer communication review_ 41, 1 (2011), 5353. 

- [91] Francesco Gringoli et al. 2019. Free your CSI: A channel state information extraction platform for modern Wi-Fi chipsets. In _Proceedings of the 13th International Workshop on Wireless Network Testbeds, Experimental Evaluation & Characterization_ . 2128. 

- [92] Jing Xu et al. 2019. Redundant sniffer deployment for multi-channel wireless network forensics with unreliable conditions. _IEEE Trans. Cognit. Commun. Networking_ 6, 1 (2019), 394407. 

- [93] Anmol Sheth et al. 2006. MOJO: A distributed physical layer anomaly detection system for 802.11 WLANs. In _Proceedings of the 4th international conference on Mobile systems, applications and services_ . 191204. 

- [94] Shaxun Chen, Kai Zeng, and Prasant Mohapatra. 2013. Efficient data capturing for network forensics in cognitive radio networks. _IEEE/ACM Transactions on Networking_ 22, 6 (2013), 19882000. 

- [95] Davi Monteiro, Yijun Yu, Andrea Zisman, and Bashar Nuseibeh. 2023. Adaptive observability for forensic-ready microservice systems. _IEEE Transactions on Services Computing_ 16, 5 (2023), 31963209. 

- [96] Xiang Zhou, Xin Peng, Tao Xie, Jun Sun, Chenjie Xu, Chao Ji, and Wenyun Zhao. 2018. Poster: Benchmarking microservice systems for software engineering research. In 2018 IEEE/ACM 40th International Conference on Software Engineering: Companion (ICSE-Companion). _IEEE, 323324_ (2018). 

- [97] Shixiong Wang et al. 2025. Uncertainty awareness in wireless communications and sensing. _IEEE Communications Magazine_ (2025). 

- [98] Clement Ruah, Osvaldo Simeone, and Bashir M Al-Hashimi. 2023. A Bayesian framework for digital twin-based control, monitoring, and data collection in wireless systems. _IEEE J. Sel. Areas Commun._ 41, 10 (2023), 31463160. 

- [99] Yichen Tian et al. 2025. CATS: Towards Accurate Device-Free Tracking by Quantifying the Sensing Confidence. _IEEE Trans. Mobile Comput._ (2025). 

- [100] Maximilian Stahlke et al. 2024. Uncertainty-based fingerprinting model monitoring for radio localization. _IEEE Journal of Indoor and Seamless Positioning and Navigation_ 2 (2024), 166176. 

- [101] Iratxe Landa et al. 2022. WIP: Impulsive noise source recognition with OFDM-WiFi signals based on channel state information using machine learning. In _2022 IEEE 23rd International Symposium on a World of Wireless, Mobile and Multimedia Networks (WoWMoM)_ . IEEE, 157160. 

- [102] Yun Wen et al. 2025. Exploring passive eves with self-refine sensing: A novel ISAC-aided secure communication system with STAR-RIS. _IEEE Trans. Wireless Commun._ (2025). 

- [103] Xianglin Yu et al. 2025. Learning-Based Predictive Beamforming for Secure ISAC via IRS. _IEEE Trans. Commun._ (2025). 

- [104] Yinghui He et al. 2023. Sencom: Integrated sensing and communication with practical wifi. In _Proceedings of the 29th Annual International Conference on Mobile Computing and Networking_ . 116. 

- [105] Chao-Kai Wen, Wan-Ting Shih, and Shi Jin. 2018. Deep learning for massive MIMO CSI feedback. _IEEE Wireless Communications Letters_ 7, 5 (2018), 748751. 

- [106] Zhenzhou Jin et al. 2025. Channel Fingerprint Construction for Massive MIMO: A Deep Conditional Generative Approach. _IEEE Trans. Wireless Commun._ (2025). 

- [107] Xiping Wang et al. 2023. Super-resolution of wireless channel characteristics: A multitask learning model. _IEEE Transactions on Antennas and Propagation_ 71, 10 (2023), 81978209. 

- [108] Wenhan Shen, Zhijin Qin, and Arumugam Nallanathan. 2023. Deep learning for super-resolution channel estimation in reconfigurable intelligent surface aided systems. _IEEE Trans. Commun._ 71, 3 (2023), 14911503. 

- [109] Taesik Nam et al. 2024. Data generation and augmentation method for deep learning-based VDU leakage signal restoration algorithm. _IEEE T INF FOREN SEC._ 19 (2024), 52205234. 

- [110] Yaoqi Yang, Bangning Zhang, Daoxing Guo, Hongyang Du, Zehui Xiong, Dusit Niyato, and Zhu Han. 2024. Generative AI for secure and privacy-preserving mobile crowdsensing. _IEEE Wireless Communications_ 31, 6 (2024), 2938. 

- [111] Cline Vanini et al. 2024. Was the Clock Correct? Exploring Timestamp Interpretation through Time Anchors for Digital Forensic Event Reconstruction. _Forensic Science International: Digital Investigation_ 49 (2024), 301759. 

- [112] Jie Xiong, Karthikeyan Sundaresan, and Kyle Jamieson. 2015. ToneTrack: Leveraging Frequency-Agile Radios for TimeBased Indoor Wireless Localization. In _Proceedings of the 21st Annual International Conference on Mobile Computing and Networking (MobiCom 15)_ . ACM, 537549. 

- [113] Ben Hilburn, Nathan West, Tim OShea, and Tamoghna Roy. 2018. SigMF: The Signal Metadata Format. _Proceedings of the GNU Radio Conference_ 3, 1 (Sept. 2018). 

- [114] Harsh Sanjay Pacherkar and Guanhua Yan. 2024. PROV5GC: Hardening 5G core network security with attack detection and attribution based on provenance graphs. In _Proceedings of the 17th ACM Conference on Security and Privacy in Wireless and Mobile Networks_ . 254264. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Intelligent Forensics in Next-Generation Mobile Networks: Evidence, Methods, and Applications 

35:33 

- [115] Amani Al-Shawabka et al. 2020. Exposing the Fingerprint: Dissecting the Impact of the Wireless Channel on Radio Fingerprinting. In _IEEE INFOCOM 2020  IEEE Conference on Computer Communications_ . IEEE, 646655. 

- [116] Wenqing Yan et al. 2022. RRF: A Robust Radiometric Fingerprint System that Embraces Wireless Channel Diversity. In _Proceedings of the 15th ACM Conference on Security and Privacy in Wireless and Mobile Networks_ . ACM, 8597. 

- [117] Maryam Amini, Razvan Stanica, and Catherine Rosenberg. 2024. Where Are the (Cellular) Data? _Comput. Surveys_ 56, 2 (2024), 48:148:25. 

- [118] Zhaowei Tan, Boyan Ding, Jinghao Zhao, Yunqi Guo, and Songwu Lu. 2022. Breaking Cellular IoT with Forged Data-plane Signaling: Attacks and Countermeasure. _ACM Transactions on Sensor Networks_ 18, 4 (2022), 59:159:26. 

- [119] Boris Danev, Heinrich Luecken, Srdjan Capkun, and Karim M. El Defrawy. 2010. Attacks on Physical-Layer Identification. In _Proceedings of the Third ACM Conference on Wireless Network Security (WiSec 10)_ . ACM, 8998. 

- [120] Chia-Cheng Yen et al. 2022. Graph Neural Network based Root Cause Analysis Using Multivariate Time-series KPIs for Wireless Networks. In _NOMS 2022-2022 IEEE/IFIP Network Operations and Management Symposium_ . IEEE, 17. 

- [121] Weili Wang et al. 2022. Real-Time Analysis of Multiple Root Causes for Anomalies Assisted by Digital Twin in NFV Environment. _IEEE Transactions on Network and Service Management_ 19, 2 (2022), 905921. 

- [122] Supriya Bajpai et al. 2024. AnomGraphAdv: Enhancing Anomaly and Network Intrusion Detection in Wireless Networks Using Adversarial Training and Temporal Graph Networks. In _Proceedings of the 17th ACM Conference on Security and Privacy in Wireless and Mobile Networks (WiSec 24)_ . ACM, 113122. 

- [123] Shuokang Huang et al. 2023. DiffAR: Adaptive Conditional Diffusion Model for Temporal-augmented Human Activity Recognition. In _Proceedings of the Thirty-Second International Joint Conference on Artificial Intelligence_ . IJCAI, 38123820. 

- [124] Brian Kim et al. 2022. Channel-Aware Adversarial Attacks Against Deep Learning-Based Wireless Signal Classifiers. _IEEE Transactions on Wireless Communications_ 21, 6 (2022), 38683880. 

- [125] Jie Ma and otehrs. 2023. White-Box Adversarial Attacks on Deep Learning-Based Radio Frequency Fingerprint Identification. In _2023 IEEE International Conference on Communications (ICC)_ . IEEE, 37143719. 

- [126] Tianya Zhao et al. 2025. Explanation-Guided Backdoor Attacks Against Model-Agnostic RF Fingerprinting Systems. _IEEE Transactions on Mobile Computing_ 24, 3 (2025), 20292042. 

- [127] Changming Li et al. 2024. Practical Adversarial Attack on WiFi Sensing Through Unnoticeable Communication Packet Perturbation. In _Proceedings of the 30th Annual International Conference on Mobile Computing and Networking (MobiCom 24)_ . ACM, 373387. 

- [128] Simson L. Garfinkel. 2012. Digital forensics XML and the DFXML toolset. _Digital Investigation_ 8, 3-4 (2012), 161174. 

- [129] Eoghan Casey et al. 2017. Advancing coordinated cyber-investigations and tool interoperability using a community developed specification language. _Digital Investigation_ 22 (2017), 1445. 

- [130] Azadeh Tabiban et al. 2022. VinciDecoder: Automatically Interpreting Provenance Graphs into Textual Forensic Reports with Application to OpenStack. In _Secure IT Systems  27th Nordic Conference, NordSec 2022_ . Springer, 346367. 

- [131] Chuan Guo et al. 2017. On Calibration of Modern Neural Networks. In _Proceedings of the 34th International Conference on Machine Learning (Proceedings of Machine Learning Research, Vol. 70)_ . 13211330. 

- [132] Scott Kuzdeba et al. 2022. Systems View to Designing RF Fingerprinting for Real-World Operations. In _Proceedings of the 2022 ACM Workshop on Wireless Security and Machine Learning_ . ACM, 3338. 

- [133] Saeif Al-Hazbi et al. 2024. Radio Frequency Fingerprinting via Deep Learning: Challenges and Opportunities. In _20th International Wireless Communications and Mobile Computing Conference (IWCMC 2024)_ . IEEE, 824829. 

- [134] Joe Breen et al. 2021. Powder: Platform for Open Wireless Data-driven Experimental Research. _Computer Networks_ 197 (2021), 108281. 

- [135] Leonardo Bonati et al. 2021. Colosseum: Large-Scale Wireless Experimentation Through Hardware-in-the-Loop Network Emulation. In _2021 IEEE International Symposium on Dynamic Spectrum Access Networks (DySPAN)_ . IEEE, 105113. 

- [136] Jakob Hoydis et al. 2022. Sionna: An Open-Source Library for Next-Generation Physical Layer Research. _arXiv preprint arXiv:2203.11854_ (2022). 

- [137] Timnit Gebru et al. 2021. Datasheets for Datasets. _Commun. ACM_ 64, 12 (2021), 8692. 

- [138] Margaret Mitchell et al. 2019. Model Cards for Model Reporting. In _Proceedings of the Conference on Fairness, Accountability, and Transparency_ . ACM, 220229. 

- [139] Bo Xu et al. 2024. Towards explainability for AI-based edge wireless signal automatic modulation classification. _Journal of Cloud Computing_ 13, 1 (2024), 10. 

- [140] Tianya Zhao, Xuyu Wang, and Shiwen Mao. 2024. Cross-domain, Scalable, and Interpretable RF Device Fingerprinting. In _IEEE INFOCOM 2024  IEEE Conference on Computer Communications_ . IEEE, 20992108. 

- [141] Sreeraj Rajendran, Wannes Meert, Vincent Lenders, and Sofie Pollin. 2019. Unsupervised wireless spectrum anomaly detection with interpretable features. _IEEE Trans. Cognit. Commun. Networking_ 5, 3 (2019), 637647. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

J. Wang et al. 

35:34 

- [142] Sreeraj Rajendran, Vincent Lenders, Wannes Meert, and Sofie Pollin. 2019. Crowdsourced wireless spectrum anomaly detection. _IEEE Trans. Cognit. Commun. Networking_ 6, 2 (2019), 694703. 

- [143] Xuanhan Zhou et al. 2021. A radio anomaly detection algorithm based on modified generative adversarial network. _IEEE Wireless Communications Letters_ 10, 7 (2021), 15521556. 

- [144] Pedro VA Alves et al. 2023. Machine learning applied to anomaly detection on 5g o-ran architecture. _Procedia Computer Science_ 222 (2023), 8193. 

- [145] Md Rakibul Ahasan et al. 2022. Supervised learning based mobile network anomaly detection from key performance indicator (KPI) data. In _2022 IEEE Region 10 Symposium (TENSYMP)_ . IEEE, 16. 

- [146] Jiajia Huang, Ernest Kurniawan, and Sumei Sun. 2022. Cellular kpi anomaly detection with gan and time series decomposition. In _ICC 2022-IEEE International Conference on Communications_ . IEEE, 40744079. 

- [147] Kazi Samin Mubasshir and other. 2025. Gotta Detect Em All: Fake Base Station and Multi-Step Attack Detection in Cellular Networks. In _Proceedings of the 34th USENIX Security Symposium_ . USENIX Association, Seattle, WA, USA. 

- [148] Zhenhua Li and otherso. 2017. FBS-Radar: Uncovering Fake Base Stations at Scale in the Wild.. In _NDSS_ . 

- [149] Shamnaz Riyaz, Kunal Sankhe, Stratis Ioannidis, and Kaushik Chowdhury. 2018. Deep learning convolutional neural networks for radio identification. _IEEE Communications Magazine_ 56, 9 (2018), 146152. 

- [150] Joshua Groen et al. 2024. TRACTOR: Traffic analysis and classification tool for open RAN. In _ICC 2024-IEEE International Conference on Communications_ . IEEE, 48944899. 

- [151] Ting-Li Huoh, Yan Luo, Peilong Li, and Tong Zhang. 2022. Flow-based encrypted network traffic classification with graph neural networks. _IEEE Trans. Netw. Serv. Manag._ 20, 2 (2022), 12241237. 

- [152] Iman Akbari et al. 2021. A look behind the curtain: Traffic classification in an increasingly encrypted web. _Proceedings of the ACM on Measurement and Analysis of Computing Systems_ 5, 1 (2021), 126. 

- [153] Haipeng Li et al. 2022. RadioNet: Robust deep-learning based radio fingerprinting. In _2022 IEEE Conference on Communications and Network Security (CNS)_ . IEEE, 190198. 

- [154] Manikanta Kotaru et al. 2015. Spotfi: Decimeter level localization using wifi. In _Proceedings of the 2015 ACM conference on special interest group on data communication_ . 269282. 

- [155] Frost Mitchell et al. 2022. Deep learning-based localization in limited data regimes. In _Proceedings of the 2022 ACM workshop on wireless security and machine learning_ . 1520. 

- [156] Andrea Nardin, Tales Imbiriba, and Pau Closas. 2023. Jamming source localization using augmented physics-based model. In _ICASSP 2023-2023 IEEE International Conference on Acoustics, Speech and Signal Processing_ . IEEE, 15. 

- [157] Bernardo Camajori Tedeschini et al. 2024. Real-time Bayesian neural networks for 6G cooperative positioning and tracking. _IEEE J. Sel. Areas Commun._ 42, 9 (2024), 23222338. 

- [158] Mohammad Amin Maleki Sadr et al. 2021. Uncertainty estimation via Monte Carlo dropout in CNN-based mmWave MIMO localization. _IEEE Signal Processing Letters_ 29 (2021), 269273. 

- [159] Jiacheng Wang et al. 2023. Through the wall detection and localization of autonomous mobile device in indoor scenario. _IEEE J. Sel. Areas Commun._ 42, 1 (2023), 161176. 

- [160] Matthias Schulz, Daniel Wegemer, and Matthias Hollick. 2017. Nexmon: Build your own wi-fi testbeds with low-level mac and phy-access using firmware patches on off-the-shelf mobile devices. In _Proceedings of the 11th Workshop on Wireless Network Testbeds, Experimental evaluation & CHaracterization_ . 5966. 

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

_NDA_ defeats all 25 documented [8, 12, 45, 54, 64] speculative execution attacks without the need to modify any existing code. Importantly, however, _NDA_ does not preclude all speculation or OoO execution. For example, one _NDA_ policy treats all instructions after an unresolved branch as unsafe. These instructions may still execute speculatively OoO, but they are restricted from propagating their output to dependents until all preceding branches resolve. As our evaluation demonstrates, despite delayed wake-ups, the vast majority of the performance gap between in-order (the only other model known to eliminate all known speculative execution attacks) and unconstrained OoO execution is recovered. 

We simulate _NDA_ designs on the SPEC CPU 2017 benchmark suite and compare its performance to InvisiSpec [69] on the same setup. InvisiSpec blocks data-cache-based attacks and introduces 7.6-32.7% overhead in our setup. In comparison, _NDA_ blocks _all_ covert channels. We show that an _NDA_ policy that mitigates controlsteering vulnerabilities, which are fundamental to unconstrained OoO execution, slows execution by only 10.7% and is 4 _._ 8 __ faster than in-order. If we also preclude Meltdown-like hardware implementation flaws, _NDA_ s strictest policy slows down execution by 125% compared to an insecure OoO processor and is 2 _._ 4 __ faster than in-order execution. 

In short, we make the following contributions: 

- We introduce a speculative-execution-attack taxonomy based on how attacks induce wrong-path execution. 

- We design _NDA_ , a new technique to control speculative data propagation in out-of-order processors to defeat speculative execution attacks. _NDA_ offers multiple variants with differing security/performance tradeoffs. 

- We evaluate six _NDA_ variants on SPEC 2017 and show they are effective and efficient. 

### **2 BACKGROUND** 

**Data Propagation in OoO Processors.** Fig. 2 illustrates conceptual steps in an instructions life-cycle in a modern OoO processor. Upon dispatch into the reorder buffer (ROB), an instruction is not ready to execute until all of its source operandscoming from instructions S1 and S2 in Fig. 2are ready (step 1). Once all source operands are ready, the instruction issues and enters the execution pipeline (step 2). When execution completes (step 3), the instruction wakes its dependents (D1-D5) by broadcasting a tag corresponding to its destination physical register to waiting instructions (step 4), marking those instructions ready. 

The essence of the _NDA_ technique is to _delay tag broadcast_ , i.e., transition from step 3 to step 4. Rather than waking dependent instructions when their input operands become _ready_ , _NDA_ wakes them up when their input operands are _safe_ . We expand on this basic concept in 5. 

**Speculative Execution Attacks** . Speculative execution attacks exploit side-effects of wrong-path execution, which are typically left undefined by processor vendors. While the contents of architectural registers and memory are guaranteed to reflect precise state of only committed instructions, wrong-path execution affects microarchitectural structures. For instance, a wrong-path cache access 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
(1) Source inputs not r eady (2)  Sources      e x ecuting. Not  r eady & instruction     c ompleted yet<br>S1 D1 S1 Ready D1<br>inst. inst.<br>S2 D5 S2 Ready D5<br>r x c b r x c b<br>(3) Instruction   output not  bc rooadcastmpleted,     (4)  C ompleted &  b roadcast<br>S1 Ready D1 S1 Ready D1<br>inst inst<br>S2 Ready D5 S2 Ready D5<br>r x c b r x c b<br>Ready<br>Ready<br>. .<br>. .<br><!-- End of picture text -->

**Figure 2: Life-cycle of instructions in OoO processors. Even after the instruction has completed execution (3), the dependant instructions (D1-D5) will not be able to access the output until it is broadcast (4).** 

may allocate new lines or modify the cache replacement order; these changes are not reverted when wrong-path instructions are squashed. A variety of other micro-architectural structures are also not reverted during squash, for example, branch direction predictors (e.g., pattern history table), pre-decoded micro-op/trace caches, memory dependence predictors, prefetchers, TLBs, fine-grain power management state (e.g., for FPU/AVX units), and performance counters. State changes in these micro-architectural structures can create side channels, where the state can be inferred, for example, based on timing particular execution sequences. We refer to a side channel that is used to intentionally transmit data as a _covert channel_ . Attackers can use _wrong-path_ execution to transmit data, via a covert channel, that is later inferred by _correct-path_ execution and hence leaks that data into architectural state. 

### **3 PROBLEM ANALYSIS** 

We next classify speculative execution attacks based on three fundamental attack phases that exist in all known attacks. We then describe the existing mitigation techniques, how they block the attacks, and their shortcomings. Lastly, to demonstrate that closing specific side channels is insufficient, we show an attack via a new covert channelthe BTB. 

### **3.1 Classifying Attacks** 

**Attack Phases.** All speculative execution attacks of which we are aware comprise three key phases _access, transmit,_ and _recover_  shown in Fig. 3. In the _Access Phase_ ( 1 __ ), secret data is loaded into a temporary register. During the _Transmit Phase_ ( 2 __ ) the secret data is covertly transmitted using micro-architectural side effects that are not reverted when wrong-path instructions are squashed. Finally, in the _Recover Phase_ ( __ 3 ), the transmitted secret is recovered to non-speculative state (e.g., by observing the memory access latency). Whereas the instructions involved in phases __ 1 and __ 2 are speculatively executed and eventually squashed, the phase __ 3 results 

<!-- Start of picture text -->
Access Phase:  Restricted<br>1<br>Speculatively read secret into a  memory/register<br>physical register Load secret  s<br>Physical register<br>Pre-process e.g.,<br>Transmit Phase:  s=(s&0xFF)*512s=&probe[s] 2<br>Speculatively transmit secret via a<br>covert channel. Preprocessing  Physical register<br>may be required Transmit via a<br>covert channel E.g:<br>T = *sT = s<br>Cache FPU Ports BTB TLB PHT ...<br>Probe covert channel<br>Recover Phase:<br>Receive covert transmission<br>Attackers Memory 3<br>non-speculatively<br><!-- End of picture text -->

**Figure 3: Three phases of speculative execution attacks. Prior defenses focus mostly on the cache covert channel, failing to prevent leaks through other channels such as the FPU [55], the BTB (3), and others.** 

are committed to the architectural state. Wrong-path execution is essential to these attacks, as it evades software and hardware protection mechanisms that prevent the secret data from leaking through architectural state. 

**Control-Steering and Chosen-Code Attacks.** We classify attacks based on their methodology for performing the _Access Phase_ ( __ 1 ) and the _Transmit Phase_ ( __ 2 ). We divide attacks based on their _Access Phase_ into two categories, which correspond to different attacker threat models. We further subdivide these two attack classes according to the covert channel exploited in the _Transmit Phase_ . Table 1 illustrates this taxonomy for currently-known attacks. 

In _control-steering_ attacks, the attacker subverts a victim programs control flow to speculatively execute instructions that, as a side-effect, leak data into a covert channel. This attack class leaks data to which the victim application has hardware access privileges, but are intended to be secret and might be protected (e.g., by permission or bounds checks) in software. For example, SGXPectre [13] coerces a secure SGX [42] enclave to access and leak its encrypted memory. We illustrate control-steering attacks in Fig. 1a. 

Unlike a classical security vulnerability, wherein the attacker directly hijacks the program counter (e.g., a stack-smashing attack that overwrites a return address), speculative control-steering attacks only misdirect wrong-path execution, for example, by mis-training branch predictors to direct instruction fetch to an attacker-selected target. Hence, they leave no trace in the committed instruction sequence, but still leak data into a covert channel. Several approaches that use control-steering have been demonstrated [3335, 38]. 

In control-steering attacks, the attacker does not typically introduce new instructions into the victim binary, rather, the attacker composes a series of gadgets from existing code, similar to Return Oriented Programming (ROP [11, 52, 56]). 

By contrast, in _chosen-code_ attacksour second category based on the _Access Phase_ we consider an adversary who can generate 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
1 for (i=0; i < 256; i++) // init channel<br>2 clflush(probeArray[i*512]);<br>3 // Phase  1 - access secret data:<br>4 // The attacker mis-trains the branch:<br>5 if (x < array_size) { // predicted taken<br>6 // wrong-path, x >= array_size<br>7 secret = array[x];<br>8 // Phase  2 - covertly transmit secret:<br>9 t = probeArray[secret * 512];<br>10 }<br>11 // ... somewhere else in attacker's code<br>12 // Phase  3 - recover secret:<br>13 for (guess = 0; guess < 256; guess++) {<br>14 addr = &probeArray[guess*512];<br>15 t1 = rdtscp(); // read timer<br>16 temp = *addr; // access probing array<br>17 t2 = rdtscp(); // read timer<br>18 if (t2-t1 <= CACHE_HIT_THRESHOLD)<br>19 results[guess] += 1;<br>20 }<br><!-- End of picture text -->

**Listing 1: Exfiltrating secret data using Spectre v1** **_controlsteering_ and the cache covert channel.** 

<!-- Start of picture text -->
1 // Phase  1 - access secret:<br>2 secret = *kernel_addr; // Faulting load<br>3 // Phase  2 - covertly transmit secret:<br>4 // Executed in wrong-path<br>5 // before fault is fired:<br>6 t = probeArray[secret * 512];<br>7 // Phase  3 - recover secret:<br>8 // see Listing 1<br><!-- End of picture text -->

**Listing 2: Exfiltrating secret data using the Meltdown** **_chosencode_ attack and a cache side-channel.** 

and execute arbitrary code sequences to mount the attack. Such an adversary already has access to its own registers and memory; these attacks instead seek to circumvent _hardware_ protections that preclude the attacker from accessing secret data in correctpath code. For instance, Meltdown [36] accesses kernel memory; Foreshadow [25, 62, 65] accesses SGX and hypervisor memory; and LazyFP [59] accesses AVX registers used by another process. These attacks exploit implementation flaws in the relative timing of hardware protection checks and data flow between wrong-path instructionsthe secret data propagates among instructions and can be leaked into a covert channel before protection checks squash the wrong-path execution. We show chosen-code attacks in Fig. 1b. 

**Sample Attack Code.** Listing 1 illustrates these phases for the Spectre v1 [34] bounds check bypass attack [27], which is a controlsteering attack. In this attack, the victim code includes instructions that access array at a given index x (Line 7). Before accessing array, the victim code performs a bounds check on x (Line 5). To circumvent the bounds check, the attacker mis-trains the branch direction-predictor by invoking the victim code repeatedly with a valid x. 

To mount the attack, the attacker now calls the victim code with an illegal value of x. The attacker chooses x such that array[x] will refer to a location in the victims memory containing a secret. The direction predictor mis-predicts the branch on Line 5 as taken, executing Lines 79 on the wrong path. During wrong-path execution, the code _accesses_ ( 1 __ ) the secret on Line 7. It then _transmits_ ( 2 __ ) the secret (still in wrong-path) on Line 9. Later, in correct-path execution, the attacker executes Lines 1320 to _recover_ ( 3 __ ) the secret from the cache side-channel. The timing for each access to probeArray on Line 16 will vary based on whether or not the corresponding cache line was loaded on Line 9. In our evaluation, 

<!-- Start of picture text -->
Phase  2<br>Phase  1<br>Attack<br>Spectre v1 [34]<br>Spectre v1.1 [33]<br>Spectre v2 [34,39]<br>Control  Ret2spec [35,38]<br>steering NetSpectre [55]<br>SMoTher Spectre [8]<br>SSB (Spectre v4) [27]<br><future attacks><br>Meltdown (v3 / v3a) [27,36]<br>Chosen LazyFP[59]<br>code Foreshadow (L1TF) [25,62,65]<br>MDS attacks [45,54,64]<br><future attacks><br>- demonstrated in prior work;         - demonstrated in this work<br>- d-cache-based attacks are defeated by prior work [31,48,53,69]<br>d-cache<br>i-cache FPU Ports BTB<br><!-- End of picture text -->

**Table 1: Taxonomy of attacks based on secret data access method** **__ 1 and covert channel** **__ 2 .** **_NDA_ blocks all existing attacks regardless of the covert channel they use. Most common attacks use the d-cache side channel to exfiltrate secret data. All currently known chosen-code attacks use loads and load-like operations. Future attacks may use other instructions or other covert transmission channels.** 

we illustrate the difference in access timing (blue squares in Fig. 4), which reveals the secret data. 

Listing 2 depicts an example of a chosen-code attacka simplified Meltdown exploit. Whereas the illegal load on Line 2 will eventually fault, the instruction on Line 6which executes on the wrong pathwill leave evidence in the cache from which the attacker can recover the secret. The _recover_ phase is identical to that in Listing 1. To avoid trapping into the fault handler, the attacker may use control-steering techniques to ensure the faulting load executes under a mis-predicted branch [36]. Nevertheless, we classify the attack as chosen-code since the attacker controls the executed binary. 

### **3.2 Limitations of Existing Defenses** 

**Current Mitigations.** Hardware defenses mitigating control-steering attacks try to prevent the attacker from mis-training branch predictors (IBRS and STIBP [29]) or use a barrier instruction to prevent speculation after a branch or context switch (lfence/IBPB [29]). Unfortunately, recent attacks [33, 35, 38] reveal techniques to overcome these mitgations. SSBD [6, 29] disables Speculative Store Bypass (SSB, explained in 4.1) to prevent attackers from reading data that was overwritten [27, 75]. However, SSBD only blocks Spectre v4. and introduces up to 8% overhead [26]. 

Software defenses, such as Retpoline [21] and RSB stuffing [28], protect call and ret instructions from mis-steering. Other compiler approaches [22, 46] create a data dependency between a branch condition and code that follows the branch, disabling speculation. However, these compiler approaches can only defeat Spectre v1 [34] attacks. A recent study suggested a compiler modification that also 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
Cache Cache<br>Cache Secret Byte (42) BTB<br>BTB BTB Secret Byte (42)<br><!-- End of picture text -->

**Figure 4: Spectre v1, using either the cache (blue squares) or the BTB (orange circles) as a covert channel. For the cache channel, only the correct guess produces a cache hit, creating the cycle difference**  _Cache_ **. For the BTB channel, only the correct guess successfully predicts the jump target, creating the cycle difference**  _BTB_ **.** 

blocks Spectre v2 attacks [57]. Unfortunately, this approach can only defeat cache-based attacks with 68-247% overhead. 

Chosen-code attacks are mitigated by preventing speculative loads from accessing restricted memory. For instance, Kernel Address Space Layout Randomization (KASLR [23]) and Kernel Page Table Isolation (KPTI [37, 43]) prevent Meltdown attacks from reading privileged kernel memory. KASLR [23] randomizes the kernel address space similar to how ASLR is used to protect user-space processes. KPTI manages separate page tables for the kernel and userspace processes, preventing user code from issuing even illegal loads to kernel memory. KPTI swaps page tables on every transfer between CPU privilege levels. Mitigating Foreshadow [25, 62, 65] requires modifications to the OS, hypervisor, and SMM code, such as modifying page-table management, altering virtual machine scheduling, and adding L1 cache flushes when switching security domains [25, 65]. 

Unfortunately, all these defense mechanisms block only specific exploit techniques. Therefore, one must deploy a myriad software and hardware defenses to be resilient against _all_ control-steering and chosen-code attacks. 

Recent work suggests preventing both control-steering and chosencode attacks by blocking the cache side channel [31, 48, 53, 69], thus interdicting the _transmit_ phase. However, given the abundant supply of covert channels (see Fig. 3), defeating speculative attacks by closing each channel individually is challenging. Exploits have already been demonstrated for other channels. Netspectre [55] demonstrated that the power state of the FPU is a viable speculative covert channel. SMoTher Spectre [8] showed how to transmit data via port contention [4]. We next show an attack via the BTB. 

**The BTB Covert Channel.** We demonstrate a new covert channel that can be exploited even when the cache covert channel is not availablethe BTB. The BTB stores a mapping between branch instructions addresses and the associated target addresses. For example, a call instruction located at address A to a function located at address B installs the mapping A => B in the BTB. The next time the processor fetches the call instruction at address A, the processors front-end will speculatively redirect fetch to address B. 

If the BTB predicts correctly (Fig. 5a), the speculatively-fetched instructions are eventually retired. However, if the prediction is 

<!-- Start of picture text -->
a) Correct BTB prediction<br>jumpToTarget<br>1 predict<br>correctTarget<br>b) Incorrect BTB prediction<br>Overhead of mis-prediction:<br>Wrong-path<br>1 predict<br>wrongTarget<br>jumpToTarget 2 squash<br>3<br>correctTarget<br><!-- End of picture text -->

**Figure 5: The BTB covert channel. The attacker can observe if the BTB prediction was correct by measuring execution time.** 

wrong, the processor will squash the wrong-path execution, starting at the mispredicted instruction at address B, before executing the correct path. This recovery process is illustrated in Fig. 5b. In our experiments on the _gem5_ [9] simulator, we observe that it takes ~ 16 cycles for the BTB miss to resolve, wrong-path execution to be squashed, and execution to resume at the correct target (1 + 2 in Fig. 5b). Crucially, updates to the BTB during speculation are not reverted by the squash, making it an effective covert channel. Note that (as with caches) in the absence of security concerns, filling the BTB (and updating its replacement policy) during speculation may be advantageous to avoid future BTB misses. 

To demonstrate the BTB covert channel, we construct a variant of Spectre v1 [34] that leaks a secret byte through a speculative BTB update, as illustrated in Listing 3. To leak a single byte, our covert channel comprises 256 distinct functions (targets in Line 2). During both the _Transmit Phase_ and _Recover Phase_ , we invoke targets only from a single call site, jumpToTarget (Line 6), ensuring that BTB entries mapping to targets all originate from the same PC and therefore conflict in the BTB. 

When the branch on Line 10 is mispredicted, the attacker can _access_ any value from the process address space, depending on the value of x. The attacker then _transmits_ the secret by speculatively calling jumpToTarget with the secret value in Line 13. If the speculation window is large enough, the processor updates the BTB entry for the call instruction in Line 6 based on secret. 

The _access_ phase must be repeated for every guess (Line 19) since the _recover_ phase is destructive: The execution of Line 21 alters the contents of the BTB to point to targets[guess]. To confirm that the BTB acts as the covert channel in our attack, it is important to validate that execution time differences do _not_ arise from i-cache or d-cache hit or miss latency; no change to the cache hierarchy during the attack may depend upon the secret value. To validate our attack, we ensure the targets array in Line 2 and all 256 target functions are cached during access, transmission, and recovery. 

We report the effectiveness of the BTB covert channel on _gem5_ via the orange circles in Fig. 4. During the _Recover Phase_ , in lines 17-24, all the _wrong_ guesses will incur the 16-cycle prediction and squashing delay, as depicted in Fig. 5b. The _correct_ guess will execute faster, as depicted in Fig. 5a. 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
1 // array of 256 unique target functions<br>2 void (*targets[256])( void );<br>3 // all jumps are from the same location,<br>4 // hence the same BTB entry is consulted<br>5 void jumpToTarget( int index)<br>6 { targets[index](); }<br>7 void victim_function(x) {<br>8 // Phase  1 - access secret data:<br>9 // The attacker mis-trains the branch:<br>10 if (x < array_size) { // predicted taken<br>11 secret = array[x]; // wrong path<br>12 // Phase  2 - covertly transmit secret:<br>13 jumpToTarget(secret); // updates BTB<br>14 } }<br>15 // ... somewhere else in attacker's code<br>16 // Phase  3 - recover secret:<br>17 for (guess = 0; guess < 256; guess++) {<br>18 // Induce victim to leak secret value<br>19 victim_function(x);<br>20 t1 = rdtscp(); // read timer<br>21 jumpToTarget(guess); // BTB prediction<br>22 t2 = rdtscp(); // read timer<br>23 if (t2-t1 <= CORRECT_PATH_THRESHOLD)<br>24 results[guess] += 1;<br>25 }<br><!-- End of picture text -->

**Listing 3: Exfiltrating secret data using the Spectre v1** **_controlsteering_ attack and the BTB side-channel.** 

The BTB covert channel is one of many potential machine-specific transmission channels. We use our BTB channel PoC to demonstrate that _NDA_ is agnostic to any specific transmission channel (6). 

### **4 THREAT MODELS** 

_NDA_ design variants address four different threat models, which vary in the locations from which secret data are stolen and whether the attacker may mount control-steering or chosen-code attacks. _NDA_ s goal is to eliminate side-channels created in _wrong-path_ execution. Correct-path side channels have been studied in prior work [51, 70, 71]. 

All threat models are agnostic to the covert channel used in the attacks. For control-steering attacks, we consider two threat models, based on where secrets reside. The first model considers attacks against secrets stored in memory or special registers, as is the case for all currently-known control-steering attacks. Our second controlsteering threat model additionally considers hypothetical attacks that leak secrets residing in general-purpose registers (GPRs). In our third threat model, for chosen-code attacks, we consider only threats against secrets in privileged memory and registers, since chosencode attacks presuppose attacker-controlled GPRs. Lastly, our fourth threat model comprises the union of these threats, considering both control-steering and chosen-code attacks for secrets in memory, special-registers, and GPRs. 

### **4.1 Leaking Memory via Control-Steering** 

The first step of all known control-steering attacks is to steer wrongpath execution into code accessing a secret in memory or manipulate execution timing to cause a load to observe a stale value. We assume the attacker can steer execution at any branch instruction and manipulate the execution timing of all instructions. Branch instructions include all variants of jmp, call, and ret. 

We do not consider _phantom branches_ , where the BTB is mistrained to steer control flow from a program counter value that does not correspond to a branch. The dispatch stage stalls micro-ops whose opcode is unknown. Hence, if the BTB predicts a branch where there is none, dispatch will stall at the phantom branch until 

its opcode is obtained, which will resolve the misprediction and cause any younger fetched instructions to be discarded before they enter the OoO back-end. Wrong-path instructions that are squashed before dispatch are not a threat. 

We also do not consider potentially faulting instructions as steering points in control-steering attacks. Whereas a fault can result in wrong-path execution, we consider attacks based on faulting instructions (e.g., Meltdown, Foreshadow, LazyFP, MDS, etc.) as part of the threat model for chosen-code attacks. 

**Speculative Store Bypass.** Also known as SSB, or Spectre variant 4 [27], this attack performs the _Access Phase_ ( __ 1 in Fig. 3) by having a malicious speculative load bypass a store whose address is still unresolved. The malicious load then speculatively yields stale (secret) data. Although this attack may not necessarily require misdirected control flow in the _Access Phase_ , we consider it a special case of control-steering, since the attacker leverages an existing code snippet. If the attacker could choose the code, they could read the stale data without the need to exploit the speculative store-bypass. 

### **4.2 Leaking GPRs via Control-Steering** 

All currently-known control-steering attacks extract secrets residing in memory. Nevertheless, we recognize that future attacks might extract secrets residing in the victims GPRs. So, our second threat model considers the attacker of 4.1 that steers the victims control flow to leak GPR contents. 

In this scenario, the steered victims code already possesses the secret in a GPR. At this point, the access phase of the control-steering attack ( 1 __ in Fig. 3) has already (possibly unintentionally) been done by the victim. We therefore focus on hindering the attacker from performing the second phase ( __ 2 in Fig. 3)transmitting the GPR-resident secret. All known attacks require data flow between micro-ops during the transmit phase to preprocess the secret (e.g., calculate an offset relative to a base address) before it can be leaked. 

We do not prevent an attack that leaks a secret using only a single speculative micro-op. In principle, it may be possible to covertly transmit GPR-based secrets using a single micro-op. For instance, if a GPR contains a secret value that corresponds to a valid virtual memory address, the attacker can speculatively issue a load that will fetch this address into the cache hierarchy, thus performing the transmit phase in a single micro-op. However, such a scenario would require (a) a secret value that forms a valid memory address, and (b) victim code that voluntarily loads the secret into a GPR shortly before the vulnerable steering point. No known attacks (cf. Table 1) exploit this behavior. 

### **4.3 Leaking Memory with Chosen-Code** 

For chosen-code attacks, we consider attackers that attempt to access secrets residing in memory. Specifically, we consider an attacker who can influence code generation to control both correct-path and wrong-path execution. We treat read operations from specialpurpose registers, such as AVX (as abused in LazyFP [59]) and Model Specific Registers (MSRs, in Meltdown variant 3a [27]) like memory accesses in crafting our defensethe special instructions (e.g., rdmsr) used to access these registers are treated like loads in our solution. In chosen-code attacks, the attacker already controls 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

their own GPRs and we therefore do not consider the contents of any GPR to be secret. 

Instructions are guaranteed to be correct-path when they retire. At retirement, the head of the ROB satisfies _hardware_ permission and memory-ordering checks. Ergo, retired instructions cannot leak secrets accessed from the _wrong-path_ . 

### **4.4 Combining the Threat Models** 

Finally, we consider _NDA_ s most conservative threat modela combination of all threats outlined above. We suppose an attacker that conducts both (a) control-steering attacks to extract secrets from the victims memory and GPRs, _and_ (b) chosen-code attacks to access privileged memory and special registers. This combined threat model is similar to the practical approach taken by Windows and Linux, which deploy mitigations for both classes of attacks [29, 37, 43, 44, 61]. 

### **5 DESIGN** 

_NDA_ s main design goal is to mitigate both control-steering and chosen-code attacks while reaping the benefits of OoO speculative execution as much as possible. We next discuss different variants of _NDA_ , which provide different policies for speculative data propagation depending on the threat model. Different _NDA_ data propagation policies offer different security guarantees and have corresponding performance implications. We build _NDA_ upon a baseline physical register-based OoO micro-architecture [74]. 

The key insight behind _NDA_ s design is that speculative instructions (either in the _correct_ or the _wrong_ -path) can safely execute without leaking secrets as long as their inputs are results of _safe_ instructions. We define instructions as _safe_ with respect to our threat models such that wrong-path execution can not leak any more information into a side channel than a correct-path instruction. Consequently, we eliminate the gap between _speculative_ side channel attacks and _non-speculative_ side channels, which security-conscious programmers already must reason about. The different _NDA_ policies, listed in rows 1-6 of Table 2, define which instructions are considered _safe_ such that they may wake-up dependent instructions (allow instructions to advance from step 3 to step 4 in Fig. 2). 

To mitigate control-steering attacks, _NDA_ restricts data propagation following an unresolved branch or unresolved store address (rows 1-4 in Table 2), depending on where secrets reside and if store-bypass (SSB) is a threat. We consider any instruction following a predicted branch as _unsafe_ until the branch target and direction are resolved. We also consider loads that follow a store with an unresolved address as _unsafe_ (see Bypass Restriction in 5.2). To mitigate chosen-code attacks, _NDA_ introduces a _propagate-on-retire_ mechanism (row 5), which defeats all 11 documented chosen-code attack variants [12, 45, 54, 64] and similar future exploits that rely on speculative loads. In this policy, the value returned by _any_ load instruction (or other instructions that read sensitive registers, such as rdmsr on x86) are considered _unsafe_ until the load is ready to retire. Finally, the two mechanisms can be combined to defend against both classes of attacks (row 6). 

### **5.1 Strict Data Propagation** 

_NDA_ addresses control-steering attacks by defining unresolved branches and unresolved storesfor which predictions may be incorrectas the borders between safe and unsafe speculation. When a branch micro-op enters the ROB, it is _unresolved_ . Since the fetch unit predicts which instructions to fetch following the branch (via the BTB, RSB, etc.), subsequently dispatched micro-ops may be wrong-path. Similarly, when a store micro-op enters the ROB, it is _unresolved_ until its address is calculated. If a stores address has not been calculated, loads that follow the store may erroneously access stale data if their addresses overlap. We consider two variants of data propagation restrictions with regards to control-steering attacks: strict and permissive. Both variants leverage a _Bypass Restriction_ mechanism to defeat SSB attacks. We now describe strict propagation and then explain permissive propagation and bypass restriction in 5.2. 

_Strict Propagation_ (rows 3-4 in Table 2) defends against threat models where secrets may reside in memory, special registers, and GPRs (i.e., the union of the threats described in 4.1 and 4.2). Under this policy, _NDA_ marks _all_ micro-ops dispatched after an unresolved branch or store as _unsafe_ . Unsafe instructions may wake up and compete to issue as in a baseline OoO (i.e., they may issue when their operands become ready). But, when an unsafe micro-op completes execution (step 3 in Fig. 2), it writes back to its destination physical register, but does not broadcast its destination tag to dependent instructions, does not mark its destination register ready, and does not forward its output value on the bypass network. Hence, dependent instructions will not issue and cannot observe the unsafe value. 

**Managing Value Propagation.** When the eldest outstanding micro-op resolves, it marks instructions in the ROB _safe_ until the next eldest unresolved branch/store. ROB entries are extended with three bits: unsafe tracks if the instruction follows a still-unresolved micro-op, exec tracks if the instruction has executed, and bcast tracks if the instruction has broadcast its tag to wake dependents. Upon instruction completion, if unsafe, tag broadcast is deferred. When a micro-op resolves, the unsafe bit for subsequent ROB entries until the next unresolved branch/store are cleared. !unsafe && exec && !bcast instructions arbitrate for tag broadcast ports, competing with instructions completing in the current cycle (completing instructions have priority to avoid pipeline stalls); bcast is set when broadcasting. 

When _safe_ instructions broadcast their tags to the issue queue, they mark their destination register(s) ready, waking their dependents (step 4 in Fig. 2). We do not add additional tag broadcast ports to the ROB over baseline OoO; the number of broadcasts is unchanged, broadcasts are time-shifted until preceding micro-ops resolve. For example, assume that the broadcast bandwidth is four and that two instructions completed this cycle. If another three instructions were marked safe, two of these newly-safe instructions can wake dependents; the third waits for the next cycle. In the majority of our evaluation, we assume broadcast and wake-up of newly- _safe_ instructions fit within the existing wake-up critical path. In Fig. 9e, we include a sensitivity study that shows the impact of further delay due to critical path constraints; a one-cycle delay reduces CPI by less than 3.6%. 

Fig. 6 illustrates an ROB snapshot when executing code akin to Listing 1, depicting various _NDA_ data propagation policies. Column 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
a Strict  b  Permissive  c Load  d   Strict prop.<br>Operation Description propagation propagation restriction + load rest.<br>1 mov   rax,[rbp-0x848] prepare call r x c b r x c b r x c r x c<br>2 mov   rdi,rax prepare call r x c b r x c b<br>3 callq 0x8c2 call victim function r x c b r x c b r x c b r x c b<br><br>4 mov   eax,[rip+0x201732] load array_size r x c b r x c b r x c r x c<br>5 cmp   r12,rax if(x < array_size) r x c b r x c b<br>67 jae lea   r 0x a 9 x 12 ,[r12+rbx*1] if calc addr.  (x <  &a arr rr ay [x _ ] size) rr xx c r r x x c b r x c b r x c<br>8 movzx eax,[rax] Load arr[x] (access phase) r x c b r x c b<br>9 movzx eax,al char s=arr[x](preprocess)<br>10 shl   eax,0x9 s=s*512 (preprocess)<br> Preparing &probe[0]<br>11 movzx edx,[rdx+rax*1] t&=probe[s] (Transmit phase)<br>Resolved branch Unresolved branch <blank> Not  r eady to e x ecute<br>r x c b R eady & e x ecuting  r x c b C ompleted, not  b roadcast  (unsafe) r x c b C ompleted &  b roadcast  (safe)<br><!-- End of picture text -->

**Figure 6: An ROB snapshot during the execution of Spectre v1 (Listing 1), with different** **_NDA_ policies. The branch (call) at line 3 has been resolved, therefore the load in line 4 is** **_safe_ under strict and permissive propagation and can broadcast (wake-up dependants). Under the load restriction policy, the instructions in lines 1,4, and 8 can be executed but are not** **_safe_ until retirement. Therefore, line 2 cannot be issued to execute.** 

__ a shows the ROB snapshot under strict propagation. The branch at Line 6 has not resolved, so all following instructions are marked _unsafe_ . Whereas the instruction at Line 7 executes to completion, it is _unsafe_ and therefore cannot wake the dependent instruction on Line 8. 

Branches resolve when the branch micro-op completes execution. Upon a misprediction, all younger micro-ops in the ROB are squashed and renaming tables are recovered, discarding values in physical registers that never became safe, preventing potentially secret data from leaking. 

### **5.2 Permissive Data Propagation** 

For threat models where _NDA_ only protects secrets in memory or special registers, we can safely optimize performance via _permissive propagation_ (rows 1-2 in Table 2), which marks only _load_ instructions after an unresolved branch/store as _unsafe_ . Arithmetic and control instructions are unconditionally marked _safe_ at dispatch. 

The key intuition for this policy is that only loads can introduce new secret values into the microarchitecture. Loads that precede the eldest unresolved micro-op will commit their value to architectural GPRs, which are not protected under this threat model. Note that wrong-path execution due to exceptions (As in Meltdown or Foreshadow) are also not addressed under this threat model; we address these as chosen-code attacks (5.3). 

For example, consider two dependent instructions _i_ 1 and _i_ 2 fetched after an unresolved branch. If _i_ 1 is an arithmetic instruction (any non-load), it is considered _safe_ . Therefore, _i_ 1 can broadcast its output upon completionallowing _i_ 2 to issuewithout waiting for the branch to resolve. 

This threat model also protects the contents of special registers (e.g., AVX or MSRs, see LazyFP [59] and Meltdown v3a [27]). The instructions to read these registers (e.g., rdmsr) are treated 

like loads and are also marked _unsafe_ when dispatched after an _unresolved_ branch. 

Lines 7-8 in Fig. 6 illustrates the difference between strict (column __ a ) and permissive (column __ b ) propagation. In contrast to strict propagation, the lea instruction on Line 7 is marked _safe_ since it is not a load operation. Therefore, lea wakes its dependent instruction on Line 8 immediately. 

**Bypass Restriction (BR).** To defeat SSB [27] attacks we introduce a new mechanism for safe store bypass, which we use in tandem with both strict and permissive propagation (rows 2,4 in Table 2). In this scheme, unlike Intels SSBD [29], loads are allowed to execute even if they bypass stores in the Load Store Queue (LSQ). However, loads are marked _unsafe_ until all bypassed stores addresses are resolved. If a bypassed store resolves its address in a way that generates an order violation, the offending load and younger instructions are squashed by the memory dependency unit. 

### **5.3 Load Restriction** 

_NDA_ protects against chosen-code attacks by blocking data propagation from speculative loads (row 5 in Table 2), such as in Meltdown [36], Foreshadow [62, 65], LazyFP [59], and MDS attacks [45, 54, 64]. These attacks exploit specific flaws in processor implementations where data propagates from a load that will eventually fault. Each of these flaws has been individually patched [25, 29]. However, given the complexity of modern processor implementations, one might expect similar implementation errors in the future. Moreover, in the chosen-code context, there are a myriad of ways to induce wrong-path execution (faulting loads, Intel TSX transaction aborts, interrupt delivery, breakpoint and syscall instructions, performance counter overflow, load replay due to memory-order misspeculation [20, 74], etc.) As prior work [69] suggests, effective defenses must address the common problems underlying chosen-code attacks. 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

|Mechanism|Control<br>steering<br>(memory)|Control<br>steering<br>(GPRs)|Chosen<br>code|Overhead<br>vs. OoO<br>**Parame**<br>Archite<br>Core (O|
|---|---|---|---|---|
|1 Perm.propagation||||10.7%|
|2 Perm.propagation+BR||||22.3%|
|3 Strictpropagation||||36.1%<br>Core(in<br>|
|4 Strictpropagation+BR||||45%<br>L1-I/L1|
|5 Load restriction||||100%<br>L2 C|
|6 Fullprotection(4+5)||||125%<br>ac<br>DRAM|
|7 InvisiSpec-Spectre*||||7.6%<br>|
|8 InvisiSpec-Future*||||32.7%|
|*<br>Defeats all covert chan<br>Defeats all covert chan<br>Defeats all covert chan<br>Our evaluation of Invis|nels            D<br>nels, but doe<br>nels, except<br>iSpec[69] o|efeats d-c<br>s not bloc<br>single mic<br>n SPEC 20|ache base<br>k SSB<br>ro-op GP<br>17 is deta|d attacks<br>R-attacks<br>iled in 6.1|

|**Parameter**|**Value**|
|---|---|
|Architecture|X86-64 at 2.0 GHz|
|Core (OoO)|8-issue, no SMT, 32 Load Queue entries, 32 Store<br>Queue entries, 192 ROB entries, 4096 BTB entries,<br>16 RAS entries|
|Core(in-order)|TimingSimpleCPU from _gem5_|
|L1-I/L1-D Cache|32kB, 64B line, 8-way set associative (SA), 4 cycle<br>round-trip (RT)latency, 1port|
|L2 Cache|2MB, 64B line, 16-waySA, 40cycle RT latency|
|DRAM|50ns response latency|

**Table 3: Gem5 simulation configuration.** 

<!-- Start of picture text -->
, 60,<br><!-- End of picture text -->

<u>addressed by prior</u> work [48 <mark>,</mark> 60, 69]are defeated. For secrets residing in memory, the output of the access phase ( __ 1 in Fig. 3) cannot be used by the transmit phase __ 2 in the same wrong-path execution window. For an attacker to leak contents from a GPR the transmit phase in a successful attack must comprise only micro- ~~ops that do not~~ depend on one another and that only depend on values from instructions prior to the branch. We note that all existing <u>attacks (cf. Table 1) require multiple dependent micro-ops to transmit secrets.</u> 

**Table 2:** **_<u>NDA</u>_** **<u>propagation policies (rows 1-6) and the attacks</u> they prevent. Bypass Restriction (BR) adds protection against SSB (Spectre v4). Special registers, such as AVX and MSRs (LazyFP** **~~[59] and Spectre v3a [27]), are protected by treat-~~ ing their** **<u>accesses like loads. None of the 25 documented at-</u> tacks [8, 12] leak data from GPRs nor without at least two dependent** **~~micro-ops.~~** 

We therefore propose a blanket _<u>NDA</u>_ <u>protection policy,</u> _<u>load re-</u> striction_ , <u>which both blocks all 11 documented [12, 45, 54, 64]</u> chosen-code attacks and offers the potential to prevent future variants. For instance, _NDA_ s load restriction blocks MDS attacks, which were discovered after our submission. Under load restriction, loads are considered unsafe until they are the eldest unretired instruction (i.e., at the head of the ROB). With load restriction, the microarchitecture guarantees that a load will wake its dependents if and only if it will immediately retire. Column __ c of Fig. 6 illustrates an ROB snapshot when load restriction is used. The loads in Lines 1, 4 are independent and can execute concurrently, enabling high Memory & Instruction Level Parallelism MLP & ILP. However, each will wake its dependents (at Lines 2, 5) only when it retires. 

**Permissive Propagation with Bypass Restriction.** This policy ~~protects secrets~~ in memory but does not protect secrets in GPRs (e.g., rax). This level of protection is on par with the threat model presented in recent work [48, 60] with the added benefit of blocking _all_ covert channels. All 14 documented control-steering attacks [8, 12], including those listed above, are blocked. Any load following an unresolved branch or store is marked unsafe. Therefore, the transmission phase __ 2 will not be able to read the output of the load. However, unlike in strict propagation, non-load micro-ops are marked safe. If the secret already resides in a GPR, the attacker can pre-process and transmit the secret using a sequence of wrongpath operations. 

**Load Restriction.** The _load restriction_ policy addresses all known chosen-code attacks, including Spectre v3, v3a, v4 [27], LazyFP [59], Foreshadow/NG [25, 62, 65], and MDS attacks [45, 54, 64]. In chosen-code threat models, the attacker already controls the executed code, and can thus trivially access the contents of their own GPRs and memory space. Load restriction protects secrets in privileged memory and special registers. Specifically, any micro-op depending on a load (or load-like instruction) will be ready only after the load retires. Upon retirement, the values returned by loads are no longer speculative and are accordingly safe to read. 

### **5.4 Preventing All Classes of Attacks** 

To defeat both control-steering and chosen-code attacks, _NDA_ s final policy composes strict propagation and load restriction (row 6 in Table 2). This _NDA_ policy is the most defensive, so we call it _full protection_ . Column __ d in Fig. 6 illustrates an ROB snapshot when the full-protection policy is used. The loads on Lines 1 and 4 are issued and executed to completion, but are not considered _safe_ . In contrast to the load-restriction case presented in Column __ c , the arithmetic operation on Line 7 is considered _unsafe_ in Column __ d and therefore cannot wake the instruction on Line 8. However, parallel execution is still possible (e.g., lines 4 and 7 still execute in parallel) unlike in an in-order processor. 

Load restriction also has the potential to block future chosen-code attacks that access memory and special registers. Additionally, given that none of the 25 existing speculative execution attacks [8, 12] leak secrets from GPRs, the load restriction policy prevents all known control-steering attacks. 

### **5.5 Security Analysis** 

**Full Protection.** Combining load restriction with the strict propagation policy (row 6 in Table 2) offers the most defensive design point of _NDA_ . The _full-protection_ policy defeats all 25 known control-steering and chosen-code attacks exfiltrating data from memory, special registers, _and_ hinders the attackers ability to transmit contents of GPRs. 

**Strict Propagation with Bypass Restriction.** This policy protects secrets in memory and hinders exfiltration of secrets in GPRs via control-steering attacks. Spectre v1, v1.1, v2, v4 (SSB) [27], and ret2spec [35, 38] are blocked. Most importantly, NetSpectre [55], SMoTher Spectre [8], and our BTB attack (3)which are not 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
OoO<br>Permissive<br>Permissive+BR<br>Strict<br>Strict+BR<br>Restricted Loads<br>Full Protection<br>In-Order<br>InvisiSpec-Spectre<br>InvisiSpec-Future<br>0 1 2 3 4 5 0 1 2 3<br>Cycles per Instruction, normalized to OoO Cycles per Instruction, normalized to OoO<br>perlbench blender<br>gcc cam4<br>bwaves deepsjeng<br>mcf imagick<br>cactuBSSN leela<br>parest povray<br>lbm nab<br>omnetpp exchange2<br>wrf fotonik3d<br>xalancbmk roms<br>x264 xz<br>namd Average<br><!-- End of picture text -->

**Figure 7:** **_NDA_ and InvisiSpec [69] performance on SPEC 2017. Error bars depict the 95% confidence intervals.** 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
200<br>150<br>Cache Secret Byte (42)<br>Cache<br>100<br>BTB Secret Byte (42) BTB<br>50<br>0<br>0 32 64 96 128 160 192 224 256<br>Guess Value<br>Cycles<br><!-- End of picture text -->

byte. For the correct guess of the secret byte, the cache covert channel yields a ~140-cycle decrease due to a cache hit. The BTB covert channel similarly yields a ~16-cycle decrease due to the overhead of mis-prediction, as shown in Figure 5. However, when running the Spectre v1 cache and BTB attacks with permissive propagation enabled, _NDA_ blocks the speculative data leakage _regardless of the covert channel in use_ . As depicted in Figure 8, the correct secret value is indistinguishable from the other 255 candidates. 

### **6.3 NDA Performance** 

**Figure 8: Spectre v1 when using** **_NDA_ permissive propagation policy. The cycle differences in Fig. 4 (Spectre v1** **_without NDA_ ) are eliminated. Thus,** **_NDA_ conceals the secret bytes value, regardless of the covert channel.** 

### **6 EVALUATION** 

We next demonstrate _NDA_ s effectiveness in mitigating speculative execution attacks and evaluate the performance of six different _NDA_ policies. 

### **6.1 Experimental Setup & Methodology** 

We evaluate _NDA_ on _gem5_ [9] running the SPEC CPU 2017 benchmark suite [58]. Table 3 shows our CPU configuration, which reflects a Haswell-like microarchitecture and matches that used in recent architectural studies of speculative execution attacks [69]. To obtain results that represent SPEC benchmark performance with statistical confidence guarantees, we extend _gem5_ to enable a simulation sampling methodology similar to SMARTS [67]. We run SPEC benchmarks on real hardware (Haswell Xeon E5-2699) and dump snapshots of their execution state at fixed intervals using gdb. We have developed a new tool to convert these snapshots to _gem5_ checkpoints and resume their execution in simulation [2, 3]. 

From each checkpoint, we warm simulation state for 5 million instructions and measure performance for 100,000 instructions. We validate that the number of unknown cache references during measurement (references to a cache set for which not all tags are initialized in warmup) is negligible (i.e., the worst-case performance error due to unknown cache references is much smaller than the sampling error). We report 95% confidence intervals of CPI in Fig. 7. 

We compare NDAs performance to both variants of InvisiSpec [69] with the same SMARTS methodology and _gem5_ configuration, using the source code provided by the authors [1]. NDAs and InvisiSpecs performance for the baseline configuration on SPEC 17 are similar within the confidence interval. Absolute performance numbers for InvisiSpec, depicted in Fig. 7, differ from the original paper due to different benchmarks (SPEC 06 vs. SPEC 17) and sampling methodology (a single billion-instruction segment vs. SMARTS sampling). Post-publication, the InvisiSpec authors released a bug fix that affects performance, which we include. 

### **6.2 Effectiveness of NDA** 

We evaluate Spectre v1 [34] (Listing 1 and Listing 3) on unmodified _gem5_ without _NDA_ protections. As illustrated in Figure 4, both the cache and the BTB covert timing channels clearly leak the secret 

We evaluate _NDA_ s performance with ten different configurations; the six _NDA_ policies described in 5, two baselines, and two InvisiSpec configurations. The baseline configurations are the in-order and unconstrained OoO processors listed in Table 3. The in-order processor represents the extreme case of no speculation and is thus trivially immune to speculative execution attacks. We note that, besides _NDA_ s _load-restriction_ and _full-protection_ , the in-order processor is the only other execution model known to defeat all 25 documented speculative execution attacks, regardless of the covert channel they use. The unconstrained OoO processor offers the best performance, but is insecure. 

**Cycles Per Instruction (CPI).** Fig. 7 depicts the CPI of all configurations across all benchmarks, normalized to OoO (averages at the bottom right). The overheads of different policies are summarized in Table 2. Defeating SSB with Bypass Restriction (BR) adds 6.6-9.9 % overhead. In the case of _permissive propagation with BR_ (row 2 in Table 2)our highest performance policy which prevents all 14 control-steering vulnerabilitiesthe average performance loss relative to the OoO baseline is 10.7%. This policy thwarts all known control-steering attacks and recovers 96% of the performance gap between the OoO and In-Order baselines. 

In the case of _full protection_ (row 6 in Table 2)our most secure policythe average performance loss is 125%. This policy prevents all 25 documented variants of both control-steering and chosen-code attacks while also offering potential protection against future attacks. Despite the restrictions it imposes on the dynamic schedule, full protection still closes 68% of the performance gap between in-order and OoO. 

Fig. 9a depicts an average time breakdown for all OoO design variants. The bars are normalized to the baseline OoO design point. _Commit_ cycles are cycles in which at least one instruction retires. _Memory stalls_ are cycles in which the head of the ROB is an incomplete memory operation. _Back-end stalls_ are cycles in which the head of the ROB is a non-memory operation that is not yet ready to retire. _Front-end stalls_ are cycles in which the ROB is empty or cycles which are spent squashing wrong-path execution. _NDA_ policies restrict data propagation and thereby limit dynamic scheduling. Therefore, on average, fewer instructions are committed in a given cycle, increasing the overall number of _commit_ cycles. Since instruction-level parallelism for both memory and non-memory instructions is reduced, more cycles are spent on _memory stalls_ and _back-end stalls_ . _Front-end stall_ cycles generally vary little across designs, on average contributing only 2% of the difference in cycles. 

**Wake-up Latency.** _NDA_ introduces a delay between instruction completion and tag broadcast. Whereas broadcast delay does not _directly_ affect CPI, the delay propagates to dependent instructions 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

**Figure 9: Aggregated statistics over SPEC 17 benchmarks.** **_(a) NDA_ extend the cycles spent on commit and backend stalls.** **_(b),(c)_ MLP & ILP is still high across** **_NDA_ policies.** **_(d)_ As expected,** **_NDA_ causes delays in latency-to-issue. However, overall impact on CPI is substantially smaller.** **_(e)_ The impact of** **_NDA_ logic latency on CPI is relatively small.** 

in the ROB by delaying their issue. We measure this effect by measuring the average delay instructions experience from dispatch to wake-up under each design. The average latencies across all benchmarks are shown in Fig. 9d. _NDA_ policies add on average 4-39 cycles. This increased latency also manifests in up to 78% increase in cycles spent on _back-end stalls_ , shown in Fig. 9a. However, the wake-up latency has a modest impact on overall performance (CPI). 

**Memory and Instruction Parallelism (MLP/ILP).** The favorable performance of _NDA_ compared to the in-order processor can be explained by observing the Memory- and Instruction-Level Parallelism of each profile. The geometric means of MLP & ILP across all benchmarks are depicted in Fig. 9b-c. We follow Chou et al [14] and report MLP as the average outstanding off-chip misses when at least one is outstanding. Whereas the MLP & ILP in the various _NDA_ profiles are at times lower than the OoO baseline by as much as 6% and 44% (respectively), they are better than the in-order baseline processor by 72% and 39%, where MLP & ILP cannot exceed 1.0. These results suggest that _NDA_ enables execution parallelism among off-chip misses despite the scheduling restrictions of speculative instructions. Importantly, _NDA_ does not typically restrict the issue time of loads, only when they may wake dependents. Ergo, typically only dependent loads are delayed, which do not add to MLP or ILP. 

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

- [44] Microsoft 2018. _Protect your Windows devices against Spectre and Meltdown_ . Microsoft. https://support.microsoft.com/en-us/help/4073757/protectyour-windows-devices-against-spectre-meltdown. 

- [45] Marina Minkin, Daniel Moghimi, Moritz Lipp, Michael Schwarz, Jo Van Bulck, Daniel Genkin, Daniel Gruss, Berk Sunar, Frank Piessens, and Yuval Yarom. 2019. Fallout: Reading Kernel Writes From User Space. (2019). 

- [46] Oleksii Oleksenko, Bohdan Trach, Tobias Reiher, Mark Silberstein, and Christof Fetzer. 2018. You Shall Not Bypass: Employing data dependencies to prevent Bounds Check Bypass. _arXiv preprint arXiv:1805.08506_ (2018). 

- [47] Dag Arne Osvik, Adi Shamir, and Eran Tromer. 2006. Cache attacks and countermeasures: the case of AES. In _Cryptographers Track at the RSA Conference_ . Springer, 120. 

- [48] Lutan Zhao Peinan Li and CAS) Rui Hou (Institute of Information Engineering, CAS); Lixin Zhang (HXT Semiconductor Co.LTD); Dan Meng (Institute of Information Engineering. 2019. Conditional Speculation: An Effective Approach to Safeguard Out-of-Order Execution Against Spectre Attacks. In _Proceedings of the 25th IEEE International Symposium on High-Performance Computer Architecture_ . IEEE. 

- [49] Peter Pessl, Leon Groot Bruinderink, and Yuval Yarom. 2017. To BLISS-B or not to be: Attacking strongSwans Implementation of Post-Quantum Signatures. In _CCS_ . ACM, 18431855. 

- [50] Peter Pessl, Daniel Gruss, Clmentine Maurice, Michael Schwarz, and Stefan Mangard. 2016. DRAMA: Exploiting DRAM Addressing for Cross-CPU Attacks.. In _USENIX Security Symposium_ . 565581. 

- [51] Moinuddin K Qureshi. 2019. CEASER: Mitigating Conflict-Based Cache Attacks via Encrypted-Address and Remapping. In _Proceedings of 51th International Symposium on Microarchitecture_ . 

- [52] Ryan Roemer, Erik Buchanan, Hovav Shacham, and Stefan Savage. 2012. Returnoriented programming: Systems, languages, and applications. _ACM Transactions on Information and System Security (TISSEC)_ 15, 1 (2012), 2. 

- [53] Christos Sakalis, Stefanos Kaxiras, Alberto Ros, Alexandra Jimborean, and Magnus Sjlander. 2019. Efficient Invisible Speculative Execution Through Selective Delay and Value Prediction. In _Proceedings of the 46th International Symposium on Computer Architecture_ . ACM, 723735. 

- [54] Michael Schwarz, Moritz Lipp, Daniel Moghimi, Jo Van Bulck, Julian Stecklina, Thomas Prescher, and Daniel Gruss. 2019. ZombieLoad: Cross-PrivilegeBoundary Data Sampling. _arXiv:1905.05726_ (2019). 

- [55] Michael Schwarz, Martin Schwarzl, Moritz Lipp, and Daniel Gruss. 2018. Netspectre: Read arbitrary memory over network. _arXiv preprint arXiv:1807.10535_ (2018). 

- [56] Hovav Shacham. 2007. The geometry of innocent flesh on the bone: Returninto-libc without function calls (on the x86). In _Proceedings of the 14th ACM conference on Computer and communications security_ . ACM, 552561. 

- [57] Zhuojia Shen, Jie Zhou, Divya Ojha, and John Criswell. 2019. Restricting Control Flow During Speculative Execution with Venkman. _arXiv preprint arXiv:1903.10651_ (2019). 

- [58] SPEC. 2017. Standard Performance Evaluation Corporation SPEC CPU 2017. https://www.spec.org/cpu2017/. 

- [59] Julian Stecklina and Thomas Prescher. 2018. LazyFP: Leaking FPU Register State using Microarchitectural Side-Channels. _arXiv preprint arXiv:1806.07480_ (2018). 

- [60] Mohammadkazem Taram, Ashish Venkat, and Dean Tullsen. 2019. ContextSensitive Fencing: Securing Speculative Execution via Microcode Customization. In _Proceedings of the Twenty-Fourth International Conference on Architectural Support for Programming Languages and Operating Systems_ . 

- [61] Ubuntu 2018. _Spectre And Meltdown_ . Ubuntu. https://wiki.ubuntu.com/ SecurityTeam/KnowledgeBase/SpectreAndMeltdown. 

- [62] Jo Van Bulck, Marina Minkin, Ofir Weisse, Daniel Genkin, Baris Kasikci, Frank Piessens, Mark Silberstein, Thomas F Wenisch, Yuval Yarom, and Raoul Strackx. 2018. Foreshadow: Extracting the keys to the Intel SGX kingdom with transient out-of-order execution. In _Proceedings of the 27th USENIX Security Symposium. USENIX Association_ . 

- [63] Jo Van Bulck, Frank Piessens, and Raoul Strackx. 2018. Nemesis: Studying Microarchitectural Timing Leaks in Rudimentary CPU Interrupt Logic. (2018). 

- [64] Stephan van Schaik, Alyssa Milburn, Sebastian Usterlund, Pietro Frigo, Giorgi<sup></sup> Maisuradze, Kaveh Razavi, Herbert Bos, and Cristiano Giuffrida. 2019. RIDL: Rogue In-flight Data Load. In _S&P_ . 

- [65] Ofir Weisse, Jo Van Bulck, Marina Minkin, Daniel Genkin, Baris Kasikci, Frank Piessens, Mark Silberstein, Raoul Strackx, Thomas F. Wenisch, and Yuval Yarom. 2018. Foreshadow-NG: Breaking the Virtual Memory Abstraction with Transient Out-of-Order Execution. _Technical report_ (2018). See also USENIX Security paper Foreshadow [62]. 

- [66] Zhenyu Wu, Zhang Xu, and Haining Wang. 2012. Whispers in the Hyper-space: High-speed Covert Channel Attacks in the Cloud.. In _USENIX Security symposium_ . 159173. 

- [67] Roland E Wunderlich, Thomas F Wenisch, Babak Falsafi, and James C Hoe. 2003. SMARTS: Accelerating microarchitecture simulation via rigorous statistical sampling. In _ACM SIGARCH Computer Architecture News_ , Vol. 31. ACM, 8497. 

- [68] Yunjing Xu, Michael Bailey, Farnam Jahanian, Kaustubh Joshi, Matti Hiltunen, and Richard Schlichting. 2011. An exploration of L2 cache covert channels in virtualized environments. In _Proceedings of the 3rd ACM workshop on Cloud computing security workshop_ . ACM, 2940. 

- [69] Mengjia Yan, Jiho Choi, Dimitrios Skarlatos, Adam Morrison, Christopher W Fletcher, and Josep Torrellas. 2018. InvisiSpec: Making Speculative Execution Invisible in the Cache Hierarchy. In _Proceedings of the 51th International Symposium on Microarchitecture (MICRO18)_ . 

- [70] Mengjia Yan, Bhargava Gopireddy, Thomas Shull, and Josep Torrellas. 2017. Secure hierarchy-aware cache replacement policy (SHARP): Defending against cache-based side channel attacks. In _Computer Architecture (ISCA), 2017 ACM/IEEE 44th Annual International Symposium on_ . IEEE, 347360. 

- [71] Mengjia Yan, Yasser Shalabi, and Josep Torrellas. 2016. ReplayConfusion: detecting cache-based covert channel attacks using record and replay. In _The 49th Annual IEEE/ACM International Symposium on Microarchitecture_ . IEEE Press, 39. 

- [72] Mengjia Yan, Read Sprabery, Bhargava Gopireddy, Christopher Fletcher, Roy Campbell, and Josep Torrellas. 2019. Attack Directories, Not Caches: Side Channel Attacks in a Non-Inclusive World. In _40th IEEE Symposium on Security and Privacy_ . 

- [73] Yuval Yarom and Katrina Falkner. 2014. FLUSH+RELOAD: a High Resolution, Low Noise, L3 Cache Side-Channel Attack. In _USENIX Security_ . 719732. 

- [74] Kenneth C Yeager. 1996. The MIPS R10000 superscalar microprocessor. _IEEE micro_ 16, 2 (1996), 2841. 

- [75] Project Zero. 2018. speculative execution, variant 4: speculative store bypass. https://bugs.chromium.org/p/project-zero/issues/detail?id=1528. 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

NDA: Preventing Speculative Execution Attacks at Their Source 

- [76] Yinqian Zhang, Ari Juels, Michael K. Reiter, and Thomas Ristenpart. 2014. CrossTenant Side-Channel Attacks in PaaS Clouds. In _ACM Conference on Computer_ 

_and Communications Security_ . ACM, 9901003. 

# SOURCE: nda.pdf

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

_NDA_ defeats all 25 documented [8, 12, 45, 54, 64] speculative execution attacks without the need to modify any existing code. Importantly, however, _NDA_ does not preclude all speculation or OoO execution. For example, one _NDA_ policy treats all instructions after an unresolved branch as unsafe. These instructions may still execute speculatively OoO, but they are restricted from propagating their output to dependents until all preceding branches resolve. As our evaluation demonstrates, despite delayed wake-ups, the vast majority of the performance gap between in-order (the only other model known to eliminate all known speculative execution attacks) and unconstrained OoO execution is recovered. 

We simulate _NDA_ designs on the SPEC CPU 2017 benchmark suite and compare its performance to InvisiSpec [69] on the same setup. InvisiSpec blocks data-cache-based attacks and introduces 7.6-32.7% overhead in our setup. In comparison, _NDA_ blocks _all_ covert channels. We show that an _NDA_ policy that mitigates controlsteering vulnerabilities, which are fundamental to unconstrained OoO execution, slows execution by only 10.7% and is 4 _._ 8 __ faster than in-order. If we also preclude Meltdown-like hardware implementation flaws, _NDA_ s strictest policy slows down execution by 125% compared to an insecure OoO processor and is 2 _._ 4 __ faster than in-order execution. 

In short, we make the following contributions: 

- We introduce a speculative-execution-attack taxonomy based on how attacks induce wrong-path execution. 

- We design _NDA_ , a new technique to control speculative data propagation in out-of-order processors to defeat speculative execution attacks. _NDA_ offers multiple variants with differing security/performance tradeoffs. 

- We evaluate six _NDA_ variants on SPEC 2017 and show they are effective and efficient. 

### **2 BACKGROUND** 

**Data Propagation in OoO Processors.** Fig. 2 illustrates conceptual steps in an instructions life-cycle in a modern OoO processor. Upon dispatch into the reorder buffer (ROB), an instruction is not ready to execute until all of its source operandscoming from instructions S1 and S2 in Fig. 2are ready (step 1). Once all source operands are ready, the instruction issues and enters the execution pipeline (step 2). When execution completes (step 3), the instruction wakes its dependents (D1-D5) by broadcasting a tag corresponding to its destination physical register to waiting instructions (step 4), marking those instructions ready. 

The essence of the _NDA_ technique is to _delay tag broadcast_ , i.e., transition from step 3 to step 4. Rather than waking dependent instructions when their input operands become _ready_ , _NDA_ wakes them up when their input operands are _safe_ . We expand on this basic concept in 5. 

**Speculative Execution Attacks** . Speculative execution attacks exploit side-effects of wrong-path execution, which are typically left undefined by processor vendors. While the contents of architectural registers and memory are guaranteed to reflect precise state of only committed instructions, wrong-path execution affects microarchitectural structures. For instance, a wrong-path cache access 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
(1) Source inputs not r eady (2)  Sources      e x ecuting. Not  r eady & instruction     c ompleted yet<br>S1 D1 S1 Ready D1<br>inst. inst.<br>S2 D5 S2 Ready D5<br>r x c b r x c b<br>(3) Instruction   output not  bc rooadcastmpleted,     (4)  C ompleted &  b roadcast<br>S1 Ready D1 S1 Ready D1<br>inst inst<br>S2 Ready D5 S2 Ready D5<br>r x c b r x c b<br>Ready<br>Ready<br>. .<br>. .<br><!-- End of picture text -->

**Figure 2: Life-cycle of instructions in OoO processors. Even after the instruction has completed execution (3), the dependant instructions (D1-D5) will not be able to access the output until it is broadcast (4).** 

may allocate new lines or modify the cache replacement order; these changes are not reverted when wrong-path instructions are squashed. A variety of other micro-architectural structures are also not reverted during squash, for example, branch direction predictors (e.g., pattern history table), pre-decoded micro-op/trace caches, memory dependence predictors, prefetchers, TLBs, fine-grain power management state (e.g., for FPU/AVX units), and performance counters. State changes in these micro-architectural structures can create side channels, where the state can be inferred, for example, based on timing particular execution sequences. We refer to a side channel that is used to intentionally transmit data as a _covert channel_ . Attackers can use _wrong-path_ execution to transmit data, via a covert channel, that is later inferred by _correct-path_ execution and hence leaks that data into architectural state. 

### **3 PROBLEM ANALYSIS** 

We next classify speculative execution attacks based on three fundamental attack phases that exist in all known attacks. We then describe the existing mitigation techniques, how they block the attacks, and their shortcomings. Lastly, to demonstrate that closing specific side channels is insufficient, we show an attack via a new covert channelthe BTB. 

### **3.1 Classifying Attacks** 

**Attack Phases.** All speculative execution attacks of which we are aware comprise three key phases _access, transmit,_ and _recover_  shown in Fig. 3. In the _Access Phase_ ( 1 __ ), secret data is loaded into a temporary register. During the _Transmit Phase_ ( 2 __ ) the secret data is covertly transmitted using micro-architectural side effects that are not reverted when wrong-path instructions are squashed. Finally, in the _Recover Phase_ ( __ 3 ), the transmitted secret is recovered to non-speculative state (e.g., by observing the memory access latency). Whereas the instructions involved in phases __ 1 and __ 2 are speculatively executed and eventually squashed, the phase __ 3 results 

<!-- Start of picture text -->
Access Phase:  Restricted<br>1<br>Speculatively read secret into a  memory/register<br>physical register Load secret  s<br>Physical register<br>Pre-process e.g.,<br>Transmit Phase:  s=(s&0xFF)*512s=&probe[s] 2<br>Speculatively transmit secret via a<br>covert channel. Preprocessing  Physical register<br>may be required Transmit via a<br>covert channel E.g:<br>T = *sT = s<br>Cache FPU Ports BTB TLB PHT ...<br>Probe covert channel<br>Recover Phase:<br>Receive covert transmission<br>Attackers Memory 3<br>non-speculatively<br><!-- End of picture text -->

**Figure 3: Three phases of speculative execution attacks. Prior defenses focus mostly on the cache covert channel, failing to prevent leaks through other channels such as the FPU [55], the BTB (3), and others.** 

are committed to the architectural state. Wrong-path execution is essential to these attacks, as it evades software and hardware protection mechanisms that prevent the secret data from leaking through architectural state. 

**Control-Steering and Chosen-Code Attacks.** We classify attacks based on their methodology for performing the _Access Phase_ ( __ 1 ) and the _Transmit Phase_ ( __ 2 ). We divide attacks based on their _Access Phase_ into two categories, which correspond to different attacker threat models. We further subdivide these two attack classes according to the covert channel exploited in the _Transmit Phase_ . Table 1 illustrates this taxonomy for currently-known attacks. 

In _control-steering_ attacks, the attacker subverts a victim programs control flow to speculatively execute instructions that, as a side-effect, leak data into a covert channel. This attack class leaks data to which the victim application has hardware access privileges, but are intended to be secret and might be protected (e.g., by permission or bounds checks) in software. For example, SGXPectre [13] coerces a secure SGX [42] enclave to access and leak its encrypted memory. We illustrate control-steering attacks in Fig. 1a. 

Unlike a classical security vulnerability, wherein the attacker directly hijacks the program counter (e.g., a stack-smashing attack that overwrites a return address), speculative control-steering attacks only misdirect wrong-path execution, for example, by mis-training branch predictors to direct instruction fetch to an attacker-selected target. Hence, they leave no trace in the committed instruction sequence, but still leak data into a covert channel. Several approaches that use control-steering have been demonstrated [3335, 38]. 

In control-steering attacks, the attacker does not typically introduce new instructions into the victim binary, rather, the attacker composes a series of gadgets from existing code, similar to Return Oriented Programming (ROP [11, 52, 56]). 

By contrast, in _chosen-code_ attacksour second category based on the _Access Phase_ we consider an adversary who can generate 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
1 for (i=0; i < 256; i++) // init channel<br>2 clflush(probeArray[i*512]);<br>3 // Phase  1 - access secret data:<br>4 // The attacker mis-trains the branch:<br>5 if (x < array_size) { // predicted taken<br>6 // wrong-path, x >= array_size<br>7 secret = array[x];<br>8 // Phase  2 - covertly transmit secret:<br>9 t = probeArray[secret * 512];<br>10 }<br>11 // ... somewhere else in attacker's code<br>12 // Phase  3 - recover secret:<br>13 for (guess = 0; guess < 256; guess++) {<br>14 addr = &probeArray[guess*512];<br>15 t1 = rdtscp(); // read timer<br>16 temp = *addr; // access probing array<br>17 t2 = rdtscp(); // read timer<br>18 if (t2-t1 <= CACHE_HIT_THRESHOLD)<br>19 results[guess] += 1;<br>20 }<br><!-- End of picture text -->

**Listing 1: Exfiltrating secret data using Spectre v1** **_controlsteering_ and the cache covert channel.** 

<!-- Start of picture text -->
1 // Phase  1 - access secret:<br>2 secret = *kernel_addr; // Faulting load<br>3 // Phase  2 - covertly transmit secret:<br>4 // Executed in wrong-path<br>5 // before fault is fired:<br>6 t = probeArray[secret * 512];<br>7 // Phase  3 - recover secret:<br>8 // see Listing 1<br><!-- End of picture text -->

**Listing 2: Exfiltrating secret data using the Meltdown** **_chosencode_ attack and a cache side-channel.** 

and execute arbitrary code sequences to mount the attack. Such an adversary already has access to its own registers and memory; these attacks instead seek to circumvent _hardware_ protections that preclude the attacker from accessing secret data in correctpath code. For instance, Meltdown [36] accesses kernel memory; Foreshadow [25, 62, 65] accesses SGX and hypervisor memory; and LazyFP [59] accesses AVX registers used by another process. These attacks exploit implementation flaws in the relative timing of hardware protection checks and data flow between wrong-path instructionsthe secret data propagates among instructions and can be leaked into a covert channel before protection checks squash the wrong-path execution. We show chosen-code attacks in Fig. 1b. 

**Sample Attack Code.** Listing 1 illustrates these phases for the Spectre v1 [34] bounds check bypass attack [27], which is a controlsteering attack. In this attack, the victim code includes instructions that access array at a given index x (Line 7). Before accessing array, the victim code performs a bounds check on x (Line 5). To circumvent the bounds check, the attacker mis-trains the branch direction-predictor by invoking the victim code repeatedly with a valid x. 

To mount the attack, the attacker now calls the victim code with an illegal value of x. The attacker chooses x such that array[x] will refer to a location in the victims memory containing a secret. The direction predictor mis-predicts the branch on Line 5 as taken, executing Lines 79 on the wrong path. During wrong-path execution, the code _accesses_ ( 1 __ ) the secret on Line 7. It then _transmits_ ( 2 __ ) the secret (still in wrong-path) on Line 9. Later, in correct-path execution, the attacker executes Lines 1320 to _recover_ ( 3 __ ) the secret from the cache side-channel. The timing for each access to probeArray on Line 16 will vary based on whether or not the corresponding cache line was loaded on Line 9. In our evaluation, 

<!-- Start of picture text -->
Phase  2<br>Phase  1<br>Attack<br>Spectre v1 [34]<br>Spectre v1.1 [33]<br>Spectre v2 [34,39]<br>Control  Ret2spec [35,38]<br>steering NetSpectre [55]<br>SMoTher Spectre [8]<br>SSB (Spectre v4) [27]<br><future attacks><br>Meltdown (v3 / v3a) [27,36]<br>Chosen LazyFP[59]<br>code Foreshadow (L1TF) [25,62,65]<br>MDS attacks [45,54,64]<br><future attacks><br>- demonstrated in prior work;         - demonstrated in this work<br>- d-cache-based attacks are defeated by prior work [31,48,53,69]<br>d-cache<br>i-cache FPU Ports BTB<br><!-- End of picture text -->

**Table 1: Taxonomy of attacks based on secret data access method** **__ 1 and covert channel** **__ 2 .** **_NDA_ blocks all existing attacks regardless of the covert channel they use. Most common attacks use the d-cache side channel to exfiltrate secret data. All currently known chosen-code attacks use loads and load-like operations. Future attacks may use other instructions or other covert transmission channels.** 

we illustrate the difference in access timing (blue squares in Fig. 4), which reveals the secret data. 

Listing 2 depicts an example of a chosen-code attacka simplified Meltdown exploit. Whereas the illegal load on Line 2 will eventually fault, the instruction on Line 6which executes on the wrong pathwill leave evidence in the cache from which the attacker can recover the secret. The _recover_ phase is identical to that in Listing 1. To avoid trapping into the fault handler, the attacker may use control-steering techniques to ensure the faulting load executes under a mis-predicted branch [36]. Nevertheless, we classify the attack as chosen-code since the attacker controls the executed binary. 

### **3.2 Limitations of Existing Defenses** 

**Current Mitigations.** Hardware defenses mitigating control-steering attacks try to prevent the attacker from mis-training branch predictors (IBRS and STIBP [29]) or use a barrier instruction to prevent speculation after a branch or context switch (lfence/IBPB [29]). Unfortunately, recent attacks [33, 35, 38] reveal techniques to overcome these mitgations. SSBD [6, 29] disables Speculative Store Bypass (SSB, explained in 4.1) to prevent attackers from reading data that was overwritten [27, 75]. However, SSBD only blocks Spectre v4. and introduces up to 8% overhead [26]. 

Software defenses, such as Retpoline [21] and RSB stuffing [28], protect call and ret instructions from mis-steering. Other compiler approaches [22, 46] create a data dependency between a branch condition and code that follows the branch, disabling speculation. However, these compiler approaches can only defeat Spectre v1 [34] attacks. A recent study suggested a compiler modification that also 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
Cache Cache<br>Cache Secret Byte (42) BTB<br>BTB BTB Secret Byte (42)<br><!-- End of picture text -->

**Figure 4: Spectre v1, using either the cache (blue squares) or the BTB (orange circles) as a covert channel. For the cache channel, only the correct guess produces a cache hit, creating the cycle difference**  _Cache_ **. For the BTB channel, only the correct guess successfully predicts the jump target, creating the cycle difference**  _BTB_ **.** 

blocks Spectre v2 attacks [57]. Unfortunately, this approach can only defeat cache-based attacks with 68-247% overhead. 

Chosen-code attacks are mitigated by preventing speculative loads from accessing restricted memory. For instance, Kernel Address Space Layout Randomization (KASLR [23]) and Kernel Page Table Isolation (KPTI [37, 43]) prevent Meltdown attacks from reading privileged kernel memory. KASLR [23] randomizes the kernel address space similar to how ASLR is used to protect user-space processes. KPTI manages separate page tables for the kernel and userspace processes, preventing user code from issuing even illegal loads to kernel memory. KPTI swaps page tables on every transfer between CPU privilege levels. Mitigating Foreshadow [25, 62, 65] requires modifications to the OS, hypervisor, and SMM code, such as modifying page-table management, altering virtual machine scheduling, and adding L1 cache flushes when switching security domains [25, 65]. 

Unfortunately, all these defense mechanisms block only specific exploit techniques. Therefore, one must deploy a myriad software and hardware defenses to be resilient against _all_ control-steering and chosen-code attacks. 

Recent work suggests preventing both control-steering and chosencode attacks by blocking the cache side channel [31, 48, 53, 69], thus interdicting the _transmit_ phase. However, given the abundant supply of covert channels (see Fig. 3), defeating speculative attacks by closing each channel individually is challenging. Exploits have already been demonstrated for other channels. Netspectre [55] demonstrated that the power state of the FPU is a viable speculative covert channel. SMoTher Spectre [8] showed how to transmit data via port contention [4]. We next show an attack via the BTB. 

**The BTB Covert Channel.** We demonstrate a new covert channel that can be exploited even when the cache covert channel is not availablethe BTB. The BTB stores a mapping between branch instructions addresses and the associated target addresses. For example, a call instruction located at address A to a function located at address B installs the mapping A => B in the BTB. The next time the processor fetches the call instruction at address A, the processors front-end will speculatively redirect fetch to address B. 

If the BTB predicts correctly (Fig. 5a), the speculatively-fetched instructions are eventually retired. However, if the prediction is 

<!-- Start of picture text -->
a) Correct BTB prediction<br>jumpToTarget<br>1 predict<br>correctTarget<br>b) Incorrect BTB prediction<br>Overhead of mis-prediction:<br>Wrong-path<br>1 predict<br>wrongTarget<br>jumpToTarget 2 squash<br>3<br>correctTarget<br><!-- End of picture text -->

**Figure 5: The BTB covert channel. The attacker can observe if the BTB prediction was correct by measuring execution time.** 

wrong, the processor will squash the wrong-path execution, starting at the mispredicted instruction at address B, before executing the correct path. This recovery process is illustrated in Fig. 5b. In our experiments on the _gem5_ [9] simulator, we observe that it takes ~ 16 cycles for the BTB miss to resolve, wrong-path execution to be squashed, and execution to resume at the correct target (1 + 2 in Fig. 5b). Crucially, updates to the BTB during speculation are not reverted by the squash, making it an effective covert channel. Note that (as with caches) in the absence of security concerns, filling the BTB (and updating its replacement policy) during speculation may be advantageous to avoid future BTB misses. 

To demonstrate the BTB covert channel, we construct a variant of Spectre v1 [34] that leaks a secret byte through a speculative BTB update, as illustrated in Listing 3. To leak a single byte, our covert channel comprises 256 distinct functions (targets in Line 2). During both the _Transmit Phase_ and _Recover Phase_ , we invoke targets only from a single call site, jumpToTarget (Line 6), ensuring that BTB entries mapping to targets all originate from the same PC and therefore conflict in the BTB. 

When the branch on Line 10 is mispredicted, the attacker can _access_ any value from the process address space, depending on the value of x. The attacker then _transmits_ the secret by speculatively calling jumpToTarget with the secret value in Line 13. If the speculation window is large enough, the processor updates the BTB entry for the call instruction in Line 6 based on secret. 

The _access_ phase must be repeated for every guess (Line 19) since the _recover_ phase is destructive: The execution of Line 21 alters the contents of the BTB to point to targets[guess]. To confirm that the BTB acts as the covert channel in our attack, it is important to validate that execution time differences do _not_ arise from i-cache or d-cache hit or miss latency; no change to the cache hierarchy during the attack may depend upon the secret value. To validate our attack, we ensure the targets array in Line 2 and all 256 target functions are cached during access, transmission, and recovery. 

We report the effectiveness of the BTB covert channel on _gem5_ via the orange circles in Fig. 4. During the _Recover Phase_ , in lines 17-24, all the _wrong_ guesses will incur the 16-cycle prediction and squashing delay, as depicted in Fig. 5b. The _correct_ guess will execute faster, as depicted in Fig. 5a. 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
1 // array of 256 unique target functions<br>2 void (*targets[256])( void );<br>3 // all jumps are from the same location,<br>4 // hence the same BTB entry is consulted<br>5 void jumpToTarget( int index)<br>6 { targets[index](); }<br>7 void victim_function(x) {<br>8 // Phase  1 - access secret data:<br>9 // The attacker mis-trains the branch:<br>10 if (x < array_size) { // predicted taken<br>11 secret = array[x]; // wrong path<br>12 // Phase  2 - covertly transmit secret:<br>13 jumpToTarget(secret); // updates BTB<br>14 } }<br>15 // ... somewhere else in attacker's code<br>16 // Phase  3 - recover secret:<br>17 for (guess = 0; guess < 256; guess++) {<br>18 // Induce victim to leak secret value<br>19 victim_function(x);<br>20 t1 = rdtscp(); // read timer<br>21 jumpToTarget(guess); // BTB prediction<br>22 t2 = rdtscp(); // read timer<br>23 if (t2-t1 <= CORRECT_PATH_THRESHOLD)<br>24 results[guess] += 1;<br>25 }<br><!-- End of picture text -->

**Listing 3: Exfiltrating secret data using the Spectre v1** **_controlsteering_ attack and the BTB side-channel.** 

The BTB covert channel is one of many potential machine-specific transmission channels. We use our BTB channel PoC to demonstrate that _NDA_ is agnostic to any specific transmission channel (6). 

### **4 THREAT MODELS** 

_NDA_ design variants address four different threat models, which vary in the locations from which secret data are stolen and whether the attacker may mount control-steering or chosen-code attacks. _NDA_ s goal is to eliminate side-channels created in _wrong-path_ execution. Correct-path side channels have been studied in prior work [51, 70, 71]. 

All threat models are agnostic to the covert channel used in the attacks. For control-steering attacks, we consider two threat models, based on where secrets reside. The first model considers attacks against secrets stored in memory or special registers, as is the case for all currently-known control-steering attacks. Our second controlsteering threat model additionally considers hypothetical attacks that leak secrets residing in general-purpose registers (GPRs). In our third threat model, for chosen-code attacks, we consider only threats against secrets in privileged memory and registers, since chosencode attacks presuppose attacker-controlled GPRs. Lastly, our fourth threat model comprises the union of these threats, considering both control-steering and chosen-code attacks for secrets in memory, special-registers, and GPRs. 

### **4.1 Leaking Memory via Control-Steering** 

The first step of all known control-steering attacks is to steer wrongpath execution into code accessing a secret in memory or manipulate execution timing to cause a load to observe a stale value. We assume the attacker can steer execution at any branch instruction and manipulate the execution timing of all instructions. Branch instructions include all variants of jmp, call, and ret. 

We do not consider _phantom branches_ , where the BTB is mistrained to steer control flow from a program counter value that does not correspond to a branch. The dispatch stage stalls micro-ops whose opcode is unknown. Hence, if the BTB predicts a branch where there is none, dispatch will stall at the phantom branch until 

its opcode is obtained, which will resolve the misprediction and cause any younger fetched instructions to be discarded before they enter the OoO back-end. Wrong-path instructions that are squashed before dispatch are not a threat. 

We also do not consider potentially faulting instructions as steering points in control-steering attacks. Whereas a fault can result in wrong-path execution, we consider attacks based on faulting instructions (e.g., Meltdown, Foreshadow, LazyFP, MDS, etc.) as part of the threat model for chosen-code attacks. 

**Speculative Store Bypass.** Also known as SSB, or Spectre variant 4 [27], this attack performs the _Access Phase_ ( __ 1 in Fig. 3) by having a malicious speculative load bypass a store whose address is still unresolved. The malicious load then speculatively yields stale (secret) data. Although this attack may not necessarily require misdirected control flow in the _Access Phase_ , we consider it a special case of control-steering, since the attacker leverages an existing code snippet. If the attacker could choose the code, they could read the stale data without the need to exploit the speculative store-bypass. 

### **4.2 Leaking GPRs via Control-Steering** 

All currently-known control-steering attacks extract secrets residing in memory. Nevertheless, we recognize that future attacks might extract secrets residing in the victims GPRs. So, our second threat model considers the attacker of 4.1 that steers the victims control flow to leak GPR contents. 

In this scenario, the steered victims code already possesses the secret in a GPR. At this point, the access phase of the control-steering attack ( 1 __ in Fig. 3) has already (possibly unintentionally) been done by the victim. We therefore focus on hindering the attacker from performing the second phase ( __ 2 in Fig. 3)transmitting the GPR-resident secret. All known attacks require data flow between micro-ops during the transmit phase to preprocess the secret (e.g., calculate an offset relative to a base address) before it can be leaked. 

We do not prevent an attack that leaks a secret using only a single speculative micro-op. In principle, it may be possible to covertly transmit GPR-based secrets using a single micro-op. For instance, if a GPR contains a secret value that corresponds to a valid virtual memory address, the attacker can speculatively issue a load that will fetch this address into the cache hierarchy, thus performing the transmit phase in a single micro-op. However, such a scenario would require (a) a secret value that forms a valid memory address, and (b) victim code that voluntarily loads the secret into a GPR shortly before the vulnerable steering point. No known attacks (cf. Table 1) exploit this behavior. 

### **4.3 Leaking Memory with Chosen-Code** 

For chosen-code attacks, we consider attackers that attempt to access secrets residing in memory. Specifically, we consider an attacker who can influence code generation to control both correct-path and wrong-path execution. We treat read operations from specialpurpose registers, such as AVX (as abused in LazyFP [59]) and Model Specific Registers (MSRs, in Meltdown variant 3a [27]) like memory accesses in crafting our defensethe special instructions (e.g., rdmsr) used to access these registers are treated like loads in our solution. In chosen-code attacks, the attacker already controls 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

their own GPRs and we therefore do not consider the contents of any GPR to be secret. 

Instructions are guaranteed to be correct-path when they retire. At retirement, the head of the ROB satisfies _hardware_ permission and memory-ordering checks. Ergo, retired instructions cannot leak secrets accessed from the _wrong-path_ . 

### **4.4 Combining the Threat Models** 

Finally, we consider _NDA_ s most conservative threat modela combination of all threats outlined above. We suppose an attacker that conducts both (a) control-steering attacks to extract secrets from the victims memory and GPRs, _and_ (b) chosen-code attacks to access privileged memory and special registers. This combined threat model is similar to the practical approach taken by Windows and Linux, which deploy mitigations for both classes of attacks [29, 37, 43, 44, 61]. 

### **5 DESIGN** 

_NDA_ s main design goal is to mitigate both control-steering and chosen-code attacks while reaping the benefits of OoO speculative execution as much as possible. We next discuss different variants of _NDA_ , which provide different policies for speculative data propagation depending on the threat model. Different _NDA_ data propagation policies offer different security guarantees and have corresponding performance implications. We build _NDA_ upon a baseline physical register-based OoO micro-architecture [74]. 

The key insight behind _NDA_ s design is that speculative instructions (either in the _correct_ or the _wrong_ -path) can safely execute without leaking secrets as long as their inputs are results of _safe_ instructions. We define instructions as _safe_ with respect to our threat models such that wrong-path execution can not leak any more information into a side channel than a correct-path instruction. Consequently, we eliminate the gap between _speculative_ side channel attacks and _non-speculative_ side channels, which security-conscious programmers already must reason about. The different _NDA_ policies, listed in rows 1-6 of Table 2, define which instructions are considered _safe_ such that they may wake-up dependent instructions (allow instructions to advance from step 3 to step 4 in Fig. 2). 

To mitigate control-steering attacks, _NDA_ restricts data propagation following an unresolved branch or unresolved store address (rows 1-4 in Table 2), depending on where secrets reside and if store-bypass (SSB) is a threat. We consider any instruction following a predicted branch as _unsafe_ until the branch target and direction are resolved. We also consider loads that follow a store with an unresolved address as _unsafe_ (see Bypass Restriction in 5.2). To mitigate chosen-code attacks, _NDA_ introduces a _propagate-on-retire_ mechanism (row 5), which defeats all 11 documented chosen-code attack variants [12, 45, 54, 64] and similar future exploits that rely on speculative loads. In this policy, the value returned by _any_ load instruction (or other instructions that read sensitive registers, such as rdmsr on x86) are considered _unsafe_ until the load is ready to retire. Finally, the two mechanisms can be combined to defend against both classes of attacks (row 6). 

### **5.1 Strict Data Propagation** 

_NDA_ addresses control-steering attacks by defining unresolved branches and unresolved storesfor which predictions may be incorrectas the borders between safe and unsafe speculation. When a branch micro-op enters the ROB, it is _unresolved_ . Since the fetch unit predicts which instructions to fetch following the branch (via the BTB, RSB, etc.), subsequently dispatched micro-ops may be wrong-path. Similarly, when a store micro-op enters the ROB, it is _unresolved_ until its address is calculated. If a stores address has not been calculated, loads that follow the store may erroneously access stale data if their addresses overlap. We consider two variants of data propagation restrictions with regards to control-steering attacks: strict and permissive. Both variants leverage a _Bypass Restriction_ mechanism to defeat SSB attacks. We now describe strict propagation and then explain permissive propagation and bypass restriction in 5.2. 

_Strict Propagation_ (rows 3-4 in Table 2) defends against threat models where secrets may reside in memory, special registers, and GPRs (i.e., the union of the threats described in 4.1 and 4.2). Under this policy, _NDA_ marks _all_ micro-ops dispatched after an unresolved branch or store as _unsafe_ . Unsafe instructions may wake up and compete to issue as in a baseline OoO (i.e., they may issue when their operands become ready). But, when an unsafe micro-op completes execution (step 3 in Fig. 2), it writes back to its destination physical register, but does not broadcast its destination tag to dependent instructions, does not mark its destination register ready, and does not forward its output value on the bypass network. Hence, dependent instructions will not issue and cannot observe the unsafe value. 

**Managing Value Propagation.** When the eldest outstanding micro-op resolves, it marks instructions in the ROB _safe_ until the next eldest unresolved branch/store. ROB entries are extended with three bits: unsafe tracks if the instruction follows a still-unresolved micro-op, exec tracks if the instruction has executed, and bcast tracks if the instruction has broadcast its tag to wake dependents. Upon instruction completion, if unsafe, tag broadcast is deferred. When a micro-op resolves, the unsafe bit for subsequent ROB entries until the next unresolved branch/store are cleared. !unsafe && exec && !bcast instructions arbitrate for tag broadcast ports, competing with instructions completing in the current cycle (completing instructions have priority to avoid pipeline stalls); bcast is set when broadcasting. 

When _safe_ instructions broadcast their tags to the issue queue, they mark their destination register(s) ready, waking their dependents (step 4 in Fig. 2). We do not add additional tag broadcast ports to the ROB over baseline OoO; the number of broadcasts is unchanged, broadcasts are time-shifted until preceding micro-ops resolve. For example, assume that the broadcast bandwidth is four and that two instructions completed this cycle. If another three instructions were marked safe, two of these newly-safe instructions can wake dependents; the third waits for the next cycle. In the majority of our evaluation, we assume broadcast and wake-up of newly- _safe_ instructions fit within the existing wake-up critical path. In Fig. 9e, we include a sensitivity study that shows the impact of further delay due to critical path constraints; a one-cycle delay reduces CPI by less than 3.6%. 

Fig. 6 illustrates an ROB snapshot when executing code akin to Listing 1, depicting various _NDA_ data propagation policies. Column 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
a Strict  b  Permissive  c Load  d   Strict prop.<br>Operation Description propagation propagation restriction + load rest.<br>1 mov   rax,[rbp-0x848] prepare call r x c b r x c b r x c r x c<br>2 mov   rdi,rax prepare call r x c b r x c b<br>3 callq 0x8c2 call victim function r x c b r x c b r x c b r x c b<br><br>4 mov   eax,[rip+0x201732] load array_size r x c b r x c b r x c r x c<br>5 cmp   r12,rax if(x < array_size) r x c b r x c b<br>67 jae lea   r 0x a 9 x 12 ,[r12+rbx*1] if calc addr.  (x <  &a arr rr ay [x _ ] size) rr xx c r r x x c b r x c b r x c<br>8 movzx eax,[rax] Load arr[x] (access phase) r x c b r x c b<br>9 movzx eax,al char s=arr[x](preprocess)<br>10 shl   eax,0x9 s=s*512 (preprocess)<br> Preparing &probe[0]<br>11 movzx edx,[rdx+rax*1] t&=probe[s] (Transmit phase)<br>Resolved branch Unresolved branch <blank> Not  r eady to e x ecute<br>r x c b R eady & e x ecuting  r x c b C ompleted, not  b roadcast  (unsafe) r x c b C ompleted &  b roadcast  (safe)<br><!-- End of picture text -->

**Figure 6: An ROB snapshot during the execution of Spectre v1 (Listing 1), with different** **_NDA_ policies. The branch (call) at line 3 has been resolved, therefore the load in line 4 is** **_safe_ under strict and permissive propagation and can broadcast (wake-up dependants). Under the load restriction policy, the instructions in lines 1,4, and 8 can be executed but are not** **_safe_ until retirement. Therefore, line 2 cannot be issued to execute.** 

__ a shows the ROB snapshot under strict propagation. The branch at Line 6 has not resolved, so all following instructions are marked _unsafe_ . Whereas the instruction at Line 7 executes to completion, it is _unsafe_ and therefore cannot wake the dependent instruction on Line 8. 

Branches resolve when the branch micro-op completes execution. Upon a misprediction, all younger micro-ops in the ROB are squashed and renaming tables are recovered, discarding values in physical registers that never became safe, preventing potentially secret data from leaking. 

### **5.2 Permissive Data Propagation** 

For threat models where _NDA_ only protects secrets in memory or special registers, we can safely optimize performance via _permissive propagation_ (rows 1-2 in Table 2), which marks only _load_ instructions after an unresolved branch/store as _unsafe_ . Arithmetic and control instructions are unconditionally marked _safe_ at dispatch. 

The key intuition for this policy is that only loads can introduce new secret values into the microarchitecture. Loads that precede the eldest unresolved micro-op will commit their value to architectural GPRs, which are not protected under this threat model. Note that wrong-path execution due to exceptions (As in Meltdown or Foreshadow) are also not addressed under this threat model; we address these as chosen-code attacks (5.3). 

For example, consider two dependent instructions _i_ 1 and _i_ 2 fetched after an unresolved branch. If _i_ 1 is an arithmetic instruction (any non-load), it is considered _safe_ . Therefore, _i_ 1 can broadcast its output upon completionallowing _i_ 2 to issuewithout waiting for the branch to resolve. 

This threat model also protects the contents of special registers (e.g., AVX or MSRs, see LazyFP [59] and Meltdown v3a [27]). The instructions to read these registers (e.g., rdmsr) are treated 

like loads and are also marked _unsafe_ when dispatched after an _unresolved_ branch. 

Lines 7-8 in Fig. 6 illustrates the difference between strict (column __ a ) and permissive (column __ b ) propagation. In contrast to strict propagation, the lea instruction on Line 7 is marked _safe_ since it is not a load operation. Therefore, lea wakes its dependent instruction on Line 8 immediately. 

**Bypass Restriction (BR).** To defeat SSB [27] attacks we introduce a new mechanism for safe store bypass, which we use in tandem with both strict and permissive propagation (rows 2,4 in Table 2). In this scheme, unlike Intels SSBD [29], loads are allowed to execute even if they bypass stores in the Load Store Queue (LSQ). However, loads are marked _unsafe_ until all bypassed stores addresses are resolved. If a bypassed store resolves its address in a way that generates an order violation, the offending load and younger instructions are squashed by the memory dependency unit. 

### **5.3 Load Restriction** 

_NDA_ protects against chosen-code attacks by blocking data propagation from speculative loads (row 5 in Table 2), such as in Meltdown [36], Foreshadow [62, 65], LazyFP [59], and MDS attacks [45, 54, 64]. These attacks exploit specific flaws in processor implementations where data propagates from a load that will eventually fault. Each of these flaws has been individually patched [25, 29]. However, given the complexity of modern processor implementations, one might expect similar implementation errors in the future. Moreover, in the chosen-code context, there are a myriad of ways to induce wrong-path execution (faulting loads, Intel TSX transaction aborts, interrupt delivery, breakpoint and syscall instructions, performance counter overflow, load replay due to memory-order misspeculation [20, 74], etc.) As prior work [69] suggests, effective defenses must address the common problems underlying chosen-code attacks. 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

|Mechanism|Control<br>steering<br>(memory)|Control<br>steering<br>(GPRs)|Chosen<br>code|Overhead<br>vs. OoO<br>**Parame**<br>Archite<br>Core (O|
|---|---|---|---|---|
|1 Perm.propagation||||10.7%|
|2 Perm.propagation+BR||||22.3%|
|3 Strictpropagation||||36.1%<br>Core(in<br>|
|4 Strictpropagation+BR||||45%<br>L1-I/L1|
|5 Load restriction||||100%<br>L2 C|
|6 Fullprotection(4+5)||||125%<br>ac<br>DRAM|
|7 InvisiSpec-Spectre*||||7.6%<br>|
|8 InvisiSpec-Future*||||32.7%|
|*<br>Defeats all covert chan<br>Defeats all covert chan<br>Defeats all covert chan<br>Our evaluation of Invis|nels            D<br>nels, but doe<br>nels, except<br>iSpec[69] o|efeats d-c<br>s not bloc<br>single mic<br>n SPEC 20|ache base<br>k SSB<br>ro-op GP<br>17 is deta|d attacks<br>R-attacks<br>iled in 6.1|

|**Parameter**|**Value**|
|---|---|
|Architecture|X86-64 at 2.0 GHz|
|Core (OoO)|8-issue, no SMT, 32 Load Queue entries, 32 Store<br>Queue entries, 192 ROB entries, 4096 BTB entries,<br>16 RAS entries|
|Core(in-order)|TimingSimpleCPU from _gem5_|
|L1-I/L1-D Cache|32kB, 64B line, 8-way set associative (SA), 4 cycle<br>round-trip (RT)latency, 1port|
|L2 Cache|2MB, 64B line, 16-waySA, 40cycle RT latency|
|DRAM|50ns response latency|

**Table 3: Gem5 simulation configuration.** 

<!-- Start of picture text -->
, 60,<br><!-- End of picture text -->

<u>addressed by prior</u> work [48 <mark>,</mark> 60, 69]are defeated. For secrets residing in memory, the output of the access phase ( __ 1 in Fig. 3) cannot be used by the transmit phase __ 2 in the same wrong-path execution window. For an attacker to leak contents from a GPR the transmit phase in a successful attack must comprise only micro- ~~ops that do not~~ depend on one another and that only depend on values from instructions prior to the branch. We note that all existing <u>attacks (cf. Table 1) require multiple dependent micro-ops to transmit secrets.</u> 

**Table 2:** **_<u>NDA</u>_** **<u>propagation policies (rows 1-6) and the attacks</u> they prevent. Bypass Restriction (BR) adds protection against SSB (Spectre v4). Special registers, such as AVX and MSRs (LazyFP** **~~[59] and Spectre v3a [27]), are protected by treat-~~ ing their** **<u>accesses like loads. None of the 25 documented at-</u> tacks [8, 12] leak data from GPRs nor without at least two dependent** **~~micro-ops.~~** 

We therefore propose a blanket _<u>NDA</u>_ <u>protection policy,</u> _<u>load re-</u> striction_ , <u>which both blocks all 11 documented [12, 45, 54, 64]</u> chosen-code attacks and offers the potential to prevent future variants. For instance, _NDA_ s load restriction blocks MDS attacks, which were discovered after our submission. Under load restriction, loads are considered unsafe until they are the eldest unretired instruction (i.e., at the head of the ROB). With load restriction, the microarchitecture guarantees that a load will wake its dependents if and only if it will immediately retire. Column __ c of Fig. 6 illustrates an ROB snapshot when load restriction is used. The loads in Lines 1, 4 are independent and can execute concurrently, enabling high Memory & Instruction Level Parallelism MLP & ILP. However, each will wake its dependents (at Lines 2, 5) only when it retires. 

**Permissive Propagation with Bypass Restriction.** This policy ~~protects secrets~~ in memory but does not protect secrets in GPRs (e.g., rax). This level of protection is on par with the threat model presented in recent work [48, 60] with the added benefit of blocking _all_ covert channels. All 14 documented control-steering attacks [8, 12], including those listed above, are blocked. Any load following an unresolved branch or store is marked unsafe. Therefore, the transmission phase __ 2 will not be able to read the output of the load. However, unlike in strict propagation, non-load micro-ops are marked safe. If the secret already resides in a GPR, the attacker can pre-process and transmit the secret using a sequence of wrongpath operations. 

**Load Restriction.** The _load restriction_ policy addresses all known chosen-code attacks, including Spectre v3, v3a, v4 [27], LazyFP [59], Foreshadow/NG [25, 62, 65], and MDS attacks [45, 54, 64]. In chosen-code threat models, the attacker already controls the executed code, and can thus trivially access the contents of their own GPRs and memory space. Load restriction protects secrets in privileged memory and special registers. Specifically, any micro-op depending on a load (or load-like instruction) will be ready only after the load retires. Upon retirement, the values returned by loads are no longer speculative and are accordingly safe to read. 

### **5.4 Preventing All Classes of Attacks** 

To defeat both control-steering and chosen-code attacks, _NDA_ s final policy composes strict propagation and load restriction (row 6 in Table 2). This _NDA_ policy is the most defensive, so we call it _full protection_ . Column __ d in Fig. 6 illustrates an ROB snapshot when the full-protection policy is used. The loads on Lines 1 and 4 are issued and executed to completion, but are not considered _safe_ . In contrast to the load-restriction case presented in Column __ c , the arithmetic operation on Line 7 is considered _unsafe_ in Column __ d and therefore cannot wake the instruction on Line 8. However, parallel execution is still possible (e.g., lines 4 and 7 still execute in parallel) unlike in an in-order processor. 

Load restriction also has the potential to block future chosen-code attacks that access memory and special registers. Additionally, given that none of the 25 existing speculative execution attacks [8, 12] leak secrets from GPRs, the load restriction policy prevents all known control-steering attacks. 

### **5.5 Security Analysis** 

**Full Protection.** Combining load restriction with the strict propagation policy (row 6 in Table 2) offers the most defensive design point of _NDA_ . The _full-protection_ policy defeats all 25 known control-steering and chosen-code attacks exfiltrating data from memory, special registers, _and_ hinders the attackers ability to transmit contents of GPRs. 

**Strict Propagation with Bypass Restriction.** This policy protects secrets in memory and hinders exfiltration of secrets in GPRs via control-steering attacks. Spectre v1, v1.1, v2, v4 (SSB) [27], and ret2spec [35, 38] are blocked. Most importantly, NetSpectre [55], SMoTher Spectre [8], and our BTB attack (3)which are not 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

Weisse, et al. 

<!-- Start of picture text -->
OoO<br>Permissive<br>Permissive+BR<br>Strict<br>Strict+BR<br>Restricted Loads<br>Full Protection<br>In-Order<br>InvisiSpec-Spectre<br>InvisiSpec-Future<br>0 1 2 3 4 5 0 1 2 3<br>Cycles per Instruction, normalized to OoO Cycles per Instruction, normalized to OoO<br>perlbench blender<br>gcc cam4<br>bwaves deepsjeng<br>mcf imagick<br>cactuBSSN leela<br>parest povray<br>lbm nab<br>omnetpp exchange2<br>wrf fotonik3d<br>xalancbmk roms<br>x264 xz<br>namd Average<br><!-- End of picture text -->

**Figure 7:** **_NDA_ and InvisiSpec [69] performance on SPEC 2017. Error bars depict the 95% confidence intervals.** 

NDA: Preventing Speculative Execution Attacks at Their Source 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

<!-- Start of picture text -->
200<br>150<br>Cache Secret Byte (42)<br>Cache<br>100<br>BTB Secret Byte (42) BTB<br>50<br>0<br>0 32 64 96 128 160 192 224 256<br>Guess Value<br>Cycles<br><!-- End of picture text -->

byte. For the correct guess of the secret byte, the cache covert channel yields a ~140-cycle decrease due to a cache hit. The BTB covert channel similarly yields a ~16-cycle decrease due to the overhead of mis-prediction, as shown in Figure 5. However, when running the Spectre v1 cache and BTB attacks with permissive propagation enabled, _NDA_ blocks the speculative data leakage _regardless of the covert channel in use_ . As depicted in Figure 8, the correct secret value is indistinguishable from the other 255 candidates. 

### **6.3 NDA Performance** 

**Figure 8: Spectre v1 when using** **_NDA_ permissive propagation policy. The cycle differences in Fig. 4 (Spectre v1** **_without NDA_ ) are eliminated. Thus,** **_NDA_ conceals the secret bytes value, regardless of the covert channel.** 

### **6 EVALUATION** 

We next demonstrate _NDA_ s effectiveness in mitigating speculative execution attacks and evaluate the performance of six different _NDA_ policies. 

### **6.1 Experimental Setup & Methodology** 

We evaluate _NDA_ on _gem5_ [9] running the SPEC CPU 2017 benchmark suite [58]. Table 3 shows our CPU configuration, which reflects a Haswell-like microarchitecture and matches that used in recent architectural studies of speculative execution attacks [69]. To obtain results that represent SPEC benchmark performance with statistical confidence guarantees, we extend _gem5_ to enable a simulation sampling methodology similar to SMARTS [67]. We run SPEC benchmarks on real hardware (Haswell Xeon E5-2699) and dump snapshots of their execution state at fixed intervals using gdb. We have developed a new tool to convert these snapshots to _gem5_ checkpoints and resume their execution in simulation [2, 3]. 

From each checkpoint, we warm simulation state for 5 million instructions and measure performance for 100,000 instructions. We validate that the number of unknown cache references during measurement (references to a cache set for which not all tags are initialized in warmup) is negligible (i.e., the worst-case performance error due to unknown cache references is much smaller than the sampling error). We report 95% confidence intervals of CPI in Fig. 7. 

We compare NDAs performance to both variants of InvisiSpec [69] with the same SMARTS methodology and _gem5_ configuration, using the source code provided by the authors [1]. NDAs and InvisiSpecs performance for the baseline configuration on SPEC 17 are similar within the confidence interval. Absolute performance numbers for InvisiSpec, depicted in Fig. 7, differ from the original paper due to different benchmarks (SPEC 06 vs. SPEC 17) and sampling methodology (a single billion-instruction segment vs. SMARTS sampling). Post-publication, the InvisiSpec authors released a bug fix that affects performance, which we include. 

### **6.2 Effectiveness of NDA** 

We evaluate Spectre v1 [34] (Listing 1 and Listing 3) on unmodified _gem5_ without _NDA_ protections. As illustrated in Figure 4, both the cache and the BTB covert timing channels clearly leak the secret 

We evaluate _NDA_ s performance with ten different configurations; the six _NDA_ policies described in 5, two baselines, and two InvisiSpec configurations. The baseline configurations are the in-order and unconstrained OoO processors listed in Table 3. The in-order processor represents the extreme case of no speculation and is thus trivially immune to speculative execution attacks. We note that, besides _NDA_ s _load-restriction_ and _full-protection_ , the in-order processor is the only other execution model known to defeat all 25 documented speculative execution attacks, regardless of the covert channel they use. The unconstrained OoO processor offers the best performance, but is insecure. 

**Cycles Per Instruction (CPI).** Fig. 7 depicts the CPI of all configurations across all benchmarks, normalized to OoO (averages at the bottom right). The overheads of different policies are summarized in Table 2. Defeating SSB with Bypass Restriction (BR) adds 6.6-9.9 % overhead. In the case of _permissive propagation with BR_ (row 2 in Table 2)our highest performance policy which prevents all 14 control-steering vulnerabilitiesthe average performance loss relative to the OoO baseline is 10.7%. This policy thwarts all known control-steering attacks and recovers 96% of the performance gap between the OoO and In-Order baselines. 

In the case of _full protection_ (row 6 in Table 2)our most secure policythe average performance loss is 125%. This policy prevents all 25 documented variants of both control-steering and chosen-code attacks while also offering potential protection against future attacks. Despite the restrictions it imposes on the dynamic schedule, full protection still closes 68% of the performance gap between in-order and OoO. 

Fig. 9a depicts an average time breakdown for all OoO design variants. The bars are normalized to the baseline OoO design point. _Commit_ cycles are cycles in which at least one instruction retires. _Memory stalls_ are cycles in which the head of the ROB is an incomplete memory operation. _Back-end stalls_ are cycles in which the head of the ROB is a non-memory operation that is not yet ready to retire. _Front-end stalls_ are cycles in which the ROB is empty or cycles which are spent squashing wrong-path execution. _NDA_ policies restrict data propagation and thereby limit dynamic scheduling. Therefore, on average, fewer instructions are committed in a given cycle, increasing the overall number of _commit_ cycles. Since instruction-level parallelism for both memory and non-memory instructions is reduced, more cycles are spent on _memory stalls_ and _back-end stalls_ . _Front-end stall_ cycles generally vary little across designs, on average contributing only 2% of the difference in cycles. 

**Wake-up Latency.** _NDA_ introduces a delay between instruction completion and tag broadcast. Whereas broadcast delay does not _directly_ affect CPI, the delay propagates to dependent instructions 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

**Figure 9: Aggregated statistics over SPEC 17 benchmarks.** **_(a) NDA_ extend the cycles spent on commit and backend stalls.** **_(b),(c)_ MLP & ILP is still high across** **_NDA_ policies.** **_(d)_ As expected,** **_NDA_ causes delays in latency-to-issue. However, overall impact on CPI is substantially smaller.** **_(e)_ The impact of** **_NDA_ logic latency on CPI is relatively small.** 

in the ROB by delaying their issue. We measure this effect by measuring the average delay instructions experience from dispatch to wake-up under each design. The average latencies across all benchmarks are shown in Fig. 9d. _NDA_ policies add on average 4-39 cycles. This increased latency also manifests in up to 78% increase in cycles spent on _back-end stalls_ , shown in Fig. 9a. However, the wake-up latency has a modest impact on overall performance (CPI). 

**Memory and Instruction Parallelism (MLP/ILP).** The favorable performance of _NDA_ compared to the in-order processor can be explained by observing the Memory- and Instruction-Level Parallelism of each profile. The geometric means of MLP & ILP across all benchmarks are depicted in Fig. 9b-c. We follow Chou et al [14] and report MLP as the average outstanding off-chip misses when at least one is outstanding. Whereas the MLP & ILP in the various _NDA_ profiles are at times lower than the OoO baseline by as much as 6% and 44% (respectively), they are better than the in-order baseline processor by 72% and 39%, where MLP & ILP cannot exceed 1.0. These results suggest that _NDA_ enables execution parallelism among off-chip misses despite the scheduling restrictions of speculative instructions. Importantly, _NDA_ does not typically restrict the issue time of loads, only when they may wake dependents. Ergo, typically only dependent loads are delayed, which do not add to MLP or ILP. 

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

- [44] Microsoft 2018. _Protect your Windows devices against Spectre and Meltdown_ . Microsoft. https://support.microsoft.com/en-us/help/4073757/protectyour-windows-devices-against-spectre-meltdown. 

- [45] Marina Minkin, Daniel Moghimi, Moritz Lipp, Michael Schwarz, Jo Van Bulck, Daniel Genkin, Daniel Gruss, Berk Sunar, Frank Piessens, and Yuval Yarom. 2019. Fallout: Reading Kernel Writes From User Space. (2019). 

- [46] Oleksii Oleksenko, Bohdan Trach, Tobias Reiher, Mark Silberstein, and Christof Fetzer. 2018. You Shall Not Bypass: Employing data dependencies to prevent Bounds Check Bypass. _arXiv preprint arXiv:1805.08506_ (2018). 

- [47] Dag Arne Osvik, Adi Shamir, and Eran Tromer. 2006. Cache attacks and countermeasures: the case of AES. In _Cryptographers Track at the RSA Conference_ . Springer, 120. 

- [48] Lutan Zhao Peinan Li and CAS) Rui Hou (Institute of Information Engineering, CAS); Lixin Zhang (HXT Semiconductor Co.LTD); Dan Meng (Institute of Information Engineering. 2019. Conditional Speculation: An Effective Approach to Safeguard Out-of-Order Execution Against Spectre Attacks. In _Proceedings of the 25th IEEE International Symposium on High-Performance Computer Architecture_ . IEEE. 

- [49] Peter Pessl, Leon Groot Bruinderink, and Yuval Yarom. 2017. To BLISS-B or not to be: Attacking strongSwans Implementation of Post-Quantum Signatures. In _CCS_ . ACM, 18431855. 

- [50] Peter Pessl, Daniel Gruss, Clmentine Maurice, Michael Schwarz, and Stefan Mangard. 2016. DRAMA: Exploiting DRAM Addressing for Cross-CPU Attacks.. In _USENIX Security Symposium_ . 565581. 

- [51] Moinuddin K Qureshi. 2019. CEASER: Mitigating Conflict-Based Cache Attacks via Encrypted-Address and Remapping. In _Proceedings of 51th International Symposium on Microarchitecture_ . 

- [52] Ryan Roemer, Erik Buchanan, Hovav Shacham, and Stefan Savage. 2012. Returnoriented programming: Systems, languages, and applications. _ACM Transactions on Information and System Security (TISSEC)_ 15, 1 (2012), 2. 

- [53] Christos Sakalis, Stefanos Kaxiras, Alberto Ros, Alexandra Jimborean, and Magnus Sjlander. 2019. Efficient Invisible Speculative Execution Through Selective Delay and Value Prediction. In _Proceedings of the 46th International Symposium on Computer Architecture_ . ACM, 723735. 

- [54] Michael Schwarz, Moritz Lipp, Daniel Moghimi, Jo Van Bulck, Julian Stecklina, Thomas Prescher, and Daniel Gruss. 2019. ZombieLoad: Cross-PrivilegeBoundary Data Sampling. _arXiv:1905.05726_ (2019). 

- [55] Michael Schwarz, Martin Schwarzl, Moritz Lipp, and Daniel Gruss. 2018. Netspectre: Read arbitrary memory over network. _arXiv preprint arXiv:1807.10535_ (2018). 

- [56] Hovav Shacham. 2007. The geometry of innocent flesh on the bone: Returninto-libc without function calls (on the x86). In _Proceedings of the 14th ACM conference on Computer and communications security_ . ACM, 552561. 

- [57] Zhuojia Shen, Jie Zhou, Divya Ojha, and John Criswell. 2019. Restricting Control Flow During Speculative Execution with Venkman. _arXiv preprint arXiv:1903.10651_ (2019). 

- [58] SPEC. 2017. Standard Performance Evaluation Corporation SPEC CPU 2017. https://www.spec.org/cpu2017/. 

- [59] Julian Stecklina and Thomas Prescher. 2018. LazyFP: Leaking FPU Register State using Microarchitectural Side-Channels. _arXiv preprint arXiv:1806.07480_ (2018). 

- [60] Mohammadkazem Taram, Ashish Venkat, and Dean Tullsen. 2019. ContextSensitive Fencing: Securing Speculative Execution via Microcode Customization. In _Proceedings of the Twenty-Fourth International Conference on Architectural Support for Programming Languages and Operating Systems_ . 

- [61] Ubuntu 2018. _Spectre And Meltdown_ . Ubuntu. https://wiki.ubuntu.com/ SecurityTeam/KnowledgeBase/SpectreAndMeltdown. 

- [62] Jo Van Bulck, Marina Minkin, Ofir Weisse, Daniel Genkin, Baris Kasikci, Frank Piessens, Mark Silberstein, Thomas F Wenisch, Yuval Yarom, and Raoul Strackx. 2018. Foreshadow: Extracting the keys to the Intel SGX kingdom with transient out-of-order execution. In _Proceedings of the 27th USENIX Security Symposium. USENIX Association_ . 

- [63] Jo Van Bulck, Frank Piessens, and Raoul Strackx. 2018. Nemesis: Studying Microarchitectural Timing Leaks in Rudimentary CPU Interrupt Logic. (2018). 

- [64] Stephan van Schaik, Alyssa Milburn, Sebastian Usterlund, Pietro Frigo, Giorgi<sup></sup> Maisuradze, Kaveh Razavi, Herbert Bos, and Cristiano Giuffrida. 2019. RIDL: Rogue In-flight Data Load. In _S&P_ . 

- [65] Ofir Weisse, Jo Van Bulck, Marina Minkin, Daniel Genkin, Baris Kasikci, Frank Piessens, Mark Silberstein, Raoul Strackx, Thomas F. Wenisch, and Yuval Yarom. 2018. Foreshadow-NG: Breaking the Virtual Memory Abstraction with Transient Out-of-Order Execution. _Technical report_ (2018). See also USENIX Security paper Foreshadow [62]. 

- [66] Zhenyu Wu, Zhang Xu, and Haining Wang. 2012. Whispers in the Hyper-space: High-speed Covert Channel Attacks in the Cloud.. In _USENIX Security symposium_ . 159173. 

- [67] Roland E Wunderlich, Thomas F Wenisch, Babak Falsafi, and James C Hoe. 2003. SMARTS: Accelerating microarchitecture simulation via rigorous statistical sampling. In _ACM SIGARCH Computer Architecture News_ , Vol. 31. ACM, 8497. 

- [68] Yunjing Xu, Michael Bailey, Farnam Jahanian, Kaustubh Joshi, Matti Hiltunen, and Richard Schlichting. 2011. An exploration of L2 cache covert channels in virtualized environments. In _Proceedings of the 3rd ACM workshop on Cloud computing security workshop_ . ACM, 2940. 

- [69] Mengjia Yan, Jiho Choi, Dimitrios Skarlatos, Adam Morrison, Christopher W Fletcher, and Josep Torrellas. 2018. InvisiSpec: Making Speculative Execution Invisible in the Cache Hierarchy. In _Proceedings of the 51th International Symposium on Microarchitecture (MICRO18)_ . 

- [70] Mengjia Yan, Bhargava Gopireddy, Thomas Shull, and Josep Torrellas. 2017. Secure hierarchy-aware cache replacement policy (SHARP): Defending against cache-based side channel attacks. In _Computer Architecture (ISCA), 2017 ACM/IEEE 44th Annual International Symposium on_ . IEEE, 347360. 

- [71] Mengjia Yan, Yasser Shalabi, and Josep Torrellas. 2016. ReplayConfusion: detecting cache-based covert channel attacks using record and replay. In _The 49th Annual IEEE/ACM International Symposium on Microarchitecture_ . IEEE Press, 39. 

- [72] Mengjia Yan, Read Sprabery, Bhargava Gopireddy, Christopher Fletcher, Roy Campbell, and Josep Torrellas. 2019. Attack Directories, Not Caches: Side Channel Attacks in a Non-Inclusive World. In _40th IEEE Symposium on Security and Privacy_ . 

- [73] Yuval Yarom and Katrina Falkner. 2014. FLUSH+RELOAD: a High Resolution, Low Noise, L3 Cache Side-Channel Attack. In _USENIX Security_ . 719732. 

- [74] Kenneth C Yeager. 1996. The MIPS R10000 superscalar microprocessor. _IEEE micro_ 16, 2 (1996), 2841. 

- [75] Project Zero. 2018. speculative execution, variant 4: speculative store bypass. https://bugs.chromium.org/p/project-zero/issues/detail?id=1528. 

MICRO-52, October 1216, 2019, Columbus, OH, USA 

NDA: Preventing Speculative Execution Attacks at Their Source 

- [76] Yinqian Zhang, Ari Juels, Michael K. Reiter, and Thomas Ristenpart. 2014. CrossTenant Side-Channel Attacks in PaaS Clouds. In _ACM Conference on Computer_ 

_and Communications Security_ . ACM, 9901003. 

# SOURCE: presentation.pdf

# **<mark>Meltdown and Spectre</mark>** Yuhao Jiang, Daiqi Guo 

## **What are meltdown and spectre?** 

They are the nicknames for the three vulnerabilities: 

 Variant 1: bounds check bypass (CVE-2017-5753)  Variant 2: branch target injection (CVE-2017-5715)  Variant 3: rogue data cache load (CVE-2017-5754) Where Variant 1 & 2 are Spectre and Variant 3 is Meltdown 

## **What do they affect?** 

- Affects some/all modern processors, servers, mobile phones (Apple SoCs) 

- Meltdown 

   - Intel, ARM, IBM  

   - Desktop, Laptop, and Cloud computers 

- Spectre 

   - Intel, AMD, ARM, IBM  

   - Desktops, Laptops, Cloud Servers, as well as Smartphones 

- Affects all operating systems  Linux, Windows, MacOS ... 

## **What do they affect? (cont.)** 

- Meltdown: 

   - Breaks the most fundamental isolation between user applications and the operating system. 

   - Allows a program to access the memory, and thus also the secrets, of other programs and the operating system. 

- Spectre: 

   - Breaks the isolation between different applications. 

   - Allows an attacker to trick error-free programs, which follow best practices, into leaking their secrets. 

## **What do they exploit?** 

- Exploit the three major designs in modern processors: 

   - Out-of-order Execution 

   - Speculative Execution 

   - Caching 

- Both attacks use side channels to obtain the information from the accessed memory location. 

## **What is Out-of-order Execution?** 

- It is an approach to processing that allows instructions for high-performance microprocessors to begin execution as soon as their operands are ready. 

- Although instructions are issued in-order, they can proceed out-of- order with respect to each other. 

- The goal of OoO processing is to allow the processor to avoid a class of stalls that occur when the data needed to perform an operation are unavailable. 

## **Out-of-order Execution steps** 

1. Instruction fetch. 

2. Instruction dispatch to an instruction queue (also called instruction buffer or reservation stations). 

3. The instruction waits in the queue until its input operands are available. The instruction is then allowed to leave the queue before earlier, older instructions. 

4. The instruction is issued to the appropriate functional unit and executed by that unit. 

5. The results are queued. 

6. Only after all older instructions have their results written back to the register file, then this result is written back to the register file. This is called the graduation or retire stage. 

## **Life example of Out-of-order Execution: make tea** 

## **Life example of Out-of-order Execution: make tea** 

- wash tea cups -> boiling water -> make tea 

- wash tea cups ---------------------------------------->-------| 

   - |-->wait for use------>-----|---->make tea 

   - boiling water -> boiled->| 

- wash tea cups -------------------------------------->break cups-->--| 

   - |-->wait for use-->----------------------|-->water not use 

   - boiling water -> boiled->| 

- Because the cups are broken when washing them (raise error), the boiled water wont be used in next steps. 

- However, dont use the boiled water doesnt  mean the boiled water will disappear, it is still placed in the waitting area (caching). 

## **What is Speculative Execution?** 

- It is a technique used by modern CPUs to speed up performance. The CPU may execute certain tasks ahead of time, "speculating" that they will be needed and complete them. 

- If the tasks are required, a speed-up is achieved, because the work is already complete. 

- If the tasks are not required, changes made by the tasks are reverted and the results are ignored. 

## **Life example of Speculative Execution: order coffee** 

## **Life example of Speculative Execution: order coffee** 

 Barista: make Latte   || speculate:need Latte ->make Latte->available Latte---| Customer: need Latte   || need Latte -|-> Got it ! Days: day 1 || day 2 

 Barista: make Latte || speculate Latte || speculate Latte - make it -|-> make French Vanilla Customer: need Latte || need Latte         || need French Vanilla---------|     & throw away Latte Days: day 1 || day 2 || day 3 

## **Caching** 

- The CPU requests data from memory which is stored in a cache 

- Speeds up memory access 

- Temporal locality: something which was accessed recently from memory might be accessed again soon 

   - Ex. a counter in a loop 

- Spatial locality: something which is close to another thing which was accessed recently might be accessed soon 

   - Ex. elements in an array 

## **What is side-channel attack?** 

- Attack which is enabled by the micro architectural design of the CPU and based on information gained from the implementation of a computer system. 

   - Caches: attack which monitors how quickly data accesses take and infer whether or not said data was in the cache 

   - Timing: attack which monitors time it takes for machine to do various computations 

   - Power-monitoring: attack which monitors power consumption of hardware on varius computations 

   - 

- ... 

## **Cache side-channel attack** 

- The side channel comes from monitoring how quickly data can be accessed from the cache. 

- Data which is accessed quickly => stored in the cache 

- Data which is accessed slow => stored in main memory 

## **Exploit Caching** 

- Flush + Reload 

   - Flush any access of memory for data you control from the cache (by clflush) 

   - Lets malicious (or user program) run and access memory you control with secret 

   - Try reloading elements from the controlled memory and see how quickly they are accessed 

- Evict + Reload 

   - Evicting memory access of data you control by loading other (possibly random) data into the cache 

   - Due to limited size of cache evict the specific cache line 

   - Let victim program run and access memory using secret, reload data and measure access time 

## **Privilege check** 

- Modern CPUs enforce a privilege check of a program accessing kernel memory 

- This privilege check sometimes occurs too late during speculative execution. (i.e. once the data has already been read). 

-  Priviledge check arent performed until the instruction is completed. 

-  The CPUs knows that this occurs so anything unprivileged which was executed will be forgotten and an exception will be raised (usually SIGSEGV) 

   - As a result, the memory that accessed recently is still stored in cache 

## **Meltdown attack** 

This is the core of Meltdown, lets walk through it step by step. 

## **Meltdown attack** 

- Step 1, first of all, allocates a block of memory consisting of 256 pages of memory (256 * 4096 bytes), we denote it as RBX here. 

- Each page in this block of memory wont be cached at this point because it has never been accessed. 

## **Meltdown attack** 

- Step 2, line 2: xor rax, rax 

- This step is used for empty the register rax with all zeros. 

## **Meltdown attack** 

- Step 3, line 4: mov al, byte [rcx] 

- Load the byte value located at the target kernel address which stored in the register RCX, into the least significant byte of the register RAX represented by AL. 

## **Relationship between register AL and RAX** 

0x1122334455667788 

================ rax (64 bits) 

======== eax (32 bits) ==== ax (16 bits) ==      ah (8 bits) == al (8 bits) 

## **Meltdown attack** 

- Step 4, line 5: shl rax, 0xc 

- shift left the content in RAX with 12 bits, in another word, (value in RAX)*(2^12) => (value in RAX)*4096 

- 4096 => 4096B => 4KB, the size of a page 

## **Meltdown attack** 

- Step 5, line 6: jz retry 

-  If we copied nothing into the register AL, the register RAX keeps all zeros, then retry this loop until we copied something into the register AL and make register RAX contain something. 

## **Meltdown attack** 

- Step 6, line 7: mov rbx, qword [rbx + rax] 

- Copied the value into the probe array RBX at index RAX 

- Use that multiplied value as an index into the block of allocated memory and read one byte (ie: read one byte from page N where N is the value in RAX) 

## **Meltdown attack** 

- Assuming the CPU starts out-of-order execution 

   - There is a race condition between the privilege check of line 4 codes and the codes after line 4 

   - The privilege check may finished after line 5 code. 

   - It will cause one page of the allocated block of memory to be cached on the CPU. 

- The page that is cached will be directly related to the byte read from kernel mode memory. 

   - For example: if the value of the one byte from kernel address is 21, then the 21st page of the allocated memory block will now be cached on the CPU. 

## **Meltdown attack** 

- Finally, the attacker observes the side effects of this out-of-order execution to determine the secret byte that was read. 

   - Catch the exception thrown by privilege check from line 4 code above. 

   - Loop through every page in the allocated block of memory 

   - Time how long it takes to read one byte from each page. 

   - If the byte loads quickly then the page must have been cached and gives away the secret. 

- Continuing the example from previous slide, pages 0 through 20 of the allocated memory block would be slow to read, but page 21 would be considerably faster  so the secret value must be 21. 

## **Spectre attack Variant 1** 

- Exploiting Conditional Branch Misprediction 

- The code above is an example of conditional branch 

- x = (address of a secret byte to read) - (base address of array1) 

## **Spectre attack Variant 1** 

- This code looks normal and correct 

- If x is less than the length of array1, the loop executes successfully 

- But let's assume that we have a variable here that stores the password at the address secret, and let A=secret-array1, so we can use array1[A] to represent the value of secret. 

## **Spectre attack Variant 1** 

- When the x satisfy the loop condition and we execute this loop for multiple times, the branch predictor will think the next loop also satisfies the loop condition and execute this loop. 

## **Spectre attack Variant 1** 

- If at this time we assigned the value A to the x, the branch predictor will predict the loop for execution (actually should not execute), the CPU will execute the loop body, and then load the password secret value in cache, and use it as the address to access array2. 

- But eventually, the CPU will found this loop should not be executed, so the value got in this loop will become invalid. 

## **Spectre attack Variant 1** 

 Finally, we can read array2, and if we read an address for a short amount of time, that address is the one that is cached (our password value). 

## **Spectre attack Variant 2** 

- Poisoning Indirect Branches 

- Indirect Branch: jumping to code at some memory location 

   - e.g. jmp [eax] => jump to instruction stored at memory address in register EAX 

- Variant 2 is much like variant 1, but instead of abusing the data lookup portion of the CPU, it abuses the ability for a CPU to predict which way it will go when a function pointer is called. 

- The attacker needs to locate a Spectre gadget, i.e., a code fragment whose speculative execution will transfer the victims sensitive information into a covert channel. 

## **Spectre attack Variant 2** 

- Attacker chooses a Spectre gadget from the victims address space and trains the Branch Target Buffer (BTB) to mispredict a branch from an indirect branch instruction to the address of the gadget, resulting in speculative execution of the gadget. 

   - Not reliant on the vulnerability of victims code. 

   -  Attacker has to find the virtual address of gadget 

- Exploiting Branch Target Buffer (BTB) 

## **Branch Target Buffer (BTB)** 

- The Branch Target Buffer (BTB) keeps a mapping from addresses of recently executed branch instructions to destination addresses . 

- Processors can use the BTB to predict future code addresses even before decoding the branch instructions. 

   - Using Speculative Execution to improve the performance 

- Only the 31 least significant bits of the branch address are used to index the BTB. 

## **Branch Target Buffer (BTB)** 

- Allows the CPU to speculatively execute code at predicted indirect branch target without actually having decoded the branch instructions 

- Attacker trains the Branch Target Buffer (BTB) to mispredict a branch from an indirect branch instruction to the address of the gadget. 

## **Code example of Branch Target Buffer misprediction** 

## **Spectre attack Variant 2** 

- As a result, the gadget code was run by speculative execution because of branch misprediction 

   - The result will be loaded into the cache 

   - Use cache side-channel attack to gain the secert value 

## **Mitigation** 

- Anyway, the best way to solve these hardware vulnerabilities is through the hardware way, i.e. re-designing the CPU. However, it may take a lot time and cost huge amount of money. 

- Not all computer users will have the money, time or skills to change the computer CPU. 

- So, there come out some software patches to mitigate these vulnerabilities through a software way. 

## **Meltdown Mitigation** 

- Luckily, there are software patches against Meltdown. 

- So, update your Operating System and Softwares to the newest version! 

- For Linux, this software patch is called KPTI (formerly KAISER) 

   - Kernel page-table isolation 

   -  Kernel address isolation to have side-channels efficiently removed 

   -  Still have time punishment 

## **Meltdown Mitigation** 

 KPTI implements two page tables for each process. One is essentially unchanged and includes both kernel-space and user-space addresses, and is only used when the system is running in kernel mode. 

## **Meltdown Mitigation** 

 The second "shadow" page table contains a copy of all of the user-space mappings, but leaves out the kernel side. Instead, there is a minimal set of kernel-space mappings that provides the information needed to handle system calls and interrupts, but no more. 

## **Meltdown Mitigation** 

 Whenever a process is running in user mode, the shadow page tables will be active. The bulk of the kernel's address space will thus be completely hidden from the process, defeating the known hardware-based attacks. 

**Meltdown Mitigation**  Whenever the system needs to switch to kernel mode, response to system call, exception, or interrupt, a switch to the other page tables will be made. The code that manages the return to user space must then make the shadow page tables active again.  KASLR:  kernel address space layout randomization Randomizes the location of the kernel address space on every boot 

## **Spectre Mitigation** 

- Spectre is harder to exploit than Meltdown, but it is also harder to mitigate. However, it is possible to prevent specific known exploits based on Spectre through software patches. 

- Remember to update your Operating System and Softwares to the newest version for keeping known Spectre attack away. 

## **Spectre Mitigation** 

- Preventing Speculative Execution: 

- Ensure control flow leads the instruction 

-  Software using serialization or speculation blocking 

-  Causing a significant degradation in the performance 

-  Preventing Access to Secret Data (more for JIT compiler)  Chrome: each website per process 

-  Limiting Data Extraction from Covert Channels 

- Preventing Branch Poisoning 

## **Spectre Mitigation** 

- Preventing Data from Entering Covert Channels 

   - Future processors (no such design is currently available) 

- KAISER/KPTI does not help for Mitigation 

- Google also have posted a patch called Retpoline for mitigating Spectre Variant 2 

- Other Linux Spectre mitigation details: <u><mark>https://www.kernel.org/doc/html/latest/admin-guide/hw-vuln/spectre.html#turning-on-mitigation-for-spectre-variant-1-and-spectre</mark></u> 

<mark>-variant-2</mark> 

**Command line code for checking the vulnerabilities** To see if the computer(Linux) has the meltdown and spectre vulnerabilities: 

$ git clone https://github.com/speed47/spectre-meltdown-checker.git 

We can see there are still some Variants of Spectre are not solved. 

## **Reference** 

<u>https://meltdownattack.com/ https://meltdownattack.com/meltdown.pdf https://spectreattack.com/spectre.pdf https://searchdatacenter.techtarget.com/definition/out-of-order-execution https://www.computerhope.com/jargon/s/spec-exec.htm https://www.blackhat.com/docs/asia-17/materials/asia-17-Irazoqui-Cache-Side-Channel-Attack-Exploitability-And-Countermeasures.pdf https://www.mikelangelo-project.eu/2016/09/cache-based-side-channel-attacks/ https://conference.hitb.org/hitbsecconf2016ams/materials/D2T1%20-%20Anders%20Fogh%20-%20Cache%20Side%20Channel%20At tacks.pdf https://hackernoon.com/a-simplified-explanation-of-the-meltdown-cpu-vulnerability-ad316cd0f0de</u> ~ <u>http://www.cs.toronto.edu/ arnold/427/18s/427_18S/indepth/spectre_meltdown/index.html http://www.cs.toronto.edu/~arnold/427/19s/427_19S/indepth/sm/Meltdown-and-Spectre.pdf https://events19.linuxfoundation.org/wp-content/uploads/2017/11/Spectre-Meltdown-Linux-Greg-Kroah-Hartman-The-Linux-Foundation .pdf https://lwn.net/Articles/738975/</u> 

