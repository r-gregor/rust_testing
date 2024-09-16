pub fn age_test(age: &u32) {

	/* BETTER */
	match age {
		0  ..   7 => println!("An infant."),
		7  ..= 17 => println!("Not yet grownup."),
		18 ..= 65 => println!("An adoult, but not yet senior."),
		61 ..  86 => println!("Senior. Enjoy the rest of your life."),
		86 ..     => println!("You're still alive? Welcome to the Dinosaurs' club!"),
	}

	/* IF ACTIONS IN CODE BLOCKS ...
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
	*/
}
