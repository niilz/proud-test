extern crate proud;

use proud::{generate_structs, ProtoBuf};

fn main() {
    #[derive(ProtoBuf)]
    struct AnyStruct;

    generate_structs!("person.proto");

    /*
    let person = Person {
        name: "Foo".to_string();
    };
    */
}
