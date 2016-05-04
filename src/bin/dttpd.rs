#![crate_name="dttpd"]
#![crate_type="bin"]

// rustc feature enables
//#![feature(hash)]

// library imports
extern crate rand;
extern crate rustc_serialize as serialize;
//extern crate time;

// dttp project imports
extern crate dttp;

// library uses
use std::env;
use std::fs::File;
use std::io::{ BufReader, BufRead};
use std::net::SocketAddr;
use std::str::FromStr;

// dttp lib uses
use dttp::Auth;
use dttp::Datetime;
use dttp::Mote;
use dttp::Hub;
use dttp::crypto::*;
use dttp::crypto::keybase::*;

// entry function
fn main(){
	let mut args = env::args();
	let mut hostname = "localhost".to_string();
	let mut port = 8960;
	if args.len() > 1 {
		hostname = args.next().unwrap();}
	if args.len() > 2 {
		let port_arg = args.next().unwrap();
		let port_arg : Option<u16> = 
			u16::from_str_radix( port_arg.as_ref(), 10).ok();
		if let Some( port_arg) = port_arg {
			port = port_arg}}

	let mut hub = Hub::new( test_auth(), hostname, port);
	hub.say_hi();

	//add bootstrap remotes
	let bs_list = load_bootstrap_list( "config/dev.bs");
	println!( "bs list: {:?}", bs_list);
	for &bs in bs_list.iter() {
		hub.add_remote( bs);}

	let mut mote = test_mote();
	mote.dttpv = "0.1.0/test".to_string();
	let mote_hash : u64 = mote.hash_sip();
	println!( "generated test mote: {:016x} :: {}", mote_hash, mote);
	//println!( "mote json: {}", 
	//	serialize::json::encode( &mote.to_msg()));
	hub.add_mote( mote);

	// launch_hub
	//let _hub_clone = hub.clone();
	hub.launch();}

fn test_auth() -> Auth {
	Auth::new(
		Some( "kurotetsuka".to_string()),
		None,
		Some( "kurotetsuka@gmail.com".to_string()),
		Some( 0x0a1a20c0))}


fn test_mote() -> Mote {
	// create new auth
	let auth = test_auth();
	println!( "auth: {}", auth);

	// setup crypto
	let crypto : Box<CryptoProvider> = Box::new( KeybaseSession::new());

	// create mote
	let mut mote = Mote::new(
		auth,
		"test test :P".to_string(),
		Datetime::new( 1964, 256, 43200_000),
		"test test yo yo bro".to_string());
	crypto.sign( &mut mote).ok();

	return mote;}

fn load_bootstrap_list( filename: &str) -> Vec<SocketAddr> {
	// vars
	let mut result : Vec<SocketAddr> = Vec::new();

	// open the file
	let file = File::open( filename).unwrap();
	let reader = BufReader::new( file);

	// read each line
	for line in reader.lines() {
		if line.is_err() { break;}
		let line = line.unwrap();
		let line = line.trim();

		// ignore comments
		if line.starts_with( '#') { continue;}

		// parse address
		let addr : Option<SocketAddr> =
			SocketAddr::from_str( line).ok();
		if addr.is_none() { continue;}

		// push address
		result.push( addr.unwrap());}

	// done
	return result;}
