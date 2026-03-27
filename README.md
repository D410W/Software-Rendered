# SoftRendered - A software rendered game engine in Rust

This is a personal project of mine with the purpose of getting experience on game engine architecture and Rust projects.
It is also inspired by [Tsodin's](https://github.com/tsoding) series on [software rendering](https://github.com/tsoding).

## Current state

I'm still working on adding new features and optimizing the pipeline. The current structure uses a struct that implements the GameState trait to separate the engine and to process the game logic.

Example of a working game can be found in the src/main.rs and src/gamelogic/core.rs files.
(Obs.: dennis_uncompressed.dds isn't uploaded due to it's size. You can either create the uncompressed dds file or comment out the load_texture_dds and use load_model_obj.)
