
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
