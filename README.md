# roguelike-rs

A nethack-inspired curses-based roguelike.

<img src="https://imgur.com/j3Fvc0u.png" align="center" />
How the game looks like.

## Controls
The same as nethack, except for exiting.

`q` quits

`,` picks item up

`.` waits

`a` applies (uses) an item

`d` drops an item

`>` uses the stairs

`012` chooses level up bonus

`hjkl`/arrow keys moves you around

## Supported platforms

Should compile on every platform LLVM supports, since it is written in Rust.

## Known bugs

There is a bug in the dungeon generating algorithm that lets closed rooms to
appear, causing unfinishable levels.

## Inspiration:
https://sites.google.com/site/jicenospam/visibilitydetermination

https://tomassedovic.github.io/roguelike-tutorial/index.html

Thank you for your time and patience!
