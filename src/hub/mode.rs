// library uses

// local uses

#[derive( PartialEq, Eq)]
#[derive( Copy, Hash)]
#[derive( RustcEncodable, RustcDecodable)]
pub enum Mode {
	Track,
	Bootstrap,
	Push,
	Pull,
	Serve,
}
