
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
