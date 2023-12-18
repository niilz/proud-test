extern crate proud;
use proud::{generate_structs, ProtoBuf};

#[derive(ProtoBuf)]
struct AnyStruct;

#[derive(Debug)]
struct ProtoField {
    name: String,
    typ: String,
}

#[derive(ProtoBuf)]
struct PersonRs {
    name: String,
    role: String,
    age: u32,
    is_coder: bool,
    degree: Option<String>,
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
        degree: None,
    };

    let proto_buf = person.to_proto();
    println!("{proto_buf}");
}
