// library uses
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{ Hash, Hasher, SipHasher};
use std::str::FromStr;

use rustc_serialize::base64::*;
use rustc_serialize::json;
use rustc_serialize::json::*;

// local uses
use auth::*;
use consts::*;
use dt::*;
use key::*;

/// a unit of signed communication
#[derive( Clone, Hash)]
pub struct Mote {
	// a string indicating the protocol version / extension
	pub dttpv: String,
	// the party signing the mote
	pub auth: Auth,
	// a string describing the data
	pub meta: String,
	// the release date of the mote
	pub datetime: Datetime,
	// the data field
	pub data: String,
	// attached signature
	pub sig: String,
}
impl Mote {
	// constructors
	pub fn null() -> Mote {
		Mote {
			dttpv: DTTPV.to_string(),
			auth: Auth::null(),
			meta: String::new(),
			datetime: Datetime::null(),
			data: String::new(),
			sig: String::new(),}}

	pub fn new_text(
			auth: Auth,
			meta: String,
			datetime: Datetime,
			data: String) -> Mote {
		Mote {
			dttpv: DTTPV.to_string(),
			auth: auth,
			meta: meta,
			datetime: datetime,
			data: data,
			sig: String::new(),}}

	// from_ constructors
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
			dttpv: dttpv, auth: auth,
			meta: meta, datetime: datetime,
			data: data, sig: sig,})}

	pub fn hash_sip( &self) -> u64 {
		let mut hasher = SipHasher::new();
		self.hash( &mut hasher);
		hasher.finish()}
	pub fn hash_sipk( &self, key0: u64, key1: u64) -> u64 {
		let mut hasher = SipHasher::new_with_keys( key0, key1);
		self.hash( &mut hasher);
		hasher.finish()}

	pub fn sign( &mut self, key: &DttpSecretKey){
		// generate plainbytes to sign
		let mut plain : Vec<u8> = Vec::new();
		// push dttpv bytes
		plain.extend( self.dttpv.as_bytes());
		// push meta bytes
		plain.extend( self.meta.as_bytes());
		// push auth bytes
		plain.extend( self.auth.to_string().as_bytes());
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
		// push datetime bytes
		plain.extend( self.datetime.to_bytes().iter());
		// push data bytes
		plain.extend( self.data.iter());
		// check signature field
		return key.verify( plain.as_ref(), self.sig.as_ref());}

	pub fn to_msg( &self) -> MoteMsg {
		MoteMsg {
			dttpv: self.dttpv.clone(),
			meta: self.meta.clone(),
			auth: self.auth.clone(),
			datetime: self.datetime.to_string(),
			data: self.data.clone(),
			sig: self.sig.clone(),}}
}

impl fmt::Debug for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter,
			"[dttpv-{} \"{}\" \"{}\" {} \"{:?}\" \"{:?}\"]",
			self.dttpv, self.auth, self.meta, self.datetime,
			self.data.to_base64( B64_CONFIG),
			self.sig.to_base64( B64_CONFIG),)}
}

/// a mote, prepared for transport
#[derive( Hash, RustcEncodable, RustcDecodable)]
pub struct MoteMsg {
	// a string indicating the protocol version / extension
	pub dttpv: String,
	// the party signing the mote
	pub auth: String,
	// a string describing the data
	pub meta: String,
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
		map.insert( "auth".to_string(), self.auth.to_json());
		map.insert( "meta".to_string(), self.meta.to_json());
		map.insert( "datetime".to_string(), self.datetime.to_json());
		map.insert( "data".to_string(), self.data.to_json());
		map.insert( "sig".to_string(), self.sig.to_json());
		json::Json::Object( map)}
}
