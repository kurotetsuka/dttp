// library imports
extern crate dttp;

// library uses

// local uses
use dttp::*;
use dttp::crypto::*;
use dttp::crypto::keybase::*;

fn main(){
	let mut mote = Mote::new(
		Auth::new_test(),
		"test test.txt".to_string(),
		Datetime::now(),
		"test message, please ignore".to_string());
	let crypto : Box<CryptoProvider> = Box::new( KeybaseSession::new());
	crypto.sign( &mut mote).ok();
	println!( "hash: {:x}", mote.hash_sip());
	println!( "mote: {:?}", mote);
	println!( "sig_check: {}", crypto.verify( &mote).unwrap());
}
