#![crate_name="dttp"]
#![crate_type="lib"]

// rustc feature enables
#![feature(collections)]
#![feature(core)]

// library imports
#[macro_use]
extern crate arcmutex;
extern crate regex;
extern crate rustc_serialize;
extern crate rand;
extern crate time;

// reexports
pub use prelude::*;

// modules
pub mod auth;
pub mod consts;
pub mod dt;
pub mod hub;
pub mod key;
pub mod mote;
pub mod prelude;
pub mod protocol;

// tests
#[test]
fn it_works(){}
