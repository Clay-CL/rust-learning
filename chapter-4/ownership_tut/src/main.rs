fn main() {
    
    // this doesn't work since memory needs to be allocated on the heap
    // which is dynamic and cannot be fixed
    /* let s1 = String::from("hello"); */
    /* let s2 = s1; */
    /* println!("{s1}, world!"); */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
 
    // the following works since the memory is stored on the stack
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let mut s = String::from("hello");

    let mut r1 = &mut s;

    let r2 = &mut r1;
    println!("{}",r2);
    /* println!("{}, {}", r1, r2); */
    println!("{}",r1);
    
}
