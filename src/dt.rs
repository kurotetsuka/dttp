// library uses
use std::fmt;
use time::now_utc;
use std::str::FromStr;

// local uses
use self::ParseDatetimeError::*;

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

		// get year
		// get day
		// get milli
		let milli = (
			now.tm_nsec / 1000 +
			1000 * now.tm_sec +
			60 * 1000 * now.tm_min +
			24 * 60 * 1000 * now.tm_hour) as u32;

		// return
		Datetime {
			year: ( now.tm_year + 1900) as u16,
			day: now.tm_yday as u16,
			milli: milli}}

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

pub enum ParseDatetimeError {
	YearParseFail,
	DayParseFail,
	MilliParseFail,
}
impl fmt::Debug for ParseDatetimeError {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let err_string = match self {
				&YearParseFail => {
					"Failed to parse year"},
				&DayParseFail => {
					"Failed to parse day"},
				&MilliParseFail => {
					"Failed to parse milli"}};
		write!( formatter, "{}", err_string)}
}

impl FromStr for Datetime {
	type Err = ParseDatetimeError;

	fn from_str( string: &str) -> Result<Datetime, ParseDatetimeError> {
		let mut split = string.split( '.');

		// get strings
		let year_str = split.next();
		let day_str = split.next();
		let milli_str = split.next();

		// error check
		if year_str.is_none() {
			return Err( YearParseFail);}
		if day_str.is_none() {
			return Err( DayParseFail);}
		if milli_str.is_none() {
			return Err( MilliParseFail);}

		// unwrap
		let year_str = year_str.unwrap();
		let day_str = day_str.unwrap();
		let milli_str = milli_str.unwrap();

		// parse year, day, milli
		let year : Option<u16> =
			u16::from_str_radix( year_str, 16).ok();
		let day : Option<u16> =
			u16::from_str_radix( day_str, 16).ok();
		let milli : Option<u32> =
			u32::from_str_radix( milli_str, 16).ok();

		// error check
		if year.is_none() {
			return Err( YearParseFail);}
		if day.is_none() {
			return Err( DayParseFail);}
		if milli.is_none() {
			return Err( MilliParseFail);}

		// unwrap
		let year = year.unwrap();
		let day = day.unwrap();
		let milli = milli.unwrap();

		// return
		Ok( Datetime::new( year, day, milli))}
}

impl fmt::Display for Datetime {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "{:03x}.{:03x}.{:07x}",
			self.year, self.day, self.milli)}
}
