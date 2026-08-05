# "Fan Remote"

A self-contained Rust binary to expose a single X10 command (turn off that fan)
as an HTML form button.

In its current form, it's highly specialized, but I've put so much work into
locking down the systemd sandboxing for it that you might find it useful as a
starting point for creating ultra-sandboxed HTTP daemons that need to invoke
`/usr/local/bin/pycm19a.py` or something similar.

**NOTE:** If you have an X10 Firecracker CM17A, an older version of this project
with stricter sandboxing is available in the `cm17a` branch. I haven't had time
to look into how to sandbox `pycm19a.py` more effectively but, if you know how,
please let me know.

Here's what `systemd-analyze security` has to say about it as of this writing:

```none
  NAME                                                        DESCRIPTION                                                                    EXPOSURE
✓ PrivateNetwork=                                             Service has no access to the host's network
✓ User=/DynamicUser=                                          Service runs under a transient non-root user identity
✓ CapabilityBoundingSet=~CAP_SET(UID|GID|PCAP)                Service cannot change UID/GID identities/capabilities
✓ CapabilityBoundingSet=~CAP_SYS_ADMIN                        Service has no administrator privileges
✓ CapabilityBoundingSet=~CAP_SYS_PTRACE                       Service has no ptrace() debugging abilities
✓ RestrictAddressFamilies=~AF_(INET|INET6)                    Service cannot allocate Internet sockets
✓ RestrictNamespaces=~CLONE_NEWUSER                           Service cannot create user namespaces
✓ RestrictAddressFamilies=~…                                  Service cannot allocate exotic sockets
✓ CapabilityBoundingSet=~CAP_(CHOWN|FSETID|SETFCAP)           Service cannot change file ownership/access mode/capabilities
✓ CapabilityBoundingSet=~CAP_(DAC_*|FOWNER|IPC_OWNER)         Service cannot override UNIX file/IPC permission checks
✓ CapabilityBoundingSet=~CAP_NET_ADMIN                        Service has no network configuration privileges
✓ CapabilityBoundingSet=~CAP_SYS_MODULE                       Service cannot load kernel modules
✓ CapabilityBoundingSet=~CAP_SYS_RAWIO                        Service has no raw I/O access
✓ CapabilityBoundingSet=~CAP_SYS_TIME                         Service processes cannot change the system clock
✗ DeviceAllow=                                                Service has no device ACL                                                           0.2
✓ IPAddressDeny=                                              Service blocks all IP address ranges
✓ KeyringMode=                                                Service doesn't share key material with other services
✓ NoNewPrivileges=                                            Service processes cannot acquire new privileges
✓ NotifyAccess=                                               Service child processes cannot alter service state
✗ PrivateDevices=                                             Service potentially has access to hardware devices                                  0.2
✓ PrivateMounts=                                              Service cannot install system mounts
✓ PrivateTmp=                                                 Service has no access to other software's temporary files
✓ PrivateUsers=                                               Service does not have access to other users
✗ ProtectClock=                                               Service may write to the hardware clock or system clock                             0.2
✓ ProtectControlGroups=                                       Service cannot modify the control group file system
✓ ProtectHome=                                                Service has no access to home directories
✓ ProtectKernelLogs=                                          Service cannot read from or write to the kernel log ring buffer
✓ ProtectKernelModules=                                       Service cannot load or read kernel modules
✓ ProtectKernelTunables=                                      Service cannot alter kernel tunables (/proc/sys, …)
✓ ProtectProc=                                                Service has restricted access to process tree (/proc hidepid=)
✓ ProtectSystem=                                              Service has strict read-only access to the OS file hierarchy
✓ RestrictAddressFamilies=~AF_PACKET                          Service cannot allocate packet sockets
✓ RestrictSUIDSGID=                                           SUID/SGID file creation by service is restricted
✓ SystemCallArchitectures=                                    Service may execute system calls only with native ABI
✓ SystemCallFilter=~@clock                                    System call allow list defined for service, and @clock is not included
✓ SystemCallFilter=~@debug                                    System call allow list defined for service, and @debug is not included
✓ SystemCallFilter=~@module                                   System call allow list defined for service, and @module is not included
✓ SystemCallFilter=~@mount                                    System call allow list defined for service, and @mount is not included
✓ SystemCallFilter=~@raw-io                                   System call allow list defined for service, and @raw-io is not included
✓ SystemCallFilter=~@reboot                                   System call allow list defined for service, and @reboot is not included
✓ SystemCallFilter=~@swap                                     System call allow list defined for service, and @swap is not included
✓ SystemCallFilter=~@privileged                               System call allow list defined for service, and @privileged is not included
✓ SystemCallFilter=~@resources                                System call allow list defined for service, and @resources is not included
✓ AmbientCapabilities=                                        Service process does not receive ambient capabilities
✓ CapabilityBoundingSet=~CAP_AUDIT_*                          Service has no audit subsystem access
✓ CapabilityBoundingSet=~CAP_KILL                             Service cannot send UNIX signals to arbitrary processes
✓ CapabilityBoundingSet=~CAP_MKNOD                            Service cannot create device nodes
✓ CapabilityBoundingSet=~CAP_NET_(BIND_SERVICE|BROADCAST|RAW) Service has no elevated networking privileges
✓ CapabilityBoundingSet=~CAP_SYSLOG                           Service has no access to kernel logging
✓ CapabilityBoundingSet=~CAP_SYS_(NICE|RESOURCE)              Service has no privileges to change resource use parameters
✓ RestrictNamespaces=~CLONE_NEWCGROUP                         Service cannot create cgroup namespaces
✓ RestrictNamespaces=~CLONE_NEWIPC                            Service cannot create IPC namespaces
✓ RestrictNamespaces=~CLONE_NEWNET                            Service cannot create network namespaces
✓ RestrictNamespaces=~CLONE_NEWNS                             Service cannot create file system namespaces
✓ RestrictNamespaces=~CLONE_NEWPID                            Service cannot create process namespaces
✓ RestrictRealtime=                                           Service realtime scheduling access is restricted
✓ SystemCallFilter=~@cpu-emulation                            System call allow list defined for service, and @cpu-emulation is not included
✓ SystemCallFilter=~@obsolete                                 System call allow list defined for service, and @obsolete is not included
✗ RestrictAddressFamilies=~AF_NETLINK                         Service may allocate netlink sockets                                                0.1
✗ RootDirectory=/RootImage=                                   Service runs within the host's root directory                                       0.1
✗ SupplementaryGroups=                                        Service runs with supplementary groups                                              0.1
✓ CapabilityBoundingSet=~CAP_MAC_*                            Service cannot adjust SMACK MAC
✓ CapabilityBoundingSet=~CAP_SYS_BOOT                         Service cannot issue reboot()
✓ Delegate=                                                   Service does not maintain its own delegated control group subtree
✓ LockPersonality=                                            Service cannot change ABI personality
✓ MemoryDenyWriteExecute=                                     Service cannot create writable executable memory mappings
✓ RemoveIPC=                                                  Service user cannot leave SysV IPC objects around
✓ RestrictNamespaces=~CLONE_NEWUTS                            Service cannot create hostname namespaces
✓ UMask=                                                      Files created by service are accessible only by service's own user by default
✓ CapabilityBoundingSet=~CAP_LINUX_IMMUTABLE                  Service cannot mark files immutable
✓ CapabilityBoundingSet=~CAP_IPC_LOCK                         Service cannot lock memory into RAM
✓ CapabilityBoundingSet=~CAP_SYS_CHROOT                       Service cannot issue chroot()
✓ ProtectHostname=                                            Service cannot change system host/domainname
✓ CapabilityBoundingSet=~CAP_BLOCK_SUSPEND                    Service cannot establish wake locks
✓ CapabilityBoundingSet=~CAP_LEASE                            Service cannot create file leases
✓ CapabilityBoundingSet=~CAP_SYS_PACCT                        Service cannot use acct()
✓ CapabilityBoundingSet=~CAP_SYS_TTY_CONFIG                   Service cannot issue vhangup()
✓ CapabilityBoundingSet=~CAP_WAKE_ALARM                       Service cannot program timers that wake up the system
✓ RestrictAddressFamilies=~AF_UNIX                            Service cannot allocate local sockets
✓ ProcSubset=                                                 Service has no access to non-process /proc files (/proc subset=)

→ Overall exposure level for fan_remote.service: 0.7 SAFE 😀
```

Again, if you can figure out how to set a `/dev` ACL without breaking the use of
libusb to talk to the CM19A, please let me know.

## Reference Materials

If you'd like to make your own systemd services more secure, here are the
resources I used:

- [Writing a Secure Systemd Service with Sandboxing and Dynamic Users](https://nickb.dev/blog/writing-a-secure-systemd-service-with-sandboxing-and-dynamic-users)
- [security and hardening options for systemd service units](https://gist.github.com/ageis/f5595e59b1cddb1513d1b425a323db04)
- [Use TemporaryFileSystem to hide files or directories from systemd services](https://www.sherbers.de/use-temporaryfilesystem-to-hide-files-or-directories-from-systemd-services/)

I also mixed together these for how to do socket activation, since Rust services
start so quickly and systemd socket activation terminates the connection for
you, allowing the unit file to omit permission to use any network-related
syscalls:

- [The End of the Road: systemd’s “Socket” Units](https://www.linux.com/training-tutorials/end-road-systemds-socket-units/)
- [Systemd socket based activation](https://web.archive.org/web/20210617031521/https://leonardoce.wordpress.com/2015/03/08/systemd-socket-based-activation/)
- [The `listenfd` crate for Rust](https://lib.rs/crates/listenfd)

I've also since found these useful resources which may help with hardening
_other people's_ unit files:

- [How to harden a systemd service unit](https://linux-audit.com/systemd/how-to-harden-a-systemd-service-unit/)
- [Credentials - systemd.io](https://systemd.io/CREDENTIALS/)

...and, if you're a novice at or newcomer to writing unit files, you may find
this helpful:

- [Understanding Systemd Units and Unit Files](https://www.digitalocean.com/community/tutorials/understanding-systemd-units-and-unit-files)
