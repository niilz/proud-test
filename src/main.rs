extern crate proud;
use proud::{generate_structs, ProtoBuf};

#[derive(ProtoBuf)]
struct AnyStruct;

#[derive(ProtoBuf)]
struct PersonRs {
    name: String,
}

fn main() {
    // Generate structs from proto-file
    generate_structs!("person.proto");

    // Generate proto data from rust-struct
    let person = PersonRs {
        name: "person".to_string(),
    };

    let proto_buf = person.to_proto();
    println!("{proto_buf}");
}
