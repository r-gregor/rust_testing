/*
 * src/main.rs
 * chapter 5.3: Method Syntax
 * 20240905
 */

#[derive(Debug)]
struct Rectangle {
	width:  u32,
	height: u32,
}

/* method */
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

/* main */
fn main() {

	let rect1 = Rectangle {
		width:  30,
		height: 50,
	};

	println!(
		"The area of the rectangle is {} sqauare pixels.",
		rect1.area()
	);
}
