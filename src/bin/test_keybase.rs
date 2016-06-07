// library imports
extern crate dttp;
extern crate rustc_serialize as serialize;

// library uses
use std::process::exit;

// local uses
use dttp::*;
use dttp::crypto::*;
use dttp::crypto::keybase::*;

fn main(){
	// create mote
	let mut mote = Mote::new(
		Auth::new_test(),
		"test test.txt".to_string(),
		Datetime::now(),
		"test message, please ignore".to_string());

	// setup crypto
	let crypto : Box<CryptoProvider> = Box::new( KeybaseSession::new());
	let sign_result = crypto.sign( &mut mote);

	// error check
	if let Err( message) = sign_result {
		println!( "signing failed!:\n{}", message);
		exit( 1);}

	// print everything
	else {
		println!( "hash: {:x}", mote.hash_sip());
		println!( "mote:\n  {:?}\n", mote);
		println!( "mote json:\n{}",
			serialize::json::as_pretty_json( &mote.to_msg()));
		println!( "sig_check: {:?}", crypto.verify( &mote));}

	//exit
	exit( 0);
}
