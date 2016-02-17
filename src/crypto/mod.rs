// library uses
//use std::fmt;
//use rand::Rng;

// modules
//pub mod gpg;
pub mod keybase;

// local uses
use mote::*;

pub trait CryptoProvider {
	fn sign( &self, mote: &mut Mote) -> Result<(),String>;
	fn verify( &self, mote: &Mote) -> Result<bool,String>;
	fn encrypt( &self, mote: &mut Mote) -> Result<(),String>;
	fn decrypt( &self, mote: &mut Mote) -> Result<(),String>;
}
