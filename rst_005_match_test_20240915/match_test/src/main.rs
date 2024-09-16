pub mod match_test;

fn main() {
	// let ages: [u32; 10] = [5, 56, 99, 14, 85, 19, 77, 18, 22, 56];
	let ages = vec![5, 56, 99, 14, 85, 19, 77, 18, 22, 56];
	for age in ages.iter() {
		print!("Age: {:2} --> ", age);
		match_test::age_test(age);
	}
}
