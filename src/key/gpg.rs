use rand::Rng;
use gpgme::keys::Key;
use super::fake::*;

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

pub type GpgKeyPair = ( GpgSecKey, GpgPubKey);
pub fn keygen_gpg<R: Rng>( rng: &mut R) -> DttpKeyPair {
	keygen_fake( rng)}
