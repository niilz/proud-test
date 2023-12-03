extern crate proud;
use proud::{generate_structs, ProtoBuf};

#[derive(ProtoBuf)]
struct AnyStruct;

#[derive(ProtoBuf)]
struct PersonRs {
    name: String,
    role: String,
    age: u32,
    is_coder: bool,
}

fn main() {
    // Generate structs from proto-file
    generate_structs!("person.proto");

    // Generate proto data from rust-struct
    let person = PersonRs {
        name: "person".to_string(),
        role: "developer".to_string(),
        age: 40,
        is_coder: true,
    };

    let proto_buf = person.to_proto();
    println!("{proto_buf:?}");
}
