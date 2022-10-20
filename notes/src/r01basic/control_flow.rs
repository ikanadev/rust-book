pub fn flow() {
    /*
     * If values doesn't need parens
     * if a < 5 { ... }
     * if is an expresion so we can use it to assign it to a variable
     */
    let a = if true { 5 } else { 4 }; // a is 5
    println!("a is {}", a);
    /*
     * LOOPS:
     * loop runs a block forever unless we break it, we can label a loop
     */
    let mut i = 1;
    'loop_one: loop {
        i = i + 1;
        if i > 5 {
            println!("Loop breaks at 5");
            break 'loop_one;
        }
    }
    // also we can break with a value to assign it
    let mut counter = 0;
    let ten = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("ten is {}", ten);
    /*
     * WHILE executes a block while the condition is true
     */
    let mut counter = 3;
    while counter != 0 {
        counter -= 1;
    }
    println!("counter is {}", counter);
    /*
     * FOR needs no presentation
     */
    let a = [10, 20];
    for (i, element) in a.iter().enumerate() {
        println!("the value of {} is: {}", i, element);
    }
}
