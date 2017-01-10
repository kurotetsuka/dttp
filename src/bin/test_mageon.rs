// features and plugins
#![feature(
	plugin,
	custom_derive,
	slice_patterns,
	advanced_slice_patterns)]
#![plugin(
	peg_syntax_ext)]

// library imports
extern crate dttp;

// local uses
use dttp::mageon::Mageon;

peg! grammar( r#"
	#[pub]
	quot_str -> String = "\"" s:$( ( !( "\\\"" / "\"" ) . ) ** "\\\"" ) "\""
		{ // remove the quotes, preserve the rest 
			s.replace( "\\\"", "\"")}
"#);

fn main(){
	let _tests = vec![
		r#"hello: { "asdf": 1}."#,
		r#"there: [ 1, 2, 3, 4]."#,
		r#"aly?: 1."#,
		r#"aly!: 1."#,
		r#"aly: hi, [ 1, "hi"]."#,
		r#"aly: hi, "hi"."#,
		r#"aly?"#,
		r#"aly!"#,
		r#"aly."#,];

	let test = "\"asdf\"";
	println!( "parsing <{}>", test);
	println!( "result: {:?}\n", grammar::quot_str( test));

	//for test in tests {
	//	println!( "parsing <{}>", test);
	//	println!( "result: {:?}\n", test.parse::<Mageon>());}

	let x : Mageon = "hi!: 1, 2, 3.".parse().unwrap();
	println!( "x: {:?}", x);
}