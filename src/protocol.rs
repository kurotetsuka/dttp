// library uses
use std::fmt;
use std::str::FromStr;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use regex::Regex;

// local uses
use mageon::*;
use mote::spec::*;
use self::Command::*;
use self::Response::*;

pub enum Command {
	Version( String),
	Opt( Json),
	HaveDec( MoteSpec),
	HaveReq( MoteSpec),
	Get( MoteSpec),
	Fetch( MoteSpec),
	Want( MoteSpec),
	Take( Json),
	SelfDec( String),
	OthersReq,
	Profile( String),
}

pub enum CommandParseError {
	GrammarError,
	InvalidArgsError,
	UnknownError,
}

pub enum Response {
	Okay,
	OkayResult( Json),
	Affirm,
	AffirmResult( Json),
	Deny,
	Error( String, String),
}

pub enum ResponseParseErr {
	GrammarError,
	InvalidArgsError,
	UnknownError,
}

impl FromStr for Command {
	// todo: create custom ( more usable ) error type
	type Err = CommandParseError;
	fn from_str( string: &str) -> Result<Command, CommandParseError> {
		let mageon = string.parse::<Mageon>();
		if mageon.is_err() {
			return Err( CommandParseError::GrammarError);}
		let mageon = mageon.unwrap();

		let cmd = mageon.verb.clone();
		match cmd.as_ref() {
			"get?" => Command::parse_get( mageon),
			_ => Err( CommandParseError::UnknownError) }}
}

impl Command {
	fn parse_get( mag: Mageon) -> Result<Command, CommandParseError> {
		if mag.args.len() != 1 {
			return Err( CommandParseError::GrammarError);}
		if let MagArg::Str( ref spec_str) = mag.args[0] {
			let spec = spec_str.parse::<MoteSpec>();
			if let Ok( spec) = spec {
				return Ok( Get( spec))}
			else {
				return Err( CommandParseError::InvalidArgsError);}}
		else {
			return Err( CommandParseError::GrammarError);}

		Err( CommandParseError::UnknownError) }
	/*fn parse_have( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }
	fn parse_have_req( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }
	fn parse_hello( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }
	fn parse_others( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }
	fn parse_take( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }
	fn parse_want( _string: &str) -> Result<Command, CommandParseError> {
		Err( CommandParseError::UnknownError) }*/
}
impl fmt::Display for Command {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Version( ref string) =>
				write!( formatter, "dttpv: {}", string),
			/*&Opt( ref json) =>
				write!( formatter, "{}", *json),
			&HaveDec( ref hash) =>
				write!( formatter, "have:{:016x}.", *hash),
			&HaveReq( ref hash) =>
				write!( formatter, "have?:{:016x}.", *hash),
			&Get( ref hash) =>
				write!( formatter, "get?:{:016x}.", *hash),
			&Fetch( ref hash) =>
				write!( formatter, "fetch?:{:016x}.", *hash),
			&OthersReq =>
				write!( formatter, "others?"),
			&Profile( ref string) =>
				write!( formatter, "{:016x}", *hash),
			&WantReq( ref hash) =>
				write!( formatter, "want?:{:016x}.", *hash),
			&SelfDec( ref hostname) =>
				write!( formatter, "self:{}.", *hostname),
			&Take( ref data) =>
				write!( formatter, "take:{}.",
					json::encode( data).unwrap()),*/
			_ => write!( formatter, "asdf"),}}
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
			let regex = Regex::new( r"err:(.+), (.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse message
			let code = cap.at( 1);
			if code.is_none() { return None;}
			let code = code.unwrap().to_string();
			let message = cap.at( 1);
			if message.is_none() { return None;}
			let message = message.unwrap().to_string();
			return Some( Error( code, message));}

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
			&AffirmResult( ref data) =>
				write!( formatter, "yes:{}.",
					json::encode( data).unwrap()),
			&Deny =>
				write!( formatter, "no."),
			&Error( ref code, ref message) =>
				write!( formatter, "err:{}, {}.", code, message),}}
}
