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
