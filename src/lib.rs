#![crate_name="dttp"]
#![crate_type="lib"]

// features and plugins
#![feature(
	plugin,
	custom_derive,
	slice_patterns,
	advanced_slice_patterns)]
#![plugin(
	peg_syntax_ext)]

// library imports
extern crate arcmutex;
extern crate gpgme;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

// reexports
pub use prelude::*;

// modules
pub mod auth;
pub mod consts;
pub mod crypto;
pub mod dt;
pub mod hub;
pub mod mageon;
pub mod mote;
pub mod prelude;
pub mod protocol;
