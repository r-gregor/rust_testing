/* src/struct_test_print.rs
 * chapter 5.2: An Example Program Using Structs
 *  pretty-print the struct
 * 20240905
 */

/* structuring data with struct */
#[derive(Debug)] /* after error 2 */
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main() {

    /* structuring data with struct */
    let rect_st = Rectangle {
        width:  30,
        height: 50,
    };

    let _area = rect_st.width * rect_st.height;

    /*
     * command ':r !rustc src/bin/struct_test_print src/struct_test_print.rs'
     * error 1:
        = help: the trait `std::fmt::Display` is not implemented for `Rectangle` ...
     * ---
     * error 2:
     * = help: the trait `Debug` is not implemented for `Rectangle`
    */
    // println!("rect_st is {rect_st}"); /* error 1 */
    println!("Output '{}':", "{:?}");
    println!("rect_st is{rect_st:?}"); /* error 2 */
    println!("---");
    println!("Output '{}':", "{:#?}");
    println!("rect_st is{rect_st:#?}"); /* error 2 */
}

