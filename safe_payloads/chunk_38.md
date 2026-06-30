
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
