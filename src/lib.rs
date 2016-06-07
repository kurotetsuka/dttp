#![crate_name="dttp"]
#![crate_type="lib"]

// features and plugins
#![feature( custom_derive)]
#![feature( plugin)]
#![plugin( peg_syntax_ext)]
#![plugin( serde_macros)]

// library imports
extern crate arcmutex;
extern crate gpgme;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
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
pub mod mote;
pub mod prelude;
pub mod protocol;
