# Guessing Game
A Game based off of the Rust Book's *Guessing Game* Project, adapted as a GUI Interface. This was mainly an exercise to try out Rust's Egui. Game Logic was mostly handwritten, although the styling was assisted by Claude. This was mainly written in Windows (Because my dumb chuddy WSL Doesn't support graphical interfaces) but *presumably* it can also run on Linux Distrubtions as well.

You guess a number between **1 to 100**. Any other number will be invalid.

# Running the Game
1. Fork the Git repo and put it into your file system.
2. In a command terminal, enter `cargo release --build` and enter `cargo run --release` to run the game.

Space requirements for the file itself: 2.24 GB (Why is it so large?). The exe file itself: 12.4 MB. I have only tested this in my system, so I doubt this will work; good luck!

# Commentary
Coding in Claude is fun! It auto-completes most of the logic I can't be bothered to write on my own and helps me learn along the way, especially how scant documentation in Egui unfortunately is. Although in the future, I'll try and rely less on Claude to try and figure out how to style and center my objects correctly, moreso referencing this game instead. But to learn how to use AI will probably be common place in any job (assuming that I get one!)

But, the need to code by hand will always be present, in order to correct Claude (especially the free version) in its mistakes. For example, I had to continuously correct Cluade in its usage of outdated functions.