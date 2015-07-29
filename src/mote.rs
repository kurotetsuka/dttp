// library uses
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use rustc_serialize::base64::*;
use rustc_serialize::json;
use rustc_serialize::json::*;

// local uses
use auth::*;
use consts::*;
use dt::*;
use key::*;

/// class that defines the types of data carried by a mote
#[derive( Clone, Copy, Hash)]
pub enum Class {
	// text classes
	Plain,
	Markdown,
	// + text data classes
	Json,
	// binary classes
	Raw,
	// + image classes
	Png,
	// + video classes
	Mp4,
}
impl Class {
	fn to_byte( &self) -> u8 {
		match self {
			&Class::Plain =>
				0x00,
			&Class::Markdown =>
				0x01,
			&Class::Json =>
				0x02,
			&Class::Raw =>
				0x80,
			&Class::Png =>
				0x81,
			&Class::Mp4 =>
				0x82,}}
}
impl FromStr for Class {
	type Err = ();

	fn from_str( string: &str) -> Result<Class, ()> {
		match string {
			"plain" => Ok( Class::Plain),
			"markdown" => Ok( Class::Markdown),
			"json" => Ok( Class::Json),
			"raw" => Ok( Class::Raw),
			"png" => Ok( Class::Png),
			"mp4" => Ok( Class::Mp4),
			_ => Err( ()),}}
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
	// a string indicating the protocol version / extension
	pub dttpv: String,
	// a string describing the data
	pub meta: String,
	// the party signing the mote
	pub auth: Auth,
	// the type of data
	pub class: Class,
	// the release date of the mote
	pub datetime: Datetime,
	// the data field
	pub data: Vec<u8>,
	// attached signature
	pub sig: Vec<u8>,
}
impl Mote {
	// constructors
	pub fn null() -> Mote {
		Mote {
			dttpv: DTTPV.to_string(),
			meta: String::new(),
			auth: Auth::null(),
			class: Class::Raw,
			datetime: Datetime::null(),
			data: Vec::new(),
			sig: Vec::new(),}}
	pub fn new_bin(
			meta: String, class: Class,
			datetime: Datetime, data: Vec<u8>) -> Mote {
		Mote {
			dttpv: DTTPV.to_string(),
			meta: meta,
			auth: Auth::null(),
			class: class,
			datetime: datetime,
			data: data,
			sig: Vec::new(),}}
	pub fn new_text(
			meta: String, class: Class,
			datetime: Datetime, data: String) -> Mote {
		Mote {
			dttpv: DTTPV.to_string(),
			meta: meta,
			auth: Auth::null(),
			class: class,
			datetime: datetime,
			data: data.into_bytes(),
			sig: Vec::new(),}}

	pub fn from_str( string: &str) -> Option<Mote> {
		// parse message into message struct
		let msg : Option<MoteMsg> = json::decode( string).ok();
		if msg.is_none() { return None;}
		let msg = msg.unwrap();
		Mote::from_msg( &msg)}

	pub fn from_msg( msg: &MoteMsg) -> Option<Mote> {
		// get dttpv, meta
		let dttpv = msg.dttpv.clone();
		let meta = msg.meta.clone();

		// parse auth
		let auth = Auth::from_str( msg.auth.as_ref());
		if auth.is_none() { return None;}
		let auth = auth.unwrap();

		// parse class
		let class = Class::from_str( msg.class.as_ref());
		if class.is_err() { return None;}
		let class = class.unwrap();

		// parse datetime
		let datetime = Datetime::from_str( msg.datetime.as_ref());
		if datetime.is_none() { return None;}
		let datetime = datetime.unwrap();

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
			dttpv: dttpv, meta: meta, auth: auth,
			class: class, datetime: datetime,
			data: data, sig: sig,})}

	pub fn sign( &mut self, key: &DttpSecretKey){
		// generate plainbytes to sign
		let mut plain : Vec<u8> = Vec::new();
		// push dttpv bytes
		plain.extend( self.dttpv.as_bytes());
		// push meta bytes
		plain.extend( self.meta.as_bytes());
		// push auth bytes
		plain.extend( self.auth.to_string().as_bytes());
		// push class bytes
		plain.push( self.class.to_byte());
		// push datetime bytes
		plain.extend( self.datetime.to_bytes().iter());
		// push data bytes
		plain.extend( self.data.iter());
		// set signature
		self.sig = key.sign( plain.as_ref());}

	pub fn verify( &self, key: &DttpPublicKey) -> bool {
		// generate plainbytes to verify
		let mut plain : Vec<u8> = Vec::new();
		// push dttpv bytes
		plain.extend( self.dttpv.as_bytes());
		// push meta bytes
		plain.extend( self.meta.as_bytes());
		// push auth bytes
		plain.extend( self.auth.to_string().as_bytes());
		// push class bytes
		plain.push( self.class.to_byte());
		// push datetime bytes
		plain.extend( self.datetime.to_bytes().iter());
		// push data bytes
		plain.extend( self.data.iter());
		// check signature field
		return key.verify( plain.as_ref(), self.sig.as_ref());}

	pub fn to_msg( &self) -> MoteMsg {
		MoteMsg {
			dttpv: self.dttpv.to_string(),
			meta: self.meta.to_string(),
			class: self.class.to_string(),
			auth: self.auth.to_string(),
			datetime: self.datetime.to_string(),
			data: self.data.to_base64( B64_CONFIG),
			sig: self.sig.to_base64( B64_CONFIG),}}
}

impl fmt::Display for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter,
			"[dttpv-{} \"{}\" \"{}\" {} {} {} {}]",
			self.dttpv, self.meta, self.auth,
			self.class, self.datetime,
			self.data.to_base64( B64_CONFIG),
			self.sig.to_base64( B64_CONFIG),)}
}

/// a mote, prepared for transmittal
#[derive( Hash, RustcEncodable, RustcDecodable)]
pub struct MoteMsg {
	// a string indicating the protocol version / extension
	pub dttpv: String,
	// a string describing the data
	pub meta: String,
	// the party signing the mote
	pub auth: String,
	// the type of data
	pub class: String,
	// the release date of the mote
	pub datetime: String,
	// the data field
	pub data: String,
	// attached signature
	pub sig: String,
}
impl ToJson for MoteMsg {
	fn to_json( &self) -> json::Json {
		let mut map = BTreeMap::new();
		map.insert( "dttpv".to_string(), self.dttpv.to_json());
		map.insert( "meta".to_string(), self.meta.to_json());
		map.insert( "class".to_string(), self.class.to_json());
		map.insert( "auth".to_string(), self.auth.to_json());
		map.insert( "datetime".to_string(), self.datetime.to_json());
		map.insert( "data".to_string(), self.data.to_json());
		map.insert( "sig".to_string(), self.sig.to_json());
		json::Json::Object( map)}
}
