# fetch
My info fetch program, written in Rust.

* [fetch](#fetch)
    * [Help Wanted](#help-wanted)
    * [Things to Know](#things-to-know)
    * [Requirements](#requirements)
    * [Installation](#installation)
    * [Screenshots](#screenshots)
    * [Amount of code. (According to tokei).](#amount-of-code-according-to-tokei)


### Help Wanted

1. I just found out that there is already a tool called "fetch" on BSD. So to avoid confusion, the name will be changed. To those of you reading this, what do you think about one of these names? "vfetch", "rsfetch", "fetch-rs", or "fetcher"? If none of those names sound good, reply with a name of your own you think would sound good.

2. I've been made aware of various ways to make my code better (thanks to various Reddit users). If possible, I would love some help in implementing them. Some of the things that were suggested were a bit confusing to be honest, but that might be because I've just started Rust with this project.

### Things to Know

1. The packages section only works for Arch (or Arch-based) distros as it uses `pacman`.

2. I will attempt to add support for the set terminal font, but it looks like a lot of confusing work to get done.

**WARNING**: I've updated a lot of things in this, so things are GUARANTEED to be broken for other people. Please report bugs you find.

### Requirements
`mpd + mpc` for the music info. Completely optional, as music info is turned off by default.

### Installation
I have prebuilt binaries in the releases tab for people who don't want to build from source, otherwise you can do this.

1. Install rust and cargo.
2. Clone the repository.
3. `cd fetch; make; sudo make install`
4. ~~To use, put the bash function above into "$HOME/.bashrc" (If you don't care about terminal detection, you can skip this step.)~~ Removed terminal detection for now. At least until I can figure out how to implement it in rust.

Uninstall with `sudo make uninstall`.

### Screenshots

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**My Preference of Options + Execution Time**
![Default](Screenshots/preference.png?raw=true "Default")

You can't see what options I choose because I aliased fetch. Here is the actual command ran.

`fetch -C 0 -h false -i false -l false -u false`

And yes, you saw right. Execution time was 0.015s! Crazy fast.

### Amount of code. (According to tokei).

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Makefile                1           67           44            9           14
 Rust                    1          391          369           13            9
-------------------------------------------------------------------------------
 Total                   2          458          413           22           23
-------------------------------------------------------------------------------
```
