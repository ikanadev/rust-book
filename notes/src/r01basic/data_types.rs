pub fn data_types() {
    /* Rust provides 4 scalar types:
     * INTEGERS: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize those are 8, 16,
     * 32, 64 and 128 bit signed and unsigned numbers. isize and usize uses bits according to the
     * machine's architecture (32 or 64 bits)
     * A number can use a _ (underscore) separator. EX 1_024
     * Rust default is i32
     * Other base numbers has prefixes
     * Hex: 0xff
     * Oct: 0o66
     * Bin: 0b1011
     * Also you can add a suffix to a number to change its type
     */
    let a = 2i16; // a is a i16 type
    println!("a is {}", a);
    let bin = 0b10110;
    println!("bin is {}", bin);
    /*
     * FLOATING_POINT
     * f32 and f64
     * Any number who has a dot it will become a floating 32 by default
     */
    let pi = 3.14;
    println!("pi is {}", pi);

    /*
     * BOOLEANS: either true or false
     */
    /*
     * CHARACTER
     * Represents a char, its surrounded by single quotes
     */
    let a_char = 'a';
    println!("a char is {}", a_char);
    /*
     * COUMPOUND TYPES: multiple types in one
     * TUPLE: comma separated values, we can access to a tuple element by destructuring or
     * accessing by index
     * ARRAY: fixed serie of values of the same type
     * let arr: [i32: 5]   is an 5 length array of i32 elements
     * let default = [4; 5]   create an array of 5 elements where each element is 4
     */
    let tuple: (bool, u32, i8) = (false, 22, 8);
    let (a, b, c) = tuple;
    println!("Tuple values are: {}, {}, {}", a, b, c);
    println!("Tuple 1st value is: {}", tuple.0);
}
