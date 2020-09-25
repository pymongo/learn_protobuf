extern crate learn_protobuf;

use learn_protobuf::protos::user::User;
use protobuf::Message;

fn main() {
    let user_1 = User {
        id: 0,
        name: "Mike".to_string(),
        ..Default::default()
    };
    let user_1_encoded = user_1.write_to_bytes().unwrap();

    // Decode
    let user_2 = protobuf::parse_from_bytes::<User>(&user_1_encoded).unwrap();
    assert_eq!(user_1.id, user_2.id);
    assert_eq!(user_1.name, user_2.name);
}
