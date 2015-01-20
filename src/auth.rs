// library uses
use std::num;
use std::fmt;
use std::clone;
use regex::Regex;

// local uses

/// class that defines the signing agent of a mote
#[derive( Hash)]
pub struct Auth {
	pub user: Option<String>,
	pub comment: Option<String>,
	pub email: Option<String>,
	pub id: Option<u32>,
}
impl Auth {
	pub fn null() -> Auth {
		Auth {
			user: None,
			comment: None,
			email: None,
			id: None,
		}}

	pub fn new(
			user: Option<String>, comment: Option<String>,
			email: Option<String>, id: Option<u32>) -> Auth {
		Auth {
			user: user,
			comment: comment,
			email: email,
			id: id,
		}}

	pub fn new_test() -> Auth {
		Auth {
			user: Some( "kurotetsuka".to_string()),
			comment: None,
			email: Some( "kurotetsuka@gmail.com".to_string()),
			id: Some( 0x0a1a20c0),
		}}

	// this needs to be fixed to accept all possible auth strings
	pub fn from_str( string: &str) -> Option<Auth> {
		// regex with user, email, and key
		let regex = Regex::new(
			r"([:ascii:]+) <(\S+@\S+)> :: ([:xdigit:]{8})");
		if regex.is_err() { return None;}
		let regex = regex.unwrap();

		// get captures
		let cap = regex.captures( string);
		if cap.is_none() { return None;}
		let cap = cap.unwrap();
		if cap.len() < 4 { return None;}

		// parse user
		let user = cap.at( 1);
		if user.is_none() { return None;}
		let user = user.unwrap().to_string();
		// parse email
		let email = cap.at( 2);
		if email.is_none() { return None;}
		let email = email.unwrap().to_string();
		// parse id
		let id_str = cap.at( 3);
		if id_str.is_none() { return None;}
		let id_str = id_str.unwrap();
		let id : Option<u32> =
			num::from_str_radix( id_str, 16);
		if id.is_none() { return None;}
		let id = id.unwrap();

		Some( Auth::new(
			Some( user), None, Some( email), Some( id)))}
}
impl fmt::String for Auth {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let self_tuple = (
			self.user.as_ref(), self.comment.as_ref(),
			self.email.as_ref(), self.id.as_ref());
		match self_tuple {
			( Some( user), Some( comment), Some( email), Some( &id)) =>
				write!( formatter,
					"{} ({}) <{}> :: {:08x}",
					user, comment, email, id),
			( Some( user), Some( comment), Some( email), None) =>
				write!( formatter,
					"{} ({}) <{}>",
					user, comment, email),
			( Some( user), Some( comment), None, Some( &id)) =>
				write!( formatter,
					"{} ({}) :: {:08x}",
					user, comment, id),
			( Some( user), Some( comment), None, None) =>
				write!( formatter,
					"{} ({})",
					user, comment),
			( Some( user), None, Some( email), Some( &id)) =>
				write!( formatter,
					"{} <{}> :: {:08x}",
					user, email, id),
			( Some( user), None, Some( email), None) =>
				write!( formatter,
					"{} <{}>",
					user, email),
			( Some( user), None, None, Some( &id)) =>
				write!( formatter,
					"{} :: {:08x}",
					user, id),
			( Some( user), None, None, None) =>
				write!( formatter,
					"{}",
					user),
			( None, Some( comment), Some( email), Some( &id)) =>
				write!( formatter,
					"({}) <{}> :: {:08x}",
					comment, email, id),
			( None, Some( comment), Some( email), None) =>
				write!( formatter,
					"({}) <{}>",
					comment, email),
			( None, Some( comment), None, Some( &id)) =>
				write!( formatter,
					"({}) :: {:08x}",
					comment, id),
			( None, Some( comment), None, None) =>
				write!( formatter,
					"({})",
					comment),
			( None, None, Some( email), Some( &id)) =>
				write!( formatter,
					"<{}> :: {:08x}",
					email, id),
			( None, None, Some( email), None) =>
				write!( formatter,
					"<{}>",
					email),
			( None, None, None, Some( &id)) =>
				write!( formatter,
					":: {:08x}",
					id),
			( None, None, None, None) =>
				write!( formatter, ":null:"),}}
}
impl clone::Clone for Auth {
	fn clone( &self) -> Auth {
		Auth {
			user: self.user.clone(),
			comment: self.comment.clone(),
			email: self.email.clone(),
			id: self.id.clone(),
		}}

	fn clone_from( &mut self, source: &Auth){
		self.user = source.user.clone();
		self.comment = source.comment.clone();
		self.email = source.email.clone();
		self.id = source.id.clone();}
}
