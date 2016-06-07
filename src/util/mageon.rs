// library uses
use rustc_serialize::json;
use rustc_serialize::json::Json;


pub struct Mageon {
	pub verb: String,
	pub args: Json,
}

pub enum MagArg {
	String( String),
	Object( Json),
	Vec( Vec< MagArg>),
}