fn main() {
    let mut v1: Vec<i32> = Vec::new();
    
    println!("{:>5} | {:>5} | {:>5}", "#", "size", "cap");
    println!("---------------------");
    for i in 1..10001 {
        v1.push(1);
        println!("{:5} | {:5} | {:5}", i, v1.len(), v1.capacity());
    }
}


