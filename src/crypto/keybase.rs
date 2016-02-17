// library uses
use std::io::Write;
use std::process::{ Command, Stdio};

// local uses
use crypto::CryptoProvider;
use mote::*;


pub struct KeybaseSession {
	pub user: String,
}

impl KeybaseSession {
	pub fn new() -> KeybaseSession {
		KeybaseSession { user: "kurotetsuka".to_string()}}
}

impl CryptoProvider for KeybaseSession {
	fn sign( &self, mote: &mut Mote) -> Result<(),String> {
		// vars
		let msg = &*mote.data;

		// set up sign proc
		let mut sign_cmd = Command::new( "keybase");
		sign_cmd.args( &[ "sign", "-d", "-m", msg]);
		// run sign proc
		let sign_result = sign_cmd.output();
		if sign_result.is_err() {
			return Err( "Failed to start keybase sign process".to_string());}
		let sign_result = sign_result.unwrap();

		// error check
		let sign_err_bytes = sign_result.stderr;
		let sign_err = String::from_utf8( sign_err_bytes.clone()).unwrap();
		if sign_err.contains( "ERRO") {
			return Err( sign_err);}
		// get result
		let sign_out_bytes = sign_result.stdout;
		let sign_out = String::from_utf8( sign_out_bytes.clone()).unwrap();
		mote.sig = sign_out;

		// return
		return Ok(());}

	fn verify( &self, mote: &Mote) -> Result<bool,String> {
		// vars
		let msg = &*mote.data;
		let sig = &mote.sig;

		// set up verify proc
		let mut verify_cmd = Command::new( "keybase");
		verify_cmd.args( &[ "verify", "-d", "/dev/stdin", "-m", msg]);
		verify_cmd.stdin( Stdio::piped());
		verify_cmd.stderr( Stdio::piped());
		verify_cmd.stdout( Stdio::piped());
		// run verify proc
		let mut verify_proc = verify_cmd.spawn().unwrap();
		if let Some( mut verify_in) = verify_proc.stdin.as_mut(){
			verify_in.write( sig.as_bytes()).ok();}
		else {
			return Err( "Failed to start keybase verify process".to_string());}
		let verify_result = verify_proc.wait_with_output().unwrap();

		// get result
		let verify_err_bytes = verify_result.stderr;
		let verify_err = String::from_utf8( verify_err_bytes.clone()).unwrap();

		// error check and return
		if verify_err.contains( "Signature verified") {
			return Ok( true);}
		else if verify_err.contains( "bad signature") {
			return Ok( false);}
		else {
			return Err( verify_err);}}

	fn encrypt( &self, _mote: &mut Mote) -> Result<(),String> {
		return Ok(());}

	fn decrypt( &self, _mote: &mut Mote) -> Result<(),String> {
		return Ok(());}
}
