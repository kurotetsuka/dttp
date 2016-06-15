// library uses
use std::str::FromStr;
use serde_json::Value;

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

peg! mag_grammar( r#"
	use super::*;
	use super::mag_grammar_err::*;
	use serde_json;
	use serde_json::Value;

	#[pub]
	mageon -> Mageon
		= v:verb ":" WS a:args WS "." {
			Mageon { verb: v, args: a}}
		/ v:verb_solo {
			Mageon { verb: v, args: vec![]}}

	verb -> String
		= [a-zA-Z0-9_\-]+ [?!]? {
			match_str.to_string()}
	verb_solo -> String
		= [a-zA-Z0-9_\-]+ [?!] {
			match_str.to_string()}
		/ [a-zA-Z0-9_\-]+ "\." {
			match_str[ 0..match_str.len()-1].to_string()}

	args -> Vec<MagArg>
		= arg ++ ( WS "," WS )
	arg -> MagArg
		= s:arg_str { MagArg::Str( s)}
		/ o:arg_obj { MagArg::Obj( o)}
		/ v:arg_vec { MagArg::Vec( v)}

	arg_str -> String
		= [a-zA-Z0-9:_\-]+ {
			match_str.to_string() }
		/ quot_str
	arg_obj -> Value
		= json_obj {?
			let result : serde_json::Result<Value> = match_str.parse();
			if let Ok( obj) = result {
				Ok( obj)}
			else { Err( ERR_PARSE_JSON)} }
	arg_vec -> Vec<MagArg>
		= "[" WS vec:args_maybe WS "]" {
			vec }
	args_maybe -> Vec<MagArg>
		= arg ** ( WS "," WS )

	quot_str -> String
		= "\"" ( [^\\\"]+ ) ++ "\\\"" "\"" {
			// remove the quotes, preserve the rest 
			match_str[ 1..match_str.len()-1].replace( "\\\"", "\"")}

	WS = [ \t\r\n]*

	// serde will check json properly when parsing, so we only need
	// check enough to guarantee we arent interrupting a well-formed mageon
	json =
		json_str /
		json_obj /
		json_vec /
		json_other
	json_str = "\"" ( [^\\\"]+ ) ++ "\\\"" "\""
	json_obj = "{" WS json_obj_entry ++ ( WS "," WS ) WS "}"
	json_obj_entry = quot_str WS ":" WS json
	json_vec = "[" WS json WS ( "," json )* ","? "]"
	json_other = [a-zA-Z0-9+\-_\.]+
"#);

mod mag_grammar_err {
	pub static ERR_PARSE_JSON : &'static str = "json parse failed";
}

impl FromStr for Mageon {
	// todo: create custom ( more usable ) error type
	type Err = mag_grammar::ParseError;
	fn from_str( string: &str) ->
			Result<Mageon, mag_grammar::ParseError> {
		mag_grammar::mageon( string)}
}
