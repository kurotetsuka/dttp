// library uses
use std::fmt;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use regex::Regex;

// local uses
use self::Command::*;
use self::Response::*;


pub enum MoteSpec {
	ShortHash( u8),
	LongHash( u64),
	Meta( String),
	Auth( String),
}

pub enum Command {
	Hello( String),
	HaveDec( u64),
	HaveReq( u64),
	Get( u64),
	Fetch( u64),
	OthersReq,
	WantReq( u64),
	Take( Json),
}

pub enum Response {
	Okay,
	OkayResult( Json),
	Affirm,
	Deny,
	Error,
	ErrorMsg( String),
}

impl Command {
	pub fn from_str( string: &str) -> Option<Command> {
		// match get request
		let cmd = "get?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"get\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			if hash_str.is_none() { return None;}
			let hash_str = hash_str.unwrap();
			let hash : Option<u64> = 
				u64::from_str_radix( hash_str, 16).ok();
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( Get( hash));}

		// match have command
		let cmd = "have:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"have:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			if hash_str.is_none() { return None;}
			let hash_str = hash_str.unwrap();
			let hash : Option<u64> = 
				u64::from_str_radix( hash_str, 16).ok();
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( HaveDec( hash));}

		// match have request
		let cmd = "have?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"have\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			if hash_str.is_none() { return None;}
			let hash_str = hash_str.unwrap();
			let hash : Option<u64> = 
				u64::from_str_radix( hash_str, 16).ok();
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( HaveReq( hash));}

		// match hello command
		let cmd = "self:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"self:(.+:.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hostname
			let hostname = cap.at( 1);
			if hostname.is_none() { return None;}
			let hostname = hostname.unwrap().to_string();
			// return
			return Some( Hello( hostname));}

		// match others request
		let cmd = "others?";
		if string.eq( cmd) {
			return Some( OthersReq);}

		// match take command
		let cmd = "take:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"take:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let json_str = cap.at( 1);
			if json_str.is_none() { return None;}
			let json_str = json_str.unwrap();
			let json : Option<Json> = 
				Json::from_str( json_str).ok();
			if json.is_none() { return None;}
			let json = json.unwrap();
			// return
			return Some( Take( json));}

		// match want request
		let cmd = "want?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"want\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			if hash_str.is_none() { return None;}
			let hash_str = hash_str.unwrap();
			let hash : Option<u64> = 
				u64::from_str_radix( hash_str, 16).ok();
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( WantReq( hash));}

		// fallback
		return None;}
}
impl fmt::Display for Command {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Fetch( ref hash) =>
				write!( formatter, "fetch?:{:016x}.", *hash),
			&Get( ref hash) =>
				write!( formatter, "get?:{:016x}.", *hash),
			&HaveDec( ref hash) =>
				write!( formatter, "have:{:016x}.", *hash),
			&HaveReq( ref hash) =>
				write!( formatter, "have?:{:016x}.", *hash),
			&Hello( ref hostname) =>
				write!( formatter, "self:{}.", *hostname),
			&OthersReq =>
				write!( formatter, "others?"),
			&Take( ref data) =>
				write!( formatter, "take:{}.",
					json::encode( data).unwrap()),
			&WantReq( ref hash) =>
				write!( formatter, "want?:{:016x}.", *hash),}}
}


impl Response {
	pub fn from_str( string: &str) -> Option<Response> {
		// match okay response
		let res = "ok.";
		if string.eq( res) {
			return Some( Okay);}

		// match affirm response
		let res = "yes.";
		if string.eq( res) {
			return Some( Affirm);}

		// match deny response
		let res = "no.";
		if string.eq( res) {
			return Some( Deny);}

		// match error response
		let res = "err.";
		if string.eq( res) {
			return Some( Error);}

		// match okay result response
		let res = "ok:";
		if string.starts_with( res) {
			let regex = Regex::new( r"ok:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse json
			let json_str = cap.at( 1);
			if json_str.is_none() { return None;}
			let json_str = json_str.unwrap();
			let json : Option<Json> = 
				Json::from_str( json_str).ok();
			if json.is_none() { return None;}
			let json = json.unwrap();
			// return
			return Some( OkayResult( json));}

		// match error message response
		let res = "err:";
		if string.starts_with( res) {
			let regex = Regex::new( r"err:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse message
			let message = cap.at( 1);
			if message.is_none() { return None;}
			let message = message.unwrap().to_string();
			return Some( ErrorMsg( message));}

		// fallback
		return None;}
}
impl fmt::Display for Response {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Okay =>
				write!( formatter, "ok."),
			&OkayResult( ref data) =>
				write!( formatter, "ok:{}.",
					json::encode( data).unwrap()),
			&Affirm =>
				write!( formatter, "yes."),
			&Deny =>
				write!( formatter, "no."),
			&Error =>
				write!( formatter, "err."),
			&ErrorMsg( ref message) =>
				write!( formatter, "err:{}.", message),}}
}
