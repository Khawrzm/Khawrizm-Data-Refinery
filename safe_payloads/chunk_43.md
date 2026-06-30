
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
