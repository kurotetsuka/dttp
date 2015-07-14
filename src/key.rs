// library uses
//use std::fmt;
use rand::Rng;
use gpgme::keys::Key;

// local uses
//use asdf;

pub trait SecretKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, data: &[u8]) -> Vec<u8>;
}
pub trait PublicKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, data: &[u8], sig :&[u8]) -> bool;
}

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
pub fn keygen_fake<R: Rng>( rng: &mut R) -> FakeKeyPair {
	let mut sec_key = [ 0u8; 8];
	let mut pub_key = [ 0u8; 8];
	rng.fill_bytes( &mut sec_key);
	rng.fill_bytes( &mut pub_key);
	( sec_key, pub_key )}

pub struct GpgSecKey {
	_inner: Key,
}
pub struct GpgPubKey {
	_inner: Key,
}
impl SecretKey for GpgSecKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, _data: &[u8]) -> Vec<u8> {
		vec!(
			0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00)}
}
impl PublicKey for GpgPubKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, _data: &[u8], _sig :&[u8]) -> bool {
		true}
}

pub enum HyphPublicKey {
	Fake( FakePubKey),
	Gpg( GpgPubKey),
}

pub enum HyphSecretKey {
	Fake( FakeSecKey),
	Gpg( GpgSecKey),
}
