// A reference it's like a pointer that is guaranteed to be valid

pub fn references() {
    println!("####References####");
    let s1 = String::from("hello");
    // To get a reference, we use &
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // You can have only one mutable reference to a particular piece of data in a particular scope
    let mut s2 = String::from("hello");
    {
        let r1 = &mut s2;
        r1.push_str(", world");
    }
    let r2 = &mut s2;
    r2.push_str(", world");
    println!("{}", s2); // end of r2 scope
    // now we can use s2
    println!("{}", s2);
    

    // When a mutable refence is created we can't mutate the original value
    let mut x = 5;
    let r = &mut x;
    *r += 1;
    println!("{r}"); // last use of r

    x += 1; // allowed again
    println!("{x}"); // last use of x
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
