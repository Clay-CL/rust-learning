use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);


		let mut scores = HashMap::new();
		scores.insert(String::from("Blue"), 10);

		scores.entry(String::from("Yellow")).or_insert(50);
		scores.entry(String::from("Blue")).or_insert(50);

		println!("{:?}", scores)
}
