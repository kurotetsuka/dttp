// library uses
use std::str::FromStr;
use serde_json::Value;

// re-exports
pub use self::grammar::ParseResult as ParseResult;
pub use self::grammar::ParseError as ParseError;

#[derive( Debug)]
pub struct Mageon {
	pub verb: String,
	pub args: Vec<MagArg>,
}

#[derive( Debug)]
pub enum MagArg {
	Str( String),
	Obj( Value),
	Vec( Vec< MagArg>),
}

peg! grammar( r#"
use super::*;
use super::grammar_err::*;
use serde_json::Value;

#[pub]
mageon -> Mageon
	= v:verb ":" WS a:args WS "." {
		Mageon { verb: v, args: a}}
	/ v:verb_solo {
		Mageon { verb: v, args: vec![]}}

verb -> String
	= s:$([a-zA-Z0-9_\-]+ [?!]?)
	{ s.to_string() }
verb_solo -> String
	= s:$([a-zA-Z0-9_\-]+ [?!])
	{ s.to_string() }
	/ s:$([a-zA-Z0-9_\-]+) "\."
	{ s.to_string() }

args -> Vec<MagArg>
	= arg ++ ( WS "," WS )
arg -> MagArg
	= s:arg_str { MagArg::Str( s)}
	/ o:arg_obj { MagArg::Obj( o)}
	/ v:arg_vec { MagArg::Vec( v)}

arg_str -> String
	= s:$([a-zA-Z0-9:_\-]+)
	{ s.to_string() }
	/ "\"" s:$( ( !( "\\\"" / "\"" ) . ) ** "\\\"" ) "\""
	{ // remove the quotes, preserve the rest 
		s.replace( "\\\"", "\"")}
arg_obj -> Value
	= s:$(json_obj)
	{?
		let result = s.parse::<Value>();
		if let Ok( obj) = result {
			Ok( obj)}
		else {
			Err( ERR_PARSE_JSON)}}
arg_vec -> Vec<MagArg>
	= "[" WS vec:args_maybe WS "]"
	{ vec }
args_maybe -> Vec<MagArg>
	= arg ** ( WS "," WS )

#[pub]
quot_str = "\"" ( !( "\\\"" / "\"" ) . ) ** "\\\"" "\""

WS = [ \t\r\n]*

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
"#);

mod grammar_err {
	pub static ERR_PARSE_JSON : &'static str = "json parse failed";
}

impl FromStr for Mageon {
	// todo: create custom ( more usable ) error type
	type Err = ParseError;
	fn from_str( string: &str) -> ParseResult<Mageon> {
		grammar::mageon( string)}
}
