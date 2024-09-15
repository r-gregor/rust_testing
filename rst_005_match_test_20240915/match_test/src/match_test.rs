pub fn age_test(age: u32) {
	match age {
		0 ..= 17 => {
			println!("Not yet grownup.");
		}

		18 ..=65 => {
			println!("An adoult, but not yet senior.");
		}

		61 .. 86 => {
			println!("Senior. Enjoy the rest of your life.");
		}

		86 .. => {
			println!("You're still alive? Welcome to the Dinosaurs' club!");
		}
	}
}
