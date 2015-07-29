// library uses
use rustc_serialize::base64;
use rustc_serialize::base64::*;

// base dttp version
pub static DTTPV: &'static str = "0.3.0/plain";

// standard base64 config
pub static B64_CONFIG: base64::Config =
	base64::Config {
		char_set: Standard,
		newline: Newline::LF,
		pad: true,
		line_length: None };
