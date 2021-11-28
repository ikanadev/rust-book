/*
 * Rust uses snack case for function names.
 * We can call a function even before its definition
 * You need to declare the type of each parameter
 * fn add(x: i32, y: i32) {}
 * for return values we need to add an arrow and declare the type of the returning value:
 */
pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
// if we ommit the last semicolon, it will be consideres as return value
pub fn substract(x: i32, y: i32) -> i32 {
    x - y
}
