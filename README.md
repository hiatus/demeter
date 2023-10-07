# demeter
Regex-based file harvester for local and remote file systems. `demeter` will search the targeted folder or share for files matching the regular expressions provided respecting configurable constraints such as file size and path depth. If `-o, --output [folder]` is provided, `demeter` will save the matched files on a folder named `[folder]` recreating their parent folder structure on the file system.


## Modules
- `local`: harvest files in the local file system.
- `smb`: harvest files in remote SMB shares.


## Usage Examples
### Local Harvesting
Search for files with extension `.pub` or `.pem` on a user's home directory and save harvested files at `/tmp/harvest`:
```bash
user@host:~$ demeter local -r '.*\.(pem|pub)$' -o /tmp/harvest /home/user
=> /home/user/.ssh
   id_rsa.pub
=> /home/user/Desktop/lab-server
   lab02_public.pem
   lab02_private.pem

[+] 3 files matched
```
The contents of `/tmp/harvest`:
```
user@host:~$ ls /tmp/harvest/**
/tmp/harvest/:
.ssh  Desktop

/tmp/harvest/.ssh:
id_rsa.pub

/tmp/harvest/Desktop:
lab-server

/tmp/harvest/Desktop/lab-server:
lab02_private.pem  lab02_public.pem
```

### SMB Harvesting
Save a list of regular expressions to match on the remote share:
```bash
user@host:~$ vim regexes.txt
user@host:~$ cat regexes.txt
[Pp][Aa][Ss]([Ww][Oo][Rr][Dd])?
[Tt][Oo][Kk][Ee][Nn]
.*\.rdp$
.*\.env$
.*config$
.*\.[Jj][Pp][Ee]?[Gg]$
.*\.[Pp][Nn][Gg]$
```
Search the `User` share on host `10.18.98.115` for files that match the provided regular expressions with at most 1Mb in size and within a maximum depth of 5 folders. Save harvested files at `/tmp/harvest`.
```bash
user@host:~$ demeter smb -u mozart -p 'P4ssw0rd@123' -D WORKGROUP -S User -d 5 -s 1048576 -R regexes.txt -o '/tmp/harvest' smb://10.18.98.115
=> /
   .gitconfig
=> /Desktop
   api_tokens.txt
   Wallpaper.jpeg 
=> /Documents
   Default.rdp
   Enable-Passthrough.pdf (skipping download: file size 7235011 > 1048576)
=> /source/repos/Project0/Project0
   App.config

[+] 6 files matched
```
The contents of `/tmp/harvest`:
```
user@host:~$ ls /tmp/harvest/**
/tmp/harvest/:
Desktop  Documents  source

/tmp/harvest/Desktop:
api_tokens.txt  Wallpaper.jpeg

/tmp/harvest/Documents:
Default.rdp

/tmp/harvest/source:
repos

/tmp/harvest/source/repos:
Project0

/tmp/harvest/source/repos/Project0:
Project0

/tmp/harvest/source/repos/Project0/Project0:
App.config
```