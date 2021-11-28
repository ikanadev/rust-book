pub fn immutable() {
    println!("Immutability");
    // Variables are immutable by default so this will not work
    // let a = 5;
    // a = 6;
    // but this will work
    let mut number = 5;
    println!("{}", number);
    number = 6;
    println!("{}", number);
    // differences between an immutable variable and a const are:
    // const is always immutable
    // const need the variable type
    // const is not the result of a function
    const HOUR_IN_SECONDS: i32 = 60 * 60;
    println!("Const is: {}", HOUR_IN_SECONDS);
}

pub fn shadowing() {
    println!("Shadowing");
    /* we can redeclare a variable with a new type, this is block scoped, we can even change the
     * variable type
     */
    let number = 1;
    let number = number + 1;
    {
        let number = "dos";
        println!("Number is: {}", number); // dos
    }
    println!("Number is: {}", number); // 2
}
