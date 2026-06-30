
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
