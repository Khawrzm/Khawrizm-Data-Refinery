
If we consider Plaso parsers, we see a difficulty in applying the permissions factor generically. For example, for the PE parser for portable executables, some will be in user accessible locations, while others will not. Therefore, many sources will need to be evaluated on a source-by-source basis. However, for some that are in one protected location, e.g., prefetch files, the data extracted could be considered to all have the same property on a particular system. 

This leads to the following permission categories: 

User accessible 

User accessible with prompt User accessible with password / biometrics User inaccessible, but observed facets of privilege escalation User inaccessible 

## 5.1.3. Software to edit on system 

This factor addresses the ease by which a manipulation can be made. As direct manipulation of the physical representation of binary data is not possible, at some level of abstraction, a tool will be used to facilitate manipulations. Initially, this was considered from the perspective of whether a tool was available at all that would allow manipulation. However, this becomes very time-sensitive, e.g., a tampering tool release requires an update of all evaluations and is also difficult to provide a comprehensive answer for all possible file types. Therefore, the most sensible approach was to provide a context-sensitive evaluation of this, i.e., whether there is software on the system in question capable of modifying the trace, or observed facets of the use of such software. 

For example, on a Windows system, RegEdit is available, and therefore, ignoring all other factors in this section such as permissions, an adversary can use it to modify keys and values in the Registry. In contrast, a SQLite database cannot reasonably be modified on a system without third-party software or Powershell library (and installation of either could leave traces). 

Another software option is the presence of a hex editor where all files should be considered as editable. In this case, the relative complexity of making such a modification will be captured via other factors as discussed later. 

There are also some edge cases. For example for Windows event logs, the tool Event Viewer is built into Windows. However, this tool only provides the ability to read event logs and does not provide edit capabilities (other than clearing logs). A default tool for editing event logs on Windows should therefore 

4 

be considered in the category of Not on the system. There are also edge cases regarding the Windows Registry. While, as discussed above, Regedit could be used to change data in keys and values, it does not provide access to the last modified timestamp of a key often used in event reconstruction. Therefore, whether a tool is considered available or not, is dependent on the specific facet and what it is being used for. This is by design and will become clear in the examples in Sec. 6. 

A final example of the situation being important would be tampering with a value in the MountedDevices key. Here, Regedit can be used to access that registry key, but the drive letter values are the REG BINARY type, which Regedit will not provide an easy user interface method of tampering, but will provide a view that allows the hex to be manually edited. Therefore for this particular trace a summary of Tool available by default for low-level (hex) editing is the most appropriate. 

In summary, the following editing software categories exist: 

Tool available by default for UI-based editing* Tool added to this system for UI-based editing* Tool available by default for low-level (hex) editing Tool added to this system for low-level (hex) editing Not on the system 

However, the database that stores the data is encrypted using SQLCipher4 [25]. The key is available on the system in the config.json file so the database would still be accessible via an SQLbrowser supporting that encryption method. This could be classed as encrypted, but key recoverable is possible from local system. 

Another complexity within the encryption attribute is the different categories of encryption software implementation. In Hargreaves [26] the difference between file-based, file systembased, container-based and full disk encryption is described. An encrypted single file or an encrypted container, where it is not known whether a password is available to the attacker, should be initially considered as one of the last threee Encrypted variants shown below depending on the specific nature of the key/password storage. In contrast, the decryption of filebased (e.g. EFS) or full disk (e.g. Bitlocker) when the system is running means that all data is accessible to the system and the encryption may not be relevant for the accessibility of the data contained therein (subject to other permissions and exact abstraction layer of the tampering attempt). 

This leads to the following encryption categories: 

No encryption 

Encrypted but accessible live (e.g., EFS) 

*UI is used rather than GUI as manipulation tools may be a command line 

## 5.1.4. Observed facets of access 

In addition to considering if there is software on a system capable of accessing a source, it is also important to determine if there are observed facets of actual access to that source. For SQLite database viewers, the recent files list associated with the program may provide evidence of a specific database being accessed. In some cases, this information might be even more detailed; for instance, the Registry key /NTUSER.DAT/Software/Microsoft/Windows/CurrentVe rsion/Applets/RegEdit could indicate that a specific key was accessed using RegEdit. However, there may also be scenarios where no explicit traces of a relevant source being accessed are observable, but evidence shows that the associated program was executed. We define the following observed facets of access categories: 

Observed facets of edit-capable software accessing the specific source 

Observed facets of edit-capable software accessing the source Observed facets of edit-capable software being run No observed facets of source access 

Encrypted (trivial to break) e.g., ROT13 in Windows Registry Encrypted (key recovery possible from local system) Encrypted (key stored off device available to user) Encrypted (key stored off device not available to user) 

## 5.1.6. (File) Format 

The format of a source also impacts its resilience. There are many different (file) formats, and it is impossible to try and list them all here. However, given that in Sec. 5.1.3 we consider hex editors as software that allows editing of a source, it is important to capture the complexity of making such manual edits. We identified some broad categories: Sources could be plain text which would be easy to modify, they could be a structured but still text-based format such as XML or JSON, or they could be a binary format, which may be proprietary, proprietary but reverse engineered, or an open format. We also include a NA category here, to be used when a source is considered where a user interface tool is available since the low-level format becomes irrelevant at that point. 

The (file) format categories can be summarized as follows: 

Binary proprietary (currently unknown) Binary proprietary but reverse-engineered (e.g., MFT) Binary open format (e.g., SQLite) 

Text-based machine format (e.g., XML, JSON) Plain text 

## 5.1.5. Encryption 

NA (GUI edit tool available) 

Another consideration is if the source in question is encrypted. This could be argued as simply an enforcement of permissions, but there are some situations where this is not the same. For example, consider the messaging app Signal, paired with a Windows desktop computer. Here, the files are stored within a users home folder so they could have access to them. 

## 5.1.7. Organization of the source 

The organization of data within a source (structured, semistructured, or unstructured) is another factor impacting its resilience. Generally, more organized structures allow easier au- 

5 

tomation of manipulations (which then allows mass manipulation). As examples, structured data can often be accessed with tools and a potential manipulation is scriptable. For instance, it is possible to develop a Python script that scans for JPG files and manipulates the EXIF information in the header. In contrast, removing a watermark within an image<sup>6</sup> requires utilising artificial intelligence (more processing power) or manual work. Consequently, this factor considers indirectly how difficult it is to automate the manipulation of the contents of a source. Categories would therefore include: 

Structured - timestamp within a known data structure, e.g., MFT Semi-structured  a timestamp that is stored as a field within JSON but as a text string, e.g., Wed 25th Jan 2022 11:35 am Unstructured - within a Word document, within the content itself the author has made reference to a date and time of an event 

## 5.2. Scoring 

The previous section has suggested seven factors, all of which have been argued to affect the extent to which data could be tampered with. Each factor has several options or categories that allow the properties of a specific source or facet within a source to be evaluated and qualitatively described. That level of granularity is sometimes required, for example, one key within a Windows registry may have different properties to another, e.g., the user autorun keys vs the contents of keys in the SAM file, the latter being inaccessible via Regedit to even admin users on a live machine [27]. 

Let us now consider an event reconstruction that relies on some observed facets. If we can assign scores to each of the categories within each of the factors used to describe the source of a facet, then we can use this to begin to evaluate the reliability of an event reconstruction from a tamper resistance perspective. At present, there is no meaningful absolute score that could be assigned to those categories, nor data on the relative importance of each category. However, in other areas, quantitative measures are used which are broader values and are used to rate one situation over another. This is used as inspiration here. For the scoring, we decided to borrow concepts from security risk assessment [28] where the determination of risk can be seen as a function of harm and the likelihood of its occurrence. As the harm is difficult to predict, we use it to express the tampering concern of the source from that factors perspective (the tampering concern is the inverse of tampering resistance). Given a source (e.g., Windows Registry) and a factor (e.g., software to edit), we define three degrees of severity: 

- high (3) means that there is the highest tampering concern of the source from that factors perspective 

- moderate (2) means that there is a moderate tampering concern. 

## low (1) means that there is a low tampering concern. 

> 6Note that this kind of source is currently not considered by timeline generation software. 

We then looked at each factor for each category and assigned a severity where a higher number means that manipulating a source is easier. For instance, the category Cannot be made visible in the user visibility factor received a low score (1). Consequently, if a source is assigned this category, the tampering concern is low / the tampering resistance is high. Even though some categories have received the same severity, we keep the qualitative descriptions separate to facilitate a more granular analysis in the future and to provide provenance as to why a source has been given a particular rating. 

An important note is that each factor is independent of one other so a 3 in user visibility is not equivalent to a 3 in permissions. This means that at this stage a meaningful computed sum is not possible, but it does mean that sources with particular weaknesses can be easily identified. It is crucial to emphasize that evaluating the tamper resistance of the sources used for event reconstruction, as discussed above, is just as important as the numerical scores themselves. This consideration should carry significant weight when contributing to a C-Score assessment. 

## 6. Case Study examples 

We can now consider some examples taking into account the factors and scores. For the production of these examples, a template spreadsheet has been created that captures the factors discussed earlier and the available options are in a dropdown menu, and from that, a color-coded score is displayed. The template is available<sup>7</sup> and can be used to review an event reconstruction. The tables are used in these examples and could be used by an investigator to assist in structuring an assessment of tamper resistance of sources. We also consider them to be a step towards a more automated analysis in the future where some fields could be automatically populated. 
