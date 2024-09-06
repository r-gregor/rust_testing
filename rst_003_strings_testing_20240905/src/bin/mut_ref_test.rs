/*
 * src/bin/mut_ref_test.rs
 * 20240905
 */

/* main */
fn main() {
	let mut s = String::from("Gregor");
	println!("Original:   {s}");

	let s1 = &mut s;
	s1.push_str(" Redelonghi");
	println!("Result1:    {s}");

	let s2 = &mut s;
	s2.push_str(", Valvasorjeva ulica 5, 1000 Ljubljana");
	println!("Result2:    {s}");

	println!("---");

	let s2 = "PoPoKaTePeTl";
	get_string_info(s2);
	print_middletwo(s2);

	print_row_of_n_nums(10);

    // 20240906_en
    println!("---");
    // let mixed: (String, u32, bool) = ("Hello", 42, true);
    //                                    |
    //                                    +-- expected "String" found "&str" must be converted
    //                                        to "String" with .to_string() method
    let mixed1: (String, u32, bool) = ("Hello".to_string(), 42, true);
    println!("mixed1 tuple = {:?}", mixed1);

    // but OK if:
    //            Ok!                  Ok!
    let mixed2: (&str, u32, bool) = ("Hello", 42, true);
    println!("mixed2 tuple = {:?}", mixed2);


} /* end main */


fn get_string_info(s: &str) {
	println!("Value:      {}", s);
	println!("String len: {}", s.len());
}

fn print_middletwo(s: &str) {
	let middle = s.len() / 2;
	print!  ("Middle_Two: {}",&s[..middle-1]);
	print!  (" [{}] ", &s[middle-1..middle+1]);
	print!  ("{}",&s[middle+1..]);
}

fn print_row_of_n_nums(n: u32) {
	let mut i: u32 = 0;
	print!("\n'Row of {n} nums: ");

	while i != n - 1 {
		print!("{i}, ");
		i += 1;
	}
	print!("{}'\n", i + 1);
}

