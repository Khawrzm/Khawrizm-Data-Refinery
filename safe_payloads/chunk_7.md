
To perform a review of an event reconstruction, the expected facets that result from an event need to be known. This can be achieved either through existing documented forensic research, via live logging tools such as Procmon, or using timeline generation software such as Plaso. The relevant observed facets need to be identified and the source from which they are extracted determined. Then the tampering concern of the source can be reviewed and classified according to the factors discussed above. 

A simple example is provided on file creation on NTFS (see Sec. 6.1), and an extended example with multiple variations is given on USB device connection on Windows (see Sec. 6.2). Note, that the goal of this section is not to explore exhaustively each factor category but to improve the understanding of their usability in evaluating the tamper resistance of sources. 

## 6.1. File Creation reconstruction when timestomp is present 

As a first simple example, to illustrate the process, we con- 

> 7The template and examples are available via Google Sheets. To use it, open the link and create a copy (file > duplicate): https://docs.google.com/spreadsheets/d/1DnfYMtprmzp3dGt9SxRo2Jb83ruZHdRMStFz3PzZQ8/ 

6 

|n|Factors|Category<br>Score<br>SI attribute|n|Factors|Category<br>Windows/INF/setupapi.dev.log|Score|
|---|---|---|---|---|---|---|
|1|User visible|Cannot be made visible<br>1|1|User visible|User visible via GUI|3|
|2|Permissions|User inaccessible<br>1|2|Permissions|User accessible|3|
|3|Software to edit|Tool added to this system for UI-based editing<br>3|3|Software to edit|Tool available by default for UI-based editing|3|
|4<br>|Facets of access<br>|Observed facets of edit-capable software be-<br>ing run<br>2<br>|4<br>|Facets of access<br>|Observed facets of edit-capable software be-<br>ing run<br>|2|
|5|Encryption|No encryption<br>3|5|Encryption|No encryption|3|
|6|File format|NA (UI edit tool available)<br>3|6|File format|Plain text|3|
|7|Structural|Structured<br>2|7|Structural|Structured|2|
|||FN attribute||Syst|em/ControlSetxxx/Enum/USBSTOR/||
|8|User visible|Cannot be made visible<br>1|8|User visible|User visible via GUI|3|
|9|Permissions|User inaccessible<br>1|9|Permissions|User accessible with prompt|3|
|10|Software to edit|Not on the system<br>1|10|Software to edit|Tool available by default for UI-based editing|3|
|11|Facets of access|No observed facets of source access<br>1|11|Facets of access|No observed facets of source access|1|
|12|Encryption|No encryption<br>3|12|Encryption|No encryption|3|
|13|File format|Binary proprietary but reverse engineered<br>2|13|File format|NA (UI edit tool available)|3|
|14|Structural|Structured<br>2|14|Structural|Structured|2|
||||||System/MountedDevice||
|ble 1<br>e pre|: Computed severi<br>sence of timestomp|ty for two creation timestamps within the MFT given<br>.|15<br>16<br>17<br>18<br>19|User visible<br>Permissions<br>Software to edit<br>Facets of access<br>Encryption|User visible via GUI<br>User accessible with prompt<br>Tool available by default for UI-based editing<br>No observed facets of source access<br>No encryption|3<br>3<br>3<br>1<br>3|
|ut u|se the example|from Andrade [23] referenced earlier re-|20<br>|File format<br>|Binary proprietary but reverse engineered<br>|2<br>|
||||21|Structural|Structured|2|
|ardi|ng timestomp.|ooking at just the two creation timestamps|||Event Logs||
|ithi<br>b-t|n an MFT entry<br>ables, one for e|, we create Table1which consists of two<br>ach facet (the MFT SI and FN attributes).|22<br>23<br>24|User visible<br>Permissions<br>Software to edit|User visible via GUI<br>User accessible<br>Not on the system|3<br>3<br>1|
|n ro<br>ecau<br>utn|ws 3 and 10 on<br>se timestomp i<br>ottheFNattri|e can see the two diferent scores assigned<br>s capable of modifying the SIA attribute<br>uteThisthencascadesintotheflefor-|25<br>26<br>27<br>28|Facets of access<br>Encryption<br>File format<br>Structural|<br>No observed facets of source access<br>No encryption<br>Binary proprietary but reverse engineered<br>Semi-structured|1<br>3<br>2<br>2|

<!-- Start of picture text -->
SI attribute<br>1 User visible Cannot be made visible 1<br>2 Permissions User inaccessible 1<br>3 Software to edit Tool added to this system for UI-based editing 3<br>4 Facets of access Observed facets of edit-capable software be- 2<br>ing run<br>5 Encryption No encryption 3<br>6 File format NA (UI edit tool available) 3<br>7 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
Windows/INF/setupapi.dev.log<br>1 User visible User visible via GUI 3<br>2 Permissions User accessible 3<br>3 Software to edit Tool available by default for UI-based editing 3<br>4 Facets of access Observed facets of edit-capable software be- 2<br>ing run<br>5 Encryption No encryption 3<br>6 File format Plain text 3<br>7 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
FN attribute<br>8 User visible Cannot be made visible 1<br>9 Permissions User inaccessible 1<br>10 Software to edit Not on the system 1<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/ControlSetxxx/Enum/ USBSTOR/<br>8 User visible User visible via GUI 3<br>9 Permissions User accessible with prompt 3<br>10 Software to edit Tool available by default for UI-based editing 3<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format NA (UI edit tool available) 3<br>14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/MountedDevice<br>15 User visible User visible via GUI 3<br>16 Permissions User accessible with prompt 3<br>17 Software to edit Tool available by default for UI-based editing 3<br>18 Facets of access No observed facets of source access 1<br>19 Encryption No encryption 3<br>20 File format Binary proprietary but reverse engineered 2<br>21 Structural Structured 2<br><!-- End of picture text -->

Table 1: Computed severity for two creation timestamps within the MFT given the presence of timestomp. 

but use the example from Andrade [23] referenced earlier regarding timestomp. Looking at just the two creation timestamps within an MFT entry, we create Table 1 which consists of two sub-tables, one for each facet (the MFT SI and FN attributes). On rows 3 and 10 one can see the two different scores assigned because timestomp is capable of modifying the SIA attribute but not the FN attribute. This then cascades into the file format information which is not relevant when a tool is available in SIA, but is relevant for the FN. We conclude from this that in this context, the timestamp in the FN attribute is more tamper resistant than the SIA attribute, which aligns with the intuitive findings in [23] . An important highlight here is that the granularity of source in this case needs to be at the resolution of MFT attributes, one for the SIA, and one for the FN attribute, since they have different properties. 

<!-- Start of picture text -->
Event Logs<br>22 User visible User visible via GUI 3<br>23 Permissions User accessible 3<br>24 Software to edit Not on the system 1<br>25 Facets of access No observed facets of source access 1<br>26 Encryption No encryption 3<br>27 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
28 Structural Semi-structured 2<br><!-- End of picture text -->

Table 2: Computed severity for four sources used for event reconstruction of USB device connection. 

|n|Factors|Category|Score|
|---|---|---|---|
|||Registry within Shadow Copy||
|1|User visible|Visible via user setting change (not enabled)|1|
|2|Permissions|User accessible with prompt|3|
|3|Software to edit|Not on the system|1|
|4|Facets of access|No observed facets of source access|1|
|5|Encryption|No encryption|3|
|6|File format|Binary proprietary but reverse engineered|3|
|7|Structural|Structured|2|

<!-- Start of picture text -->
Registry within Shadow Copy<br>1 User visible Visible via user setting change (not enabled) 1<br>2 Permissions User accessible with prompt 3<br>3 Software to edit Not on the system 1<br>4 Facets of access No observed facets of source access 1<br>5 Encryption No encryption 3<br>6 File format Binary proprietary but reverse engineered 3<br><!-- End of picture text -->

<!-- Start of picture text -->
7 Structural Structured 2<br><!-- End of picture text -->

## 6.2. USB Device Connection 

Table 3: Computed severity of a Registry within a Windows Shadow Copy 

For the second example, we consider the connection of a USB device on Windows as summarized in Table 2. There are several known locations where modifications are made (setupapi.dev.log, Windows Registry, Event Logs). 

## 6.3. Discussion 

These examples illustrate the difference in the tamper resistance of different sources that are frequently used for event reconstruction. The factors that have been identified have been argued to have an effect on tamper resistance. While there may be additional factors or categories within those factors, this can easily be accommodated. In particular, new categories can be trivially added and appropriate scores assigned depending on if it is easier or harder to tamper with a source with those properties. 

One can see that given some specific conditions on this particular system: notepad has been run but no reference to setupapi.dev.log (row 4), and there are no observed facets of Regedit running on this system (rows 11, 18)). As a direct comparison between the sources, the Windows event logs should be considered the most difficult to tamper with from the set. Thus, if there were conflicting timestamps, from a tampering perspective only, and in the absence of other information, the times in the event logs should be considered the most reliable of the set. 

The examples demonstrated the use for some Windows event reconstructions, but the proposed categories are platformindependent. The examples did not consider external sources of facets, e.g., network server logs, although we acknowledge their importance. For brevity, this paper focuses on single evidence items which allows clarity in terms of the intrinsic resilience of individual sets of local traces. 

An extension of this would be if older copies of the Windows Registry were available within Shadow Copies as illustrated in Table 3. These can be accessed via the command line mounting of restore points (vssadmin) [29]. For that data, the tamper resistance score changes since they are not directly accessible without mounting shadow copies and Regedit cannot directly access those versions of the registry. 

In addition, Table. 4 shows that the difference in tamper resistance is significantly different when a corporate system is considered and the end-user accessibility of many of the sources is reduced compared with Table. 2. 

It has also been shown how factors can be risk scored to highlight differences between sources that are used for event reconstruction. A simplistic high, moderate, and low system has been chosen to provide an easy evaluation of sources, which given 

7 

<!-- Start of picture text -->
Windows/INF/setupapi.dev.log<br>1 User visible Cannot be made visible 1<br>2 Permissions User inaccessible 1<br>3 Software to edit Not on the system 1<br>4 Facets of access No observed facets of source access 1<br>5 Encryption No encryption 3<br>6 File format Plain text 3<br>7 Structural Semi-structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
System/ControlSetxxx/Enum/ USBSTOR/<br>8 User visible Cannot be made visible 1<br>9 Permissions User inaccessible 1<br>10 Software to edit Not on the system 1<br>11 Facets of access No observed facets of source access 1<br>12 Encryption No encryption 3<br>13 File format Binary proprietary but reverse engineered 2<br>14 Structural Structured 2<br><!-- End of picture text -->

<!-- Start of picture text -->
Event Logs<br>15 User visible Cannot be made visible 1<br>16 Permissions User inaccessible 1<br>17 Software to edit Not on the system 1<br>18 Facets of access No observed facets of source access 1<br>19 Encryption No encryption 3<br>20 File format Binary proprietary but reverse engineered 2<br><!-- End of picture text -->

<!-- Start of picture text -->
21 Structural Semi-structured 2<br><!-- End of picture text -->

cases where they differ. It can be used to express the tamper resistance of specific sources or used conjointly with the C-Scale [5] to help assess the tamper resistance of sources when expressing the strength of observed facets in light of competing hypotheses (e.g., understanding and evaluating indicators such as in C-value C2 only one source of evidence that is not difficult to tamper with). Whilst the scoring system is intended to remain partly subjective (to retain some insights into an investigators reasoning process), we imagine it being implemented in software such as Plaso to give an indicator of the tamper resistance of sources in timelines (if prepopulating the scores is feasible, which will part of future work). Finally, the scoring system has educational applications for raising awareness of the risks of relying on observed facets for event reconstruction and encouraging critical questioning of their reliability. 

## 7. Conclusions and further work 

Table 4: Computed severity for three sources when a corporate system is considered. 

that this is currently necessarily a manual process seems an appropriate level of granularity. 

At present manual scoring of sources is necessary, and many are situation/environment dependent, e.g., is the user admin or not. Some could be standardized, for example, if the machine is Windows and the user is the only account and therefore admin, then the Registry is always accessible (clicking through the UAC prompt) and Regedit will always be available. This offers the potential for prepopulating some of the scores. At present, we only offer a manual process, which is more timeconsuming but does encourage deeper thought about the nature of the sources being used for event reconstruction. It is also necessary to have a good knowledge of system behaviour to understand what a user can and cannot access on a system, plus details such as the binary nature of sources. However, we argue that a requirement of detailed knowledge of digital systems and their behaviour should not be an onerous requirement to improve the reliability of event reconstruction. 
