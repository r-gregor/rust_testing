/*
 * chapter 3.5: control flow
 * 20240903
 * 20240904
 */

/* main */
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

    println!("Branching with 'if/else if/else':");
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

    /* using if in a let statement */
    println!("Using 'if' in 'let' statement:");
	let condition = true;
	let number = if condition {5} else {6};
	println!("The value of number is {number}");

	println!("---");

    /* repeating code with loop */
    println!("Repeating code with 'loop':");
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
    println!("Returning values from loops:");
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if 10  == counter {
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
    println!("Loop 'labels':");
	let mut count = 0;
	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop { // inner loop!
			println!("remaining = {remaining}");
			if 9 == remaining {
				break;
			}
			if 2 == count {
				break 'counting_up;
			}
			remaining -= 1;
		}
		count += 1;
	}
	println!("End count = {count}");

	println!("---");

    /* while loops */
    println!("'while' loop:");
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

	println!("---");

    println!("Looping over array with 'while' loop:");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

	println!("---");

    /* for loops */
    println!("Looping over array with 'for' loop:");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {element}");
    }

    println!("---");

    /* 'for' loop with 'range' */
    println!("'for' loop with 'range':");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

} /* end main */


