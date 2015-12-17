// library uses
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

// local uses
//use auth::*;

#[derive( PartialEq, Eq, Clone, Copy, Hash)]
pub enum ControlMsg {
	Stop,
}

#[derive( PartialEq, Eq, Clone, Copy, Hash)]
pub enum WorkerType {
	Bootstrap,
	Track,
	Push,
	Pull,
	ServeListen,
	ServeHandle,
}

pub struct WorkerControl {
	pub mode: WorkerType,
	pub guard: JoinHandle<()>,
	pub control: Sender<ControlMsg>,
}
impl WorkerControl {
	pub fn new( mode: WorkerType, guard: JoinHandle<()>,
			control: Sender<ControlMsg>) -> WorkerControl {
		WorkerControl {
			mode: mode,
			guard: guard,
			control: control,}}

	pub fn stop( &mut self){
		self.control.send( ControlMsg::Stop).ok();}

	pub fn join( self){
		let _ = self.guard.join();}
}
