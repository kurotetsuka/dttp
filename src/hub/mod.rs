// standard library uses
use std::collections::HashMap;
use std::net::SocketAddr;
//use std::sync::{ Arc, Mutex};

// other library uses
use arcmutex::{ arcmutex, ArcMutex};

// local uses
use auth::*;
use mote::*;
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
	pub auth: ArcMutex<Auth>,
	// this hub's authorizing key
	//pub sec_key: DttpSecretKey,
	// this hub's verifying key
	//pub pub_key: DttpPublicKey,

	// this hub's hostname
	pub hostname: ArcMutex<String>,
	// this hub's port
	pub port: ArcMutex<u16>,

	// this hub's auth-key database
	//pub authdb: ArcMutex< HashMap<Auth, DttpPublicKey>>,
	// this hub's stored motes
	pub motedb: ArcMutex< Vec<Mote>>,
	// this hub's auth database
	pub remotedb: ArcMutex< Vec<RemoteHub>>,

	// this hub's enabled operation modes
	modes: ArcMutex< HashMap<Mode, bool>>,
	// this hub's workers
	workers: ArcMutex< HashMap<WorkerType, Vec<WorkerControl>>>,
}
impl Hub {
	//, sec_key: DttpSecretKey, pub_key: DttpPublicKey
	pub fn new( auth: Auth, hostname: String, port: u16) -> Hub {
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
			auth: arcmutex( auth),
			//sec_key: sec_key,
			//pub_key: pub_key,

			hostname: arcmutex( hostname),
			port: arcmutex( port),

			//authdb: arcmutex( HashMap::new()),
			motedb: arcmutex( Vec::new()),
			remotedb: arcmutex( Vec::new()),

			modes: arcmutex( modes),
			workers: arcmutex( workers),}}

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
