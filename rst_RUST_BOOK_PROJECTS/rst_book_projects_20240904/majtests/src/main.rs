/*
 * testing: String opertions
 * 20240904
 */

use std::io;
use std::io::Write;

fn main() {
    let name1="Gregor Redelonghi";
    let mut s11 = String::from(name1);
    println!("s11:                     '{s11}'");
    let s12 = String::from(", Valvasorjeva ulica 5, 1000 Ljubljana");
    println!("s12:                     '{s12}'");
    s11 += &s12;
    println!("s11 after 's11 += &s12': '{s11}'");

    println!("---");

    let name2 = "Gregor Redelonghi";
    let mut s21 = String::from(name2);
    println!("s21:                            '{s21}'");
    let s22 = String::from(", Valvasorjeva ulica 5, 1000 Ljubljana");
    println!("s22:                            '{s22}'");
    s21.push_str(&s22);
    println!("s21 after 's21.push_str(&s22)': '{s21}'");
    s21.push_str(", 0038640885560, jb3.z0rg@gmail.com");
    println!("s21 after 's21.push_str(\"..\")': '{s21}'");

    println!("---");

    /* slices */
    println!("Slices ...");
    /*
    let s31 = String::from("ena dva tri štiri pet šest sede osem devet deset");
    println!("Original:                                '{}'", s31);
    let start =  5;
    let stop  = 17;
    let slc31 = &s31[start..stop];
    println!("Slice from {start}-th to {stop}-th byte: '{slc31}'");

    println!("---");
    */
    let s32 = String::from("ena dva tri štiri pet šest sedem osem devet deset");
    let start =  5;
    let stop  = 27;
    let slc32 = &s32[start..stop];
    // println!("Test: start: {start}, dtop: {stop}, length: {}", slc32.len());
    println!("'{s32}' -- ORIGINAL");
    print!("'");
    io::stdout().flush().unwrap(); // continue on same line
    for _i in 0..start {
        print!(".");
    }
    print!("{slc32}");
    for _j in stop..s32.len() {
        print!(".");
    }
    print!("' -- Slice from {start}-th to {stop}-th byte");
}

