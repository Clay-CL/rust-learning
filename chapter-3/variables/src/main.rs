fn main() {

    // const decl
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3;

    let x = "Hello world";
    println!("x is {x}");

    let mut x = 5;
    println!("First value of x = {x}");

    x = "Hello again";
    x = 6;
    println!("Changed x : {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is {x}");
    }

    println!("x outside is {x}");
}
