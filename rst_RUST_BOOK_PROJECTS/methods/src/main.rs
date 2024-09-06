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

/* methods */
impl Rectangle {
	/* method area */
	fn area(&self) -> u32 {
		self.width * self.height
	}

	/* method can_hold */
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	/* method that returns WxH string */
	fn hxv(&self) -> String {
		format!("{}x{}", self.width, self.height)
	}
}

/* main */
fn main() {

	let rect1 = Rectangle {
		width:  30,
		height: 50,
	};

	let rect2 = Rectangle {
		width:  10,
		height: 40,
	};

	let rect3 = Rectangle {
		width:  60,
		height: 45,
	};

	println!(
		"The area of the rectangle is {} sqauare pixels.",
		rect1.area()
	);
	
	println!("---");

	println!(
		"Can rect1({}) hold rect2({})? {}",
		rect1.hxv(),
		rect2.hxv(),
		rect1.can_hold(&rect2)
	);

	println!(
		"Can rect1({}) hold rect3({})? {}",
		rect1.hxv(),
		rect3.hxv(),
		rect1.can_hold(&rect3)
	);

} /* end main */

