
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

