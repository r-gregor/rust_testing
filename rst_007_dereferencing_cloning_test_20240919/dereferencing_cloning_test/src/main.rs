fn main() {
    let mut test = String::new();
    let mut x: u32 = 0;

    /* primitives do not need to be cloned
     * as they implement tje 'copy' trait
     */
    println!("Output 1:");
    for i in 1..=5 {
        let mut y = x; // let mut y = x.clone() -- not necessary
        y += i;
        println!("{y}");
    }

    println!("---");

    /* x referenced so it changes
     * x refetence must be dereferenced to change
     */
    println!("Output 2:");
    for i in 1..=5 {
        *(&mut x) += i;
        println!("{x}");
    }

    println!("---");

    /* same as previous except without references */
    println!("Output 3:");
    for i in 1..=5 {
        x += i;
        println!("{x}");
    }

    println!("---");

    println!("Output 4:");
    doit(&mut x, &mut test);

    println!("---");

    /* tuples are primitives so they
     * implement 'copy' trait
     */
    println!("Output 5:");
    let tup1: (u32, u32) = (15, 12);
    let tup2 = tup1;
    println!("{:?} {:?}", tup1, tup2);

    println!("---");

    /* referencing a string */
    println!("Output 6:");

    /* error
    {
        let original = String::from("Gregor");
        let name2 = original;
        println!("{original}, {name2}");
    }
    */

    {
        let original = String::from("Gregor");
        let name2 = original.clone();
        println!("{original}, {name2}");
    }

    println!("---");

    {
        let mut original = String::from("Gregor");
        print!("{}, ", original);
        let name2 = &mut original;
        println!("{}", name2);
        // name2 = String::from("Tadeja"); // error
        *name2 = String::from("Tadeja"); // OK
        print!("{}, ", name2);
        println!("{}", original);
    }
}

/* x reference in function call must be dereferenced
 * x referenced so it changes
 */
pub fn doit(x: &mut u32, test: &mut String) {
    for i in 1..=5 {
        test.push('a');
        println!("{test}");
        *x += i;
        println!("{x}");
    }
}

