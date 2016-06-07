// library imports
extern crate dttp;

// local uses
use dttp::mageon::Mageon;

fn main(){
	let tests = vec![
		r#"hello: { "asdf": 1}."#,
		r#"there: [ 1, 2, 3, 4]."#,
		r#"aly?: 1."#,
		r#"aly!: 1."#,
		r#"aly: hi, [ 1, "hi"]."#,
		r#"aly: hi, "hi"."#,
		r#"aly?"#,
		r#"aly!"#,
		r#"aly."#,];

	for test in tests {
		println!( "parsing <{}>", test);
		println!( "result: {:?}\n", test.parse::<Mageon>());}

	let x : Mageon = "hi!: 1, 2, 3.".parse().unwrap();
	println!( "x: {:?}", x);
}