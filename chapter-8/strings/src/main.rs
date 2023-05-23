fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

		let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
		
		println!("s2 is {s2}");
		
		let s3 = String::from("foo");
		let s4 = String::from("bar");
		// the below works when s4 is a string literal
		// let s4 = "bar";
		// let s5 = s3 + s4;
		let s5 = s3 + &s4;
    println!("s5 is {s5}");
    // println!("s3 is {s3}");
    println!("s4 is {s4}");
}
