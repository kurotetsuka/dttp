// library uses
//use std::fmt;
//use rand::Rng;

// modules
//pub mod gpg;
pub mod keybase;

// local uses
use mote::*;

pub trait CryptoProvider {
	fn sign( &self, mote: &mut Mote);
	fn verify( &self, mote: &Mote) -> bool;
	fn encrypt( &self, mote: &mut Mote);
	fn decrypt( &self, mote: &mut Mote);
}
