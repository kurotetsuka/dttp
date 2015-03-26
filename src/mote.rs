// library uses
use std::num;
use std::collections::BTreeMap;
use std::fmt;

use rand::Rng;

use rustc_serialize::base64;
use rustc_serialize::base64::*;
use rustc_serialize::base64::Newline::*;
use rustc_serialize::json;
use rustc_serialize::json::*;

// local uses
use auth::*;
use dt::*;
use key::*;

/// class that defines the types of data carried by a mote
#[derive( Clone, Copy, Hash)]
pub enum Class {
	// text classes
	Plain,
	Markdown,
	//  text data classes
	Json,
	// binary classes
	Raw,
	//  image classes
	Png,
	//  video classes
	Mp4,
}
impl Class {
	pub fn from_str( string: &str) -> Option<Class> {
		match string {
			"plain" => Some( Class::Plain),
			"markdown" => Some( Class::Markdown),
			"json" => Some( Class::Json),
			"raw" => Some( Class::Raw),
			"png" => Some( Class::Png),
			"mp4" => Some( Class::Mp4),
			_ => None,}}
}
impl fmt::Display for Class {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "{}",
			match *self {
				Class::Plain => "plain",
				Class::Markdown => "markdown",
				Class::Json => "json",
				Class::Raw => "raw",
				Class::Png => "png",
				Class::Mp4 => "mp4",})}
}

/// a unit of signed communication
#[derive( Clone, Hash)]
pub struct Mote {
	// a string describing the data
	pub meta: String,
	// the type of data
	pub class: Class,
	// the party signing the mote
	pub auth: Auth,
	// the release date of the mote
	pub datetime: Datetime,
	// pregen'd salt
	pub salt: u64,
	// the data field
	pub data: Vec<u8>,
	// attached signature
	pub sig: Vec<u8>,
}
impl Mote {
	// constructors
	pub fn null() -> Mote {
		Mote {
			meta: String::new(),
			class: Class::Raw,
			auth: Auth::null(),
			datetime: Datetime::null(),
			salt: 0x00000000,
			data: Vec::new(),
			sig: Vec::new(),}}
	pub fn new_bin(
			meta: String, class: Class,
			datetime: Datetime, data: Vec<u8>) -> Mote {
		Mote {
			meta: meta,
			class: class,
			auth: Auth::null(),
			datetime: datetime,
			salt: 0x00000000,
			data: data,
			sig: Vec::new(),}}
	pub fn new_text(
			meta: String, class: Class,
			datetime: Datetime, data: String) -> Mote {
		Mote {
			meta: meta,
			class: class,
			auth: Auth::null(),
			datetime: datetime,
			salt: 0x00000000,
			data: data.into_bytes(),
			sig: Vec::new(),}}

	pub fn from_str( string: &str) ->Option<Mote> {
		// parse message into message struct
		let msg : Option<MoteMsg> = json::decode( string).ok();
		if msg.is_none() { return None;}
		let msg = msg.unwrap();
		Mote::from_msg( &msg)}

	pub fn from_msg( msg: &MoteMsg) ->Option<Mote> {
		// parse class
		let class = Class::from_str( msg.class.as_ref());
		if class.is_none() { return None;}
		let class = class.unwrap();

		// parse auth
		let auth = Auth::from_str( msg.auth.as_ref());
		if auth.is_none() { return None;}
		let auth = auth.unwrap();

		// parse datetime
		let datetime = Datetime::from_str( msg.datetime.as_ref());
		if datetime.is_none() { return None;}
		let datetime = datetime.unwrap();

		// parse salt
		let salt: Option<u64> =
			num::from_str_radix( msg.salt.as_ref(), 16).ok();
		if salt.is_none() { return None;}
		let salt = salt.unwrap();

		// parse data
		let data : Option<Vec<u8>> =
			msg.data.from_base64().ok();
		if data.is_none() { return None;}
		let data = data.unwrap();

		// parse sig
		let sig : Option<Vec<u8>> =
			msg.sig.from_base64().ok();
		if sig.is_none() { return None;}
		let sig = sig.unwrap();

		// return
		Some( Mote {
			meta: msg.meta.clone(),
			class: class,
			auth: auth,
			datetime: datetime,
			salt: salt,
			data: data,
			sig: sig,})}


	pub fn salt<R: Rng>( &mut self, rng: &mut R){
		self.salt = rng.next_u64();}

	pub fn sign<Key: SecretKey>( &mut self, auth: &Auth, key: &Key){
		//generate plainbytes to sign
		let mut plain : Vec<u8> = Vec::new();
		//push meta bytes
		plain.push_all( self.meta.as_bytes());
		//push datetime bytes
		plain.push_all( self.datetime.to_bytes().as_ref());
		//push data bytes
		plain.push_all( self.data.as_ref());
		//push salt
		for &offset in [ 56, 48, 40, 32, 24, 16, 08, 00].iter() {
			plain.push( ( self.salt >> offset) as u8);}
		//set signature fields
		self.auth = ( *auth).clone();
		self.sig = key.sign( plain.as_ref());}

	pub fn to_msg( &self) -> MoteMsg {
		let b64_config = base64::Config {
			char_set: Standard,
			newline: LF,
			pad: true,
			line_length: None };
		MoteMsg {
			meta: self.meta.to_string(),
			class: self.class.to_string(),
			auth: self.auth.to_string(),
			datetime: self.datetime.to_string(),
			salt: format!( "{:08x}", self.salt),
			data: self.data.to_base64( b64_config),
			sig: self.sig.to_base64( b64_config),}}
}

impl fmt::Display for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let b64_config = base64::Config {
			char_set: Standard,
			newline: LF,
			pad: true,
			line_length: None };
		write!( formatter,
			"[\"{}\", {}, \"{}\", {}, {:08x}, {}, {}]",
			self.meta, self.class, self.auth,
			self.datetime, self.salt,
			self.data.to_base64( b64_config),
			self.sig.to_base64( b64_config),)}
}

/// a mote, prepared for transmittal
#[derive( Hash, RustcEncodable, RustcDecodable)]
pub struct MoteMsg {
	// a string describing the data
	pub meta: String,
	// the type of data
	pub class: String,
	// the party signing the mote
	pub auth: String,
	// the release date of the mote
	pub datetime: String,
	// pregen'd salt
	pub salt: String,
	// the data field
	pub data: String,
	// attached signature
	pub sig: String,
}
impl ToJson for MoteMsg {
	fn to_json( &self) -> json::Json {
		let mut map = BTreeMap::new();
		map.insert( "meta".to_string(), self.meta.to_json());
		map.insert( "class".to_string(), self.class.to_json());
		map.insert( "auth".to_string(), self.auth.to_json());
		map.insert( "datetime".to_string(), self.datetime.to_json());
		map.insert( "salt".to_string(), self.salt.to_json());
		map.insert( "data".to_string(), self.data.to_json());
		map.insert( "sig".to_string(), self.sig.to_json());
		json::Json::Object( map)}
}
