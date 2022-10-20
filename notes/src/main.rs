mod r01basic;
mod r02ownership;
mod r03structs;

fn main() {
    println!("\n##########BASICS##########");
    r01basic::immutability::immutable();
    r01basic::immutability::shadowing();
    r01basic::data_types::data_types();
    r01basic::functions::add(3, 3);
    r01basic::functions::substract(3, 3);
    r01basic::control_flow::flow();
    println!("\n##########OWNERSHIP##########");
    r02ownership::ownership::ownership();
    println!("\n##########STRUCTS##########");
    r03structs::structs::structs();
}
