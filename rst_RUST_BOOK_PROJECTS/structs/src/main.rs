/*
 * chapter 5.2: An Example Program Using Structs
 * 20240905
 */

/* structuring data with struct */
struct Rectangle {
    width:  u32,
    height: u32,
}

/* main */
fn main() {

    /* no structuring data */
    let width1  = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} sqare pixels. -- without structuring data",
        area(width1, height1)
    );

    /* structuring data with tuple */
    let rect_tp = (30, 50);

    println!(
        "The area of the rectangle is {} sqare pixels. -- with data as tuple",
        area_tp(rect_tp)
    );

    /* structuring data with struct */
    let rect_st = Rectangle {
        width:  30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} sqare pixels. -- with data as struct",
        area_st(&rect_st)
    );

} /* end main */

/* no structuring data */
fn area(width: u32, height: u32) -> u32 {
    width * height
}

/* structuring data with tuple */
fn area_tp(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/* structuring data with struct */
fn area_st(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

