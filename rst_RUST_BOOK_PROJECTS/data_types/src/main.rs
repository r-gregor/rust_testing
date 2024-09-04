/*
 * chapter 3.2: Data Types
 * 20240903
 */

use std::io;
use std::io::Write; // flush()

fn main() {

    /* array type*/
    let a = [1, 2, 3, 4, 5];

    print!("Please enter an array index. (!max 4!):  ");
    io::stdout().flush().unwrap(); // continue on same line

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    /*
    let index: usize = index
        .trim()
        .parse()
        .expect("\
#################################
# Index entered is NOT a number #
#################################
");
    */

    loop {
        let index :isize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => -1,
        };

        if index < 0 {
            println!("Wrong input!");
            break;
        }

        if index > (a.len() - 1).try_into().unwrap() { // convert usize to isize
            println!("Index TO big!");
            break;
        }
            let element = a[index as usize]; // already checke that it is not < 0 !!
            println!("The value of the element at index {index} is: {element}");
            break;
    }

    println!("---");

    /* tuple (compaund type) */
    let fmem_data1: (String, String, u8) = (String::from("Gregor"), String::from("Redelonghi"), 56);
    let fmem_data2: (&str, &str, u8) = ("Gregor", "Redelonghi", 56);
    let fmem_data3: (&str, &str, u8) = ("Špela", "Redelonghi", 16);

    let (name1, lname1, age1) = fmem_data1;
    let (name2, lname2, age2) = fmem_data2;
    println!("Name1: {name1}, last name1: {lname1}, age1: {age1}");
    println!("Name2: {name2}, last name2: {lname2}, age2: {age2}");

    let name3  = fmem_data3.0;
    let lname3 = fmem_data3.1;
    let age3   = fmem_data3.2;
    println!("Name3: {name3}, last name3: {lname3}, age2: {age3}");

    println!("---");

}

