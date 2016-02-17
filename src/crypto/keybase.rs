

use crypto::CryptoProvider;
use mote::*;

pub fn test() -> String {
	"test test".to_string()}

pub struct KeybaseSession {
	pub user: String,
}

impl KeybaseSession {
	pub fn new() -> KeybaseSession {
		KeybaseSession { user: "kurotetsuka".to_string()}}
}

impl CryptoProvider for KeybaseSession {
	fn sign( &self, mote: &mut Mote){
		let msg = &*mote.data;}
	fn verify( &self, mote: &Mote) -> bool { true}
	fn encrypt( &self, mote: &mut Mote){}
	fn decrypt( &self, mote: &mut Mote){}
}
