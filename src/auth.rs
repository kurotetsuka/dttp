// library uses
use std::fmt;
use std::clone;
use std::str::FromStr;

// re-exports
pub use self::grammar::ParseResult as ParseResult;
pub use self::grammar::ParseError as ParseError;

/// class that defines the signing agent of a mote
#[derive( Hash, PartialEq, Eq)]
pub struct Auth {
	pub user: Option<String>,
	pub comment: Option<String>,
	pub addr: Option<String>,
	pub id: Option<u32>,
}
impl Auth {
	pub fn null() -> Auth {
		Auth {
			user: None,
			comment: None,
			addr: None,
			id: None,
		}}

	pub fn new(
			user: Option<String>, comment: Option<String>,
			addr: Option<String>, id: Option<u32>) -> Auth {
		Auth {
			user: user,
			comment: comment,
			addr: addr,
			id: id,
		}}

	pub fn new_test() -> Auth {
		Auth {
			user: Some( "kurotetsuka".to_string()),
			comment: None,
			addr: Some( "kurotetsuka@gmail.com".to_string()),
			id: Some( 0x0a1a20c0),
		}}
}

pub enum AuthParseError {
	GrammarError,
	UnknownError,
}

peg! grammar( r#"
use super::*;
use super::gammar_err::*;

#[pub]
auth -> Auth
	= user:(user?) comment:(comment?) addr:(addr?) id:(id?) {
		Auth::new( user, comment, addr, id)}

// idk the proper pgp grammar, so i'm just guessing for now
// todo: look that up, aha

user -> String
	= text
comment -> String 
	= "(" t:text ")" { t}
addr -> String
	= "<" s:$(chars "@" chars) ">"
	{ s.to_string() }
id -> u32
	= "[" s:$(hex_chars*<8>) "]"
	{?
		if let Ok( num) = s.parse::<u32>() {
			Ok( num)}
		else {
			Err( ERR_PARSE_NUM)}}

text -> String
	= s:$([a-zA-Z0-9 ]+)
	{ s.to_string()}
chars = [a-zA-Z0-9]+
hex_chars = [0-9a-f]

WS = [ ]*
"#);

mod gammar_err {
	pub static ERR_PARSE_NUM : &'static str = "u32 parse failed";
}

impl FromStr for Auth {
	// todo: create custom ( more usable ) error type
	type Err = ParseError;
	fn from_str( string: &str) -> ParseResult<Auth> {
		grammar::auth( string)}
}

impl fmt::Display for Auth {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let self_tuple = (
			self.user.as_ref(), self.comment.as_ref(),
			self.addr.as_ref(), self.id.as_ref());
		match self_tuple {
			( Some( user), Some( comment), Some( addr), Some( &id)) =>
				write!( formatter,
					"{} ({}) <{}> [{:08x}]",
					user, comment, addr, id),
			( Some( user), Some( comment), Some( addr), None) =>
				write!( formatter,
					"{} ({}) <{}>",
					user, comment, addr),
			( Some( user), Some( comment), None, Some( &id)) =>
				write!( formatter,
					"{} ({}) [{:08x}]",
					user, comment, id),
			( Some( user), Some( comment), None, None) =>
				write!( formatter,
					"{} ({})",
					user, comment),
			( Some( user), None, Some( addr), Some( &id)) =>
				write!( formatter,
					"{} <{}> [{:08x}]",
					user, addr, id),
			( Some( user), None, Some( addr), None) =>
				write!( formatter,
					"{} <{}>",
					user, addr),
			( Some( user), None, None, Some( &id)) =>
				write!( formatter,
					"{} [{:08x}]",
					user, id),
			( Some( user), None, None, None) =>
				write!( formatter,
					"{}",
					user),
			( None, Some( comment), Some( addr), Some( &id)) =>
				write!( formatter,
					"({}) <{}> [{:08x}]",
					comment, addr, id),
			( None, Some( comment), Some( addr), None) =>
				write!( formatter,
					"({}) <{}>",
					comment, addr),
			( None, Some( comment), None, Some( &id)) =>
				write!( formatter,
					"({}) [{:08x}]",
					comment, id),
			( None, Some( comment), None, None) =>
				write!( formatter,
					"({})",
					comment),
			( None, None, Some( addr), Some( &id)) =>
				write!( formatter,
					"<{}> [{:08x}]",
					addr, id),
			( None, None, Some( addr), None) =>
				write!( formatter,
					"<{}>",
					addr),
			( None, None, None, Some( &id)) =>
				write!( formatter,
					"[{:08x}]",
					id),
			( None, None, None, None) =>
				write!( formatter, ""),}}
}
impl fmt::Debug for Auth {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "[{}]", self)}
}

impl clone::Clone for Auth {
	fn clone( &self) -> Auth {
		Auth {
			user: self.user.clone(),
			comment: self.comment.clone(),
			addr: self.addr.clone(),
			id: self.id.clone(),
		}}

	fn clone_from( &mut self, source: &Auth){
		self.user = source.user.clone();
		self.comment = source.comment.clone();
		self.addr = source.addr.clone();
		self.id = source.id.clone();}
}
