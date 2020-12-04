# Josh Olds - Advent of Code 2020

I am solving this year's AoC challenges in Rust.

I have been wanting to learn Rust for some time, and this seemed like the perfect opportunity! If you would like to follow along or try out any of my examples, follow the instructions below!

---

## Linux vs. Windows

Uh oh! I noticed that the puzzle inputs read into Rust differently between Windows and Linux... Linux seems to have no trailing carriage returns, when Windows reads in a **'\r'** for all new lines. This being said, some puzzles may not work on Windows! I'll be attempting to fix this to be portable soon! 

## How to Run

- Install [Rust](https://www.rust-lang.org/tools/install)
  - The installer is super simple and very quick to setup on any platform! 
- Ensure Rust is patched and up to date
  - ```rustup update```
- Clone my repo:
  - ```git clone https://github.com/JoshOlds/Advent_of_Code_2020.git```
- Browse into the **dayX** folder of the day's code you would like to build and run.
- To build and execute the code:
  - ```cargo run```
- If you are interested in raw performance, be sure to build a release executable
  - ```cargo run --release```

And that should be all there is to it! Please let me know if anything fails to run for you.

---

## How to Set up a Rust IDE

If you are interested in diving deeper into the code, the following steps will get your IDE set up like mine.

I use Visual Studio Code, with the **rust-analyzer** extension (note, not the official Rust extension).

**rust-analyzer** should provide intellisense, code completion, and error highlighting in all Rust files. Note, for this extension you may need to enable the **all cargo features** setting in the extension's settings.

For debug, you need to install the **CodeLLDB** extension. LLDB has plugins to support Rust, and will give you user-friendly debug information for complex types and collections.

I have included my **.vscode** folder in this repo. This includes a **launch.json** file, with example debug launch settings that allow you to debug launch a Rust file with LLDB. Be sure to enable the **breakpoints in any file** setting in VSCode's global preferences.