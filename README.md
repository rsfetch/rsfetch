# fetch
My info fetch program, written in Rust. Minimalistic (for the most part).

Note: The packages section only works for Arch (or Arch-based) distros as it uses `pacman` and `paclist`.

Note 2: If you want the name of your terminal displayed, ![term](term) must be in the same directory as the fetch binary. Yes, I used a bash script, but I couldn't find a better way to do it. :(

Note 2.5: So I just noticed an odd bug with the terminal detection. You have to be in the same directory as both binaries. So I created a little bash function you can use in the meantime when I figure out how to fix it.

```
fetch () {
	pwd="$(pwd)"
	cd $HOME/.cargo/bin
	./fetch
	cd "$pwd"
}
```

Note 3: I will attempt to add support for the set terminal font, but it looks like a lot of confusing work to get done.

### Requirements
1. `lshw` for the GPU info.
2. `pacman-contrib` for the separate package counts. ~~Will add an option soon to turn those off if you don't want to install it.~~ Option has been added.

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**Default Fetch - No Caps**
![Default](Screenshots/default-nocaps.png?raw=true "Default")
