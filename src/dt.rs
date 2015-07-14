// library uses
use std::fmt;
use time::now_utc;

// local uses

/// class that defines a date
#[derive( Clone, Copy, Hash)]
pub struct Datetime {
	pub year: u16,
	pub day: u16,
	pub milli: u32,
}
impl Datetime {
	pub fn null() -> Datetime {
		Datetime {
			year: 0,
			day: 0,
			milli: 0}}

	pub fn new( year: u16, day: u16, milli: u32) -> Datetime {
		Datetime {
			year: year,
			day: day,
			milli: milli}}

	pub fn now() -> Datetime {
		// get time
		let now = now_utc();
		// get millis
		let millis : u32 = (
			24 * now.tm_hour +
			60 * now.tm_min +
			1000 * now.tm_sec +
			now.tm_nsec / 1000) as u32;
		// return
		Datetime {
			year: ( now.tm_year + 1900) as u16,
			day: now.tm_yday as u16,
			milli: millis,}}

	pub fn from_str( string : &str) -> Option<Datetime> {
		let mut split = string.split( '.');

		// parse year
		let year_str = split.next();
		if year_str.is_none() { return None;}
		let year_str = year_str.unwrap();
		let year : Option<u16> =
			u16::from_str_radix( year_str, 16).ok();
		if year.is_none() { return None;}
		let year = year.unwrap();

		// parse day
		let day_str = split.next();
		if day_str.is_none() { return None;}
		let day_str = day_str.unwrap();
		let day : Option<u16> =
			u16::from_str_radix( day_str, 16).ok();
		if day.is_none() { return None;}
		let day = day.unwrap();

		// parse milli
		let milli_str = split.next();
		if milli_str.is_none() { return None;}
		let milli_str = milli_str.unwrap();
		let milli : Option<u32> =
			u32::from_str_radix( milli_str, 16).ok();
		if milli.is_none() { return None;}
		let milli = milli.unwrap();

		Some( Datetime::new( year, day, milli))}

	pub fn to_bytes( &self) -> Vec<u8> {
		let mut result = Vec::new();
		result.push( ( self.year >> 8) as u8);
		result.push( ( self.year >> 0) as u8);
		result.push( ( self.day >> 8) as u8);
		result.push( ( self.day >> 0) as u8);
		result.push( ( self.milli >> 24) as u8);
		result.push( ( self.milli >> 16) as u8);
		result.push( ( self.milli >> 08) as u8);
		result.push( ( self.milli >> 00) as u8);
		return result;}
}

impl fmt::Display for Datetime {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "{:03x}.{:03x}.{:07x}",
			self.year, self.day, self.milli)}
}
