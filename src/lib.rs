//! MINAUTOMATA - a "falling sand" style game written in rust for the web.
//! 
//! MINAUTOMATA is compiled to web assembly (wasm) using wasm-pack. This
//! compiles the rust to wasm, and generates the javascript interface necessary
//! for running in a browser.
//! 
//! The game is based on the idea of "cellular automata" - inspired by the 
//! Noita developers [talk at GDC 2019](https://www.youtube.com/watch?v=prXuyMCgbTc).
//! Each cell - or particle - has a set of rules that it must follow. For
//! example: a particle of sand falls down if it can so one of its rules is to
//! see if there's space directly below and if there is, move into it.
//! 

pub mod actions;
pub mod colour;
pub mod game;
pub mod kinds;
pub mod log;
pub mod point;
pub mod particles;