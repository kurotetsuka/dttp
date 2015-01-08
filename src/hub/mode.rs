// library uses

// local uses

#[derive( PartialEq, Eq, Copy, Hash, RustcEncodable, RustcDecodable)]
pub enum Mode {
	Track,
	Bootstrap,
	Push,
	Pull,
	Serve,
}
