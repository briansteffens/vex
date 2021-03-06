vex: Vertical EXpansion
=======================

With shell interpreters you can expand arguments horizontally, for example:

```bash
ls /{usr,bin}/
# becomes..
ls /usr/ /bin/
```

With vex you can do something similar, but vertically:

```bash
vex "ls /[usr,bin]/"
# becomes..
ls /usr/
ls /bin/
```

The above is a trivial example. A better one might be:

```bash
vex "iptables -A [INPUT,OUTPUT] -p tcp --dport [80,443] -j ACCEPT"
# becomes..
iptables -A INPUT -p tcp --dport 80 -j ACCEPT
iptables -A INPUT -p tcp --dport 443 -j ACCEPT
iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT
iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT
```

There is also a sequential style. Instead of calculating all possible
permutations, you could do something like the following to open SSH incoming
and DNS outgoing on different protocols:

```bash
vex --seq "iptables -A [INPUT,OUTPUT] -p [tcp,udp] --dport [22,53] -j ACCEPT"
# becomes..
iptables -A INPUT -p tcp --dport 22 -j ACCEPT
iptables -A OUTPUT -p udp --dport 53 -j ACCEPT
```

# Packages

This program is on the AUR at https://aur.archlinux.org/packages/vex/

# Compiling from source

You'll need git to download the source and rust and cargo to compile. On Arch:

```bash
$ sudo pacman -S git rust cargo
```

On Debian/Ubuntu/Mint/etc:

```bash
$ sudo apt-get install git rust cargo
```

Download the source repository:

```bash
$ git clone https://github.com/briansteffens/vex
$ cd vex
```

Compile:

```bash
$ cargo build --release
```

Install:

```bash
$ sudo ln -s $PWD/target/release/vex /usr/local/bin/vex
```

# Usage

Enable DNS traffic both ways over TCP and UDP:

```bash
vex "iptables -A [INPUT,OUTPUT] -p [tcp,udp] --dport 53 -j ACCEPT"
```

View generated commands without executing them:

```bash
vex --dry "ls /[usr,bin,var]/"
```

Use custom control characters:

```bash
vex --start="<" --stop=">" --sep="|" "ls /<usr|bin|var>/"
```
