mod basic;
mod ownership;
mod structs;

fn main() {
    basic::immutability::immutable();
    basic::immutability::shadowing();
    basic::data_types::data_types();
    basic::functions::add(3, 3);
    basic::functions::substract(3, 3);
    basic::control_flow::flow();
    ownership::ownership::ownership();
    structs::structs::structs();
}
