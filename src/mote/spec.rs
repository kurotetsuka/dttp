// library uses
use std::fmt;
use std::str::FromStr;

// local uses
use auth::*;
use dt::*;
use mote::*;

// 
pub struct MoteSpec {
	hash: Option< Vec<u8>>,
	auth: Option< Auth>,
	meta: Option< String>,
	datetime: Option< Datetime>,
}

impl MoteSpec {
	pub fn is_match( &self, _mote : &Mote) -> bool {
		if let Some( _hash) = self.hash.as_ref() {}
		if let Some( _auth) = self.auth.as_ref() {}
		if let Some( _meta) = self.meta.as_ref() {}
		if let Some( _datetime) = self.datetime.as_ref() {}
		true}
	pub fn match_rate( &self, _mote : &Mote) -> [u8; 4] {
		if let Some( _hash) = self.hash.as_ref() {}
		if let Some( _auth) = self.auth.as_ref() {}
		if let Some( _meta) = self.meta.as_ref() {}
		if let Some( _datetime) = self.datetime.as_ref() {}
		let result = [ 0x00; 4];
		result}
}

/*peg! mag_grammar( r#"
	use super::*;
	use auth::*;

	#[pub]
	spec -> MoteSpec
		= hash? auth? meta? dt?

	hash -> Vec<u8> = hex_byte+
	hex_byte -> u8 = [0-9a-f]{2} { match_str.parse();}

	auth -> Auth
"#);*/

pub enum MoteSpecParseError {
	UnknownError,
}

impl FromStr for MoteSpec {
	// todo: create custom ( more usable ) error type
	type Err = MoteSpecParseError;
	fn from_str( _string: &str) ->
			Result<MoteSpec, MoteSpecParseError> {
		Err( MoteSpecParseError::UnknownError)}
}

impl fmt::Display for MoteSpec {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		// write hash field
		match self.hash.as_ref() {
			Some( hash) => {
				for byte in hash {
					write!( formatter, "{:02x}", byte).ok();}},
			None => {}}
		// write auth field
		match self.auth.as_ref() {
			Some( auth) => {
				write!( formatter, "@{:?}", auth).ok();},
			None => {}}
		// write meta field
		match self.meta.as_ref() {
			Some( meta) => {
				write!( formatter, "#{:?}", meta).ok();},
			None => {}}
		// write datetime field
		match self.datetime.as_ref() {
			Some( datetime) => {
				write!( formatter, "!{}", datetime).ok();},
			None => {}}
		Ok( ())}
}

impl fmt::Debug for MoteSpec {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		// opening
		write!( formatter, "[").ok();
		// write hash field
		match self.hash.as_ref() {
			Some( hash) => {
				for byte in hash {
					write!( formatter, "{:02x} ", byte).ok();}},
			None => {
				write!( formatter, "- ").ok();}}
		// write auth field
		match self.auth.as_ref() {
			Some( auth) => {
				write!( formatter, "{:?} ", auth).ok();},
			None => {
				write!( formatter, "- ").ok();}}
		// write meta field
		match self.meta.as_ref() {
			Some( meta) => {
				write!( formatter, "{:?} ", meta).ok();},
			None => {
				write!( formatter, "- ").ok();}}
		// write datetime field
		match self.datetime.as_ref() {
			Some( datetime) => {
				write!( formatter, "{}", datetime).ok();},
			None => {
				write!( formatter, "-").ok();}}
		// closing
		write!( formatter, "]")}
}
