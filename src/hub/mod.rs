// library uses
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{ Arc, Mutex};

// local uses
use auth::*;
use mote::*;
use key::*;
use hub::remote::*;
use hub::worker::*;
//use protocol::*;
//use protocol::Command::*;
//use protocol::Response::*;

// modules
pub mod remote;
pub mod worker;

// constants
//static PUSH_LOG_DECLINE: bool = false;
//static PUSH_PAUSE_MILLIS: i64 = 400;
//static BOOTSTRAP_PAUSE_MILLIS: i64 = 400;
//static GREET_PAUSE_MILLIS: i64 = 400;

#[derive( PartialEq, Eq, Clone, Copy, Hash)]
pub enum Mode {
	Bootstrap,
	Track,
	Push,
	Pull,
	Serve,
}

//#[derive( Clone)]
pub struct Hub {
	// this hub's authorizing party
	pub auth: Arc<Mutex< Auth>>,
	// this hub's authorizing key
	pub sec_key: HyphSecretKey,
	// this hub's verifying key
	pub pub_key: HyphPublicKey,

	// this hub's hostname
	pub hostname: Arc<Mutex< String>>,
	// this hub's port
	pub port: Arc<Mutex< u16>>,

	// this hub's auth-key database
	pub authdb: Arc<Mutex< HashMap<Auth, HyphPublicKey>>>,
	// this hub's stored motes
	pub motedb: Arc<Mutex< Vec<Mote>>>,
	// this hub's auth database
	pub remotedb: Arc<Mutex< Vec<RemoteHub>>>,

	// this hub's enabled operation modes
	modes: Arc<Mutex< HashMap<Mode, bool>>>,
	// this hub's workers
	workers: Arc<Mutex< HashMap<WorkerType, Vec<WorkerControl>>>>,
}
impl Hub {
	pub fn new( auth: Auth, sec_key: HyphSecretKey, pub_key: HyphPublicKey,
			hostname: String, port: u16) -> Hub {
		let mut workers = HashMap::new();
		workers.insert( WorkerType::Bootstrap, Vec::new());
		workers.insert( WorkerType::Track, Vec::new());
		workers.insert( WorkerType::Push, Vec::new());
		workers.insert( WorkerType::Pull, Vec::new());
		workers.insert( WorkerType::ServeListen, Vec::new());
		workers.insert( WorkerType::ServeHandle, Vec::new());

		let mut modes = HashMap::new();
		modes.insert( Mode::Bootstrap, false);
		modes.insert( Mode::Track, false);
		modes.insert( Mode::Push, false);
		modes.insert( Mode::Pull, false);
		modes.insert( Mode::Serve, false);

		Hub {
			auth: Arc::new( Mutex::new( auth)),
			sec_key: sec_key,
			pub_key: pub_key,

			hostname: Arc::new( Mutex::new( hostname)),
			port: Arc::new( Mutex::new( port)),

			authdb: Arc::new( Mutex::new( HashMap::new())),
			motedb: Arc::new( Mutex::new( Vec::new())),
			remotedb: Arc::new( Mutex::new( Vec::new())),

			modes: Arc::new( Mutex::new( modes)),
			workers: Arc::new( Mutex::new( workers)),}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}

	pub fn add_remote( &mut self, addr: SocketAddr){
		let remote = RemoteHub::new( addr);
		let mut remotedb = self.remotedb.lock().unwrap();
		remotedb.push( remote);}

	pub fn add_mote( &mut self, mote: Mote){
		let mut motedb = self.motedb.lock().unwrap();
		motedb.push( mote);}

	pub fn mode_get( &self, mode: &Mode) -> bool {
		let modes = self.modes.lock().unwrap();
		if let Some( &result) = modes.get( mode) {
			return result;}
		else {
			return false;}}

	pub fn mode_set( &mut self, mode: Mode, enabled: bool) {
		// check if we actually have to do anything
		if self.mode_get( &mode) == enabled {
			return;}
		let _workers = self.workers.lock().unwrap();
		return;}

	pub fn launch( &mut self){}
}

/*impl Clone for Hub {
	fn clone( &self) -> Hub {
		panic!();}
	fn clone_from( &mut self, _source: &Hub){}
}*/
