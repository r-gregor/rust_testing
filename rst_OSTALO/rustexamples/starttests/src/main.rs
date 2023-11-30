
// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("20230912:");
    
    let x :u32 = 44;
    let name = "Gregor Redelonghi";
    println!("Name: {name} and a number: {x}");

    println!("---");

    println!("MIN of i32: {}", i32::MIN);
    println!("MAX of i32: {}", i32::MAX);
    println!("MIN of u32: {}", u32::MIN);
    println!("MAX of u32: {}", u32::MAX);
    println!("MIN of i64: {}", i64::MIN);
    println!("MAX of i64: {}", i64::MAX);
    println!("MIN of u64: {}", u64::MIN);
    println!("MAX of u64: {}", u64::MAX);
    println!("MIN of f32: {}", f32::MIN);
    println!("MAX of f32: {}", f32::MAX);
    println!("MIN of usize: {}", usize::MIN);
    println!("MAX of usize: {}", usize::MAX);
    println!("MIN of f64: {}", f64::MIN);
    println!("MAX of f64: {}", f64::MAX);

    println!("---");

    let num1: f32 = 1.111111111111111;
    println!("f32 (sum): {}", num1 + 0.111111111111111);

    let num2: f64 = 1.111111111111111;
    println!("f64 (sum): {}", num2 + 0.111111111111111);
    println!("---"); // ------------------------------------------------------------------------
    let n1 :u32 = 5;
    let n2 :u32 = 4;
    println!("{n1} + {n2} = {}", n1 + n2);
    println!("{n1} - {n2} = {}", n1 - n2);
    println!("{n1} * {n2} = {}", n1 * n2);
    println!("{n1} / {n2} = {}", n1 / n2);
    println!("{n1} % {n2} = {}", n1 % n2);
    println!("---"); // ------------------------------------------------------------------------
    /* NOT working: int_num is NOT mutable!
    let int_num: i32 = 1_000_000;     // NOT mutable
    let int_num_ptr = int_num;        // NOT mutable
    println!("Value of int_num:   {}", *int_num_ptr);
    println!("Address of int_num: {:p}", int_num_ptr);

    *int_num_ptr += 1;
    displ_int_info(int_num_ptr);
    println!("---"); // ------------------------------------------------------------------------
    */

    let mut int_num: i32 = 1_000_000; // change: MUTABLE!
    let int_num_ptr = &mut int_num;   // change: MUTABLE
    println!("Value of int_num:   {}", *int_num_ptr);
    println!("Address of int_num: {:p}", int_num_ptr);

    *int_num_ptr += 1;
    displ_int_info(int_num_ptr);
    println!("---"); // ------------------------------------------------------------------------


} // end MAIN


/* displaying value and reference of a int pointer */
fn displ_int_info(ptr: &i32) {
    println!("Value of int_num:   {}", *ptr);
    println!("Address of int_num: {:p}", ptr);
}

