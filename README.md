<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (0.01s - 0.2s execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents

* [rsfetch](#rsfetch)
	* [Help Wanted](#help-wanted)
	* [Things to Know](#things-to-know)
	* [Requirements](#requirements)
	* [Installation](#installation)
	* [Screenshots](#screenshots)
	* [Amount of code. (According to tokei).](#amount-of-code-according-to-tokei)
    * [License](#license)

**Why I made it:** I used to use neofetch, ufetch, and 
aurafetch a lot, but then that got me thinking. I have the skill to make
 my own script, so why not? At first, it was just a simple BASH script. 
Then I decided I wanted to try my hand at Rust, as it has interested me 
for a while. So... here's the end result. I am open to any criticisms 
you have. After all, I wish to make this better (as well as improve my 
Rust skills).

### Help Wanted

I've been made aware of various ways to make my code better (thanks to various Reddit users). If possible, I would love some help in implementing them. Some of the things that were suggested were a bit confusing to be honest, but that might be because I've just started Rust with this project. If you want to help, let me know via; [Reddit](https://www.reddit.com/user/Valley6660) (please send a PM and not start a chat as Reddit Chat is broken for me), a Github issue, or a pull request. I respond usually pretty quickly (within 1-10 minutes). If I don't respond, it's because I probably crashed from sleep deprivation.

### Things to Know

1. The packages section only works for Arch (or Arch-based) distros as it uses `pacman`.

2. I will attempt to add support for the set terminal font, but it looks like a lot of confusing work to get done.

3. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community.

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

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (builds from source)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (grabs binary from releases)

### Screenshots

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**My Preference of Options + Execution Time**
![Default](Screenshots/preference.png?raw=true "Default")

You can't see what options I choose because I aliased fetch. Here is the actual command ran.

`rsfetch -C 0 -h false -i false -l false -u false`

And yes, you saw right. Execution time was 0.012s! Crazy fast.

### Amount of code. (According to tokei).

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Makefile                1           67           44            9           14
 Rust                    1          416          397            8           11
-------------------------------------------------------------------------------
 Total                   2          483          441           17           25
-------------------------------------------------------------------------------
```

### License

I have set my repo to "The Unlicense". Anyone can do anything they want, with anything in my repo.
