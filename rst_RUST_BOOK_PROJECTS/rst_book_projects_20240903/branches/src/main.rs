/*
 * chapter 3.5: control flow
 * 20240903
 */

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("---");

    let number = 7;
    /* error: 
    if number {
        println!("number was 7");
    }
    */

    if number != 0 {
        println!("Number {number} is something other than zero");
    }

    println!("---");

    let number = 6;

    if number % 4 == 0 {
        println!("Number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number {number} is divisible by 3");
    } else if number % 3 == 0 {
        println!("Number {number} is divisible by 2");
    } else {
        println!("Number {number} is not divisible by 4, 3, or 2");
    }

    println!("---");

    /* using if in fa let statement */
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is {number}");

    println!("---");

    /* repeating code with loop */
    let mut cnt: usize = 5;
    loop {
        println!("Looping ...!");
        cnt -= 1;
        if cnt == 0 {
            break
        }
    }

    println!("---");

    /* returning values from loops */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break
            counter * 2;
            /* or:
            break counter * 2;
            */
        }
    };
    println!("The result is {result}");

    println!("---");

    /* loop labels */
    // ....
}

