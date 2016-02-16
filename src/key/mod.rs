// library uses
//use std::fmt;
use rand::Rng;

// modules
pub mod gpg;
pub mod keybase;

// local uses
use self::gpg::*;
use self::keybase::*;

pub mod fake {
	pub type FakeSecKey = [u8; 8];
	pub type FakePubKey = [u8; 8];
	impl SecretKey for FakeSecKey {
		//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
		fn sign( &self, _data: &[u8]) -> Vec<u8> {
			vec!(
				0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00)}
	}
	impl PublicKey for FakePubKey {
		//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
		fn verify( &self, _data: &[u8], _sig :&[u8]) -> bool {
			true}
	}

	pub type FakeKeyPair = ( FakeSecKey, FakePubKey);
	pub fn keygen_fake<R: Rng>( rng: &mut R) -> DttpKeyPair {
		let mut sec_key = [ 0u8; 8];
		let mut pub_key = [ 0u8; 8];
		rng.fill_bytes( &mut sec_key);
		rng.fill_bytes( &mut pub_key);
		( DttpSecretKey::Fake( sec_key), DttpPublicKey::Fake( pub_key) )}
}

pub trait SecretKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, data: &[u8]) -> Vec<u8>;
}
pub trait PublicKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, data: &[u8], sig :&[u8]) -> bool;
}

pub type DttpKeyPair = ( DttpSecretKey, DttpPublicKey);
pub enum DttpSecretKey {
	Fake( FakeSecKey),
	Gpg( GpgSecKey),
}
pub enum DttpPublicKey {
	Fake( FakePubKey),
	Gpg( GpgPubKey),
}
impl SecretKey for DttpSecretKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, data: &[u8]) -> Vec<u8> {
		match self {
			&DttpSecretKey::Fake( ref inner) => inner.sign( data),
			&DttpSecretKey::Gpg( ref inner) => inner.sign( data),}}
}
impl PublicKey for DttpPublicKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, data: &[u8], sig :&[u8]) -> bool {
		match self {
			&DttpPublicKey::Fake( ref inner) => inner.verify( data, sig),
			&DttpPublicKey::Gpg( ref inner) => inner.verify( data, sig),}}
}
