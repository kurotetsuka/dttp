// library uses
use std::fmt;
use std::str::FromStr;

use serde_json;
use serde_json::Value;

// local uses
use mote::spec::*;
use self::Command::*;
use self::Response::*;

// re-exports
pub use self::grammar::ParseResult as ParseResult;
pub use self::grammar::ParseError as ParseError;

pub enum Command {
	Version( String),
	Opt( Value),
	HaveDec( MoteSpec),
	HaveReq( MoteSpec),
	Get( MoteSpec),
	Fetch( MoteSpec),
	Want( MoteSpec),
	Take( Value),
	SelfDec( String),
	OthersReq,
	Profile( String),
}

pub enum CommandParseError {
	Grammar( ParseError),
	InvalidCmd,
	InvalidArgs,
	Unknown,
}

pub enum Response {
	Okay,
	OkayResult( Value),
	Affirm,
	AffirmResult( Value),
	Deny,
	Error( String, String),
}

pub enum ResponseParseError {
	Grammar( ParseError),
	InvalidResp,
	InvalidArgs,
	Unknown,
}

pub type CommandParseResult = Result<Command,CommandParseError>;
pub type ResponseParseResult = Result<Response,ResponseParseError>;

peg! grammar(r#"
use super::*;
use super::grammar_err::*;
use serde_json::Value;

// commands
#[pub]
command -> Command
	= version
	/ opt
	/ have_dec
	/ have_req
	/ get
	/ fetch
	/ want
	/ take
	/ self_dec
	/ others_req
	/ profile

version -> Command
	= "dttpv:" arg:arg_str "."
	{ Command::Version( arg) }
opt -> Command
	= "opt:" arg:arg_json "."
	{ Command::Opt( arg) }
have_dec -> Command
	= "have:" arg:mote_spec "."
	{ Command::HaveDec( arg) }
have_req -> Command
	= "have?:" arg:mote_spec "."
	{ Command::HaveReq( arg) }
get -> Command
	= "get:" arg:mote_spec "."
	{ Command::Get( arg) }
fetch -> Command
	= "fetch:" arg:mote_spec "."
	{ Command::Fetch( arg) }
want -> Command
	= "want?:" arg:mote_spec "."
	{ Command::Want( arg) }
take -> Command
	= "take:" arg:arg_json "."
	{ Command::Take( arg) }
self_dec -> Command
	= "self:" arg:arg_str "."
	{ Command::SelfDec( arg) }
others_req -> Command
	= "others?"
	{ Command::OthersReq }
profile -> Command
	= "profile:" arg:arg_str "."
	{ Command::Profile( arg) }

// responses
#[pub]
response -> Response
	= okay
	/ okay_result
	/ affirm
	/ affirm_result
	/ deny
	/ error

okay -> Response
	= "ok."
	{ Okay }
okay_result -> Response
	= "ok:" arg:arg_json "."
	{ OkayResult( arg) }
affirm -> Response
	= "yes:" arg:arg_json "."
	{ Affirm }
affirm_result -> Response
	= "yes:" arg:arg_json "."
	{ AffirmResult( arg) }
deny -> Response
	= "no:" arg:arg_json "."
	{ Deny }
error -> Response
	= "error:" arg0:arg_str "," arg1:arg_str "."
	{ Error( arg0, arg1) }


// args

mote_spec -> MoteSpec
	= s:$(hash)
	{ s.parse().unwrap() }

arg_json -> Value
	= s:$(json_obj)
	{?
		let result = s.parse::<Value>();
		if let Ok( obj) = result {
			Ok( obj)}
		else {
			Err( ERR_INVALID_JSON)}}
arg_str -> String
	= s:$([a-zA-Z0-9:_\-]+)
	{ s.to_string() }
	/ "\"" s:$( ( !( "\\\"" / "\"" ) . ) ** "\\\"" ) "\""
	{ // remove the quotes, preserve the rest 
		s.replace( "\\\"", "\"")}
quot_str = "\"" ( !( "\\\"" / "\"" ) . ) ** "\\\"" "\""

hash -> Vec<u8> = hex_byte+
hex_byte -> u8 = s:$([0-9a-f]*<2>) { s.parse().unwrap()}

// serde will check json properly when parsing, so we only need
// check enough to guarantee we aren't interrupting a well-formed mageon
json =
	json_str /
	json_obj /
	json_vec /
	json_other
json_str = quot_str
json_obj = "{" WS json_obj_entry ++ ( WS "," WS ) WS "}"
json_obj_entry = quot_str WS ":" WS json
json_vec = "[" WS json WS ( "," json )* ","? "]"
json_other = [a-zA-Z0-9+\-_\.]+


WS = [ \t\r\n]*
"#);

mod grammar_err {
	//pub static ERR_INVALID_ARGS : &'static str = "err_invalid_args";
	pub static ERR_INVALID_JSON : &'static str = "err_invalid_json";
	//pub static ERR_INVALID_VERB : &'static str = "err_invalid_verb";
	//pub static ERR_UNIMPLEMENTED : &'static str = "err_unimplemented";
	//pub static ERR_UNKNOWN : &'static str = "err_unknown";
	//pub static ERR_INVALID : &'static str = "err_invalid";
}

impl FromStr for Command {
	type Err = CommandParseError;
	fn from_str( string: &str) -> CommandParseResult {
		match grammar::command( string) {
			Ok( command) => Ok( command),
			Err( err) => Err( CommandParseError::Grammar( err))}}
}

impl Command {}
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
					serde_json::to_string( &data).unwrap()),*/
			_ => write!( formatter, "asdf"),}}
}


impl Response {
	pub fn from_str( string: &str) -> ResponseParseResult {
		match grammar::response( string) {
			Ok( response) => Ok( response),
			Err( err) => Err( ResponseParseError::Grammar( err))}}
}
impl fmt::Display for Response {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Okay =>
				write!( formatter, "ok."),
			&OkayResult( ref data) =>
				write!( formatter, "ok:{}.",
					serde_json::to_string( &data).unwrap()),
			&Affirm =>
				write!( formatter, "yes."),
			&AffirmResult( ref data) =>
				write!( formatter, "yes:{}.",
					serde_json::to_string( &data).unwrap()),
			&Deny =>
				write!( formatter, "no."),
			&Error( ref code, ref message) =>
				write!( formatter, "err:{}, {}.", code, message),}}
}
