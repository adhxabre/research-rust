fn main() {
    println!("Hello world!");

    // immutable variable declaration
    let x = 5;

    println!("{}", x);

    // mutable variable declaration
    let mut y = 6;
    y = 10;

    println!("{}", y);

    // we can also declare a mutable variable like this
    let z = 10;
    let z = 15;

    println!("{}", z)
}
