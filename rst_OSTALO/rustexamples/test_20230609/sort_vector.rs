fn main() {

    let mut vec = vec![23.12, 3.44, 5.55, 34.90, 2.0];
    vec.sort_by(|x, y| x.partial_cmp(y).unwrap());
    println!("Sorted: {:?}", vec);
}
