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
VMA (Shared Libs) RSS: 22332 KB
VMA (Others with RSS or Hugetlb pages > 0):
 Start Addr  ->   End Addr       RSS          Hugetlb       Anonymous        Notion
7F43DBE00000 -> 7F45DBE00000           0 KB     8388608 KB           0 KB  /dev/hugepages/libvirt/qemu/2-centos8201/qemu_back_mem.pc.ram.GgqkHP
5560EA1FB000 -> 5560EB498000       19060 KB           0 KB       19060 KB  [heap]
7F43C4000000 -> 7F43C50EB000       17324 KB           0 KB       17324 KB
7F43D4000000 -> 7F43D4F3E000       15608 KB           0 KB       15608 KB
7F43BC000000 -> 7F43BCDA1000       13956 KB           0 KB       13956 KB
7F45E9F6B000 -> 7F45EACAF000       13584 KB           0 KB       13584 KB
5560E90A9000 -> 5560E9C22000       11748 KB           0 KB           0 KB  /usr/libexec/qemu-kvm
7F43D93BC000 -> 7F43D9DFC000       10496 KB           0 KB       10496 KB
7F43B43A1000 -> 7F43B4BA1000        8192 KB           0 KB        8192 KB
7F43B4BA2000 -> 7F43B53A2000        8192 KB           0 KB        8192 KB
7F45EB033000 -> 7F45EB833000        8192 KB           0 KB        8192 KB
7F43AF800000 -> 7F43B0000000        8192 KB           0 KB        8192 KB
7F4389FFD000 -> 7F438A7FD000        8192 KB           0 KB        8192 KB
7F43D9DFD000 -> 7F43DA5FD000        8192 KB           0 KB        8192 KB
7F43DA5FE000 -> 7F43DADFE000        8192 KB           0 KB        8192 KB
......
```