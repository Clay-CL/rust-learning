fn main() {

    // const decl
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3;

    let x = "Hello world";
    println!("x is {x}");

    let mut x = 5;
    println!("First value of x = {x}");

    x = 6;
    println!("Changed x : {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is {x}");
    }

    println!("x outside is {x}");

    // test loops here
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
        println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
