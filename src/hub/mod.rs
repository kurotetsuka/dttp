// library uses
use std::collections::HashMap;
//use std::hash;
//use std::io::{ Acceptor, BufferedReader, Listener};
use std::io::net::ip::SocketAddr;
//use std::io::net::ip::{ SocketAddr, ToSocketAddr};
//use std::io::net::tcp::{ TcpListener, TcpStream};
//use std::io::timer::sleep;
use std::sync::{ Arc, Mutex};
use std::sync::mpsc::Sender;
//use std::sync::mpsc::{ self, Sender, Receiver};
use std::thread::JoinGuard;
//use std::thread::{ Thread, JoinGuard};
//use std::time::duration::Duration;

//use rustc_serialize::Decodable;
//use rustc_serialize::json;
//use rustc_serialize::json::{ Json, ToJson};

// local uses
use auth::*;
use mote::*;
use hub::remote::RemoteHub;
//use protocol::*;
//use protocol::Command::*;
//use protocol::Response::*;

// modules
pub mod remote;

// constants
//static PUSH_LOG_DECLINE: bool = false;
//static PUSH_PAUSE_MILLIS: i64 = 400;
//static BOOTSTRAP_PAUSE_MILLIS: i64 = 400;
//static GREET_PAUSE_MILLIS: i64 = 400;

#[derive( PartialEq, Eq, Copy, Hash)]
pub enum Mode {
	Track,
	Bootstrap,
	Push,
	Pull,
	Serve,
}

#[derive( PartialEq, Eq, Copy, Hash)]
pub enum ControlMsg {
	Stop,
}

pub struct Hub {
	// this hub's authorizing party
	pub auth: Arc<Mutex<Auth>>,
	// this hub's hostname
	pub hostname: Arc<Mutex<String>>,
	// this hub's port
	pub port: Arc<Mutex<u16>>,
	// this hub's stored motes
	pub motedb: Arc<Mutex<Vec<Mote>>>,
	// this hub's auth-key database
	//pub authdb: Vec<Auth>,
	// this hub's auth database
	pub remotedb: Arc<Mutex<Vec<RemoteHub>>>,
	// this hub's authorizing key
	//pub sec_key: AuthSecKey,
	// this hub's verifying key
	//pub pub_key: AuthPubKey,
	// this hub's workers
	pub workers: HashMap<Mode, Vec<WorkerControl>>,
}
impl Hub {
	pub fn new( auth: Auth, hostname: String, port: u16) -> Hub {
		let mut workers = HashMap::new();
		workers.insert( Mode::Track, Vec::new());
		workers.insert( Mode::Bootstrap, Vec::new());
		workers.insert( Mode::Push, Vec::new());
		workers.insert( Mode::Pull, Vec::new());
		workers.insert( Mode::Serve, Vec::new());

		Hub {
			auth: Arc::new( Mutex::new( auth)),
			hostname: Arc::new( Mutex::new( hostname)),
			port: Arc::new( Mutex::new( port)),
			motedb: Arc::new( Mutex::new( Vec::new())),
			remotedb: Arc::new( Mutex::new( Vec::new())),
			workers: workers,}}

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
		match self.workers.get( mode) {
			Some( _) => true,
			None => false,}}
	pub fn mode_set( &mut self, _mode: Mode, _enabled: bool) {}
		//self.workers.insert( mode, enabled);}

	pub fn launch( &mut self){}
}

impl Clone for Hub {
	fn clone( &self) -> Hub {
		panic!();}
	fn clone_from( &mut self, _source: &Hub){}
}

pub struct WorkerControl {
	pub mode: Mode,
	pub guard: JoinGuard<'static, ()>,
	pub control: Sender<ControlMsg>,
}
impl WorkerControl {
	pub fn new( mode: Mode, guard: JoinGuard<()>,
			control: Sender<ControlMsg>) -> WorkerControl {
		WorkerControl {
			mode: mode,
			guard: guard,
			control: control,}}

	pub fn stop( &mut self){
		self.control.send( ControlMsg::Stop).ok();}

	pub fn join( &self){
		self.guard.join()}
}
