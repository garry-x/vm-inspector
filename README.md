# vm-inspector
Tool helping to analyse the behaviors for a VM.
  
These behaviors include:  
- The memory consumption for a running VM.  
- The boot time for a VM. (planned)   
- The cpu usage for a running VM. (planned)  
  
Following hypervisors are supported:  
- Qemu
- Cloud Hypervisor  
  
### How to use  
- Clone the source repo.
- Prepare a RUST environment (See https://rustup.rs), and build the tool.
```
cargo build --release
```
- Now, you are prepared to have a try. See the usage bellow (You need to run this tool under  'root' privilege ): 

```
Usage: vm-inspector mem [OPTIONS] --pid <PID>

Options:
  -p, --pid <PID>  PID of the hypervisor process
  -c, --cmd        Show the command line arguments
  -s, --status     Show the statistics of RSS usage
  -v, --vmas       Show the detailed RSS usage for each VMA
  -h, --help       Print help information
```
An example: 
```
$ sudo ./target/release/vm-inspector mem -p 1780842 -s
--------Process Status--------
Pid:            1780842
VmRSS:          443260 KB
 -> RssAnon:    409704 KB   (92%)
 -> RssFile:    33552 KB
 -> RssShmem:   4 KB
HugetlbPages:   8388608 KB

$ sudo ./target/release/vm-inspector mem -p 1780842 -v
---------Process VMAs---------
RSS (Shared Libs): 22332 KB
544 VMAs with RSS usage > 0:
5560EA1FB000 -> 5560EB498000 RSS:      19060 KB [heap]
7F43C4000000 -> 7F43C50EB000 RSS:      17324 KB
7F43D4000000 -> 7F43D4F3E000 RSS:      15608 KB
7F43BC000000 -> 7F43BCDA1000 RSS:      13956 KB
7F45E9F6B000 -> 7F45EACAF000 RSS:      13584 KB
5560E90A9000 -> 5560E9C22000 RSS:      11748 KB /usr/libexec/qemu-kvm
7F43D93BC000 -> 7F43D9DFC000 RSS:      10496 KB
7F43D9DFD000 -> 7F43DA5FD000 RSS:       8192 KB
7F438B800000 -> 7F438C000000 RSS:       8192 KB
7F43AF800000 -> 7F43B0000000 RSS:       8192 KB
7F43B53A3000 -> 7F43B5BA3000 RSS:       8192 KB
7F43DA5FE000 -> 7F43DADFE000 RSS:       8192 KB
7F43DADFF000 -> 7F43DB5FF000 RSS:       8192 KB
7F43DB600000 -> 7F43DBE00000 RSS:       8192 KB
7F45EB033000 -> 7F45EB833000 RSS:       8192 KB
7F45EBB48000 -> 7F45EC348000 RSS:       8192 KB
......
```