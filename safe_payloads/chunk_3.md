
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
