extern crate proud;

use proud::{generate_structs, ProtoBuf};

fn main() {
    #[derive(ProtoBuf)]
    struct AnyStruct;

    let path = generate_structs!("person.proto");
    println!("{path}");
}
