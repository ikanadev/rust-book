pub fn ownership() {
    // 3 rules of ownership.
    // 1. Each value has a variable which is its owner
    // 2. Each value only can have 1 owner
    // 3. When the owner goes out of scope, the value is dropped
    // The next code will not work because we are not copying the name, we are passing ownership to
    // name_copy
    /*
    let name = String::from("Kevv");
    let name_copy = name;
    println!("name: {}, name_copy: {}", name, name_copy);
    */
    // next code works because, values which size are known in runtime can be copied.
    let x = 1;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // we can use clone to create a copy
    let name = String::from("Kevv");
    let name_copy = name.clone();
    println!("name: {}, name_copy: {}", name, name_copy);
    // we can have many immutable references to the variable, but just one mutable reference a
    // mutable reference is defined by: &mut my_variable
    let last_name = String::from("VV");
    let ref1 = &last_name;
    let ref2 = &last_name;

    println!("ref1: {}, ref2: {}", ref1, ref2);
}
