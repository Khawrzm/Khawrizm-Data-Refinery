
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
