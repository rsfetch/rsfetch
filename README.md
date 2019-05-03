# fetch
My info fetch program, written in Rust. Minimalistic (for the most part).

WARNING: I've updated a lot of things in this, so things are GUARANTEED to be broken for other people. Please report bugs you find.

Note: The packages section only works for Arch (or Arch-based) distros as it uses `pacman` and `paclist`.

Note 2: I will attempt to add support for the set terminal font, but it looks like a lot of confusing work to get done.

### Requirements
1. `pacman-contrib` for the separate package counts. Optional, spearate package counts are disabled by default.
2. `mpd + mpc` for the music info. Completely optional, as music info is turned off by default.

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

And yes, you saw right. Execution time was 0.021s! Crazy fast.

### Amount of code. (According to tokei).

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Makefile                1           69           46            9           14
 Rust                    1          370          348           13            9
-------------------------------------------------------------------------------
 Total                   2          439          394           22           23
-------------------------------------------------------------------------------
```
