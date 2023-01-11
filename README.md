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
```