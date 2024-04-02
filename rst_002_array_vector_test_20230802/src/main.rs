/*
 * 002
 * 20230802
 */

fn main() {
	let mut nums: [i32; 5] = [1, 2, 3, 4, 5]; // arrays are fixed size!!
	// println!("{:?}", nums);
	display_collection(&nums);

	nums[2] = 8;
	display_collection(&nums);
	
	// Vectors are growable/shrinkable !!
	// let mut vec_nums: Vec<i32> = Vec::from([1, 2, 3, 4, 5]); // vec! == Vec::from !!
	let mut vec_nums: Vec<i32> = vec![1, 2, 3, 4, 5];
	display_collection(&vec_nums);

	vec_nums.push(22);
	display_collection(&vec_nums);

	vec_nums.pop();
	print!("\nAfter pop():\n");
	display_collection(&vec_nums);

	println!("Sum of all numbers in an array: {}", sum_nums(&nums));
	println!("Sum of all numbers in a vector: {}", sum_nums(&vec_nums));


} // end main

// fn display_collection(vect: &Vec<i32>) { // only works with Vectors!!
fn display_collection(collection: &[i32]) {
	println!("{:?}, length: {}", collection, collection.len());
}

// fn sum_nums(nums: &Vec<i32>) -> i32 { // NOT workig for both array and Vec because type is Vec<> !!
fn sum_nums(nums: &[i32]) -> i32 {       // WORKS because type is a SLICE!

	let mut sum: i32 = 0;
	for n in nums {
		sum += n;
	}
	return sum;
}

