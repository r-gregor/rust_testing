fn main() {
    let mut myvec: Vec<i32> = Vec::new();
    
    println!("{:>5} | {:>5} | {:>5}", "#", "size", "cap");
    println!("-------------------");
    for i in 1..10001 {
        myvec.push(1);
        println!("{:5} | {:5} | {:5}", i, myvec.len(), myvec.capacity());
    }
}


