fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct Excerpt<'a> {
//     part: &'a str
// }

fn main() {
    let s1 = String::from("Hello");
    let result;
    {
        let s2 = String::from("xyz");

        result = longest(s1.as_str(), s2.as_str());
    }
    println!("Longest is {}", result);

		// let novel = String::from("Call me Hello. Some years ago...");
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // let i = Excerpt {
        // part: first_sentence,
    // };
		// println!("ImportantExcerpt : {}", i.part);
}
