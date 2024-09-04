/*
 * chapter 3.1: Variables and mutability
 * 20240903
 */

fn main() {
    /* it does NOT work if variable is not mutable */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("---");

    /* example of shadowing the variables with 'let' keyword */
    let x2 = 5;
    let x2 = x2 + 1;
    {
        let x2 = x2 * 2;
        println!("The value of x2 in inner scope is: {x2}");
    }
    println!("The value of x2 is: {x2}");

    println!("---");

    /* use of shadowing to change the type of variable */
    let spaces = "     ";       // String of 5 spaces
    let spaces = spaces.len();  // integer: '5'
    println!("spaces: {spaces}");

    println!("---");


}

