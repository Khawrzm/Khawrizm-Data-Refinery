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
