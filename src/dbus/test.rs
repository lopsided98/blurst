use super::*;

#[test]
fn ref_arg_cast_string() {
    let val = "test".to_owned();
    assert_eq!(String::ref_arg_cast(&val).unwrap(), val);
}

#[test]
fn ref_arg_cast_string_to_u64() {
    let val = "test".to_owned();
    u64::ref_arg_cast(&val).unwrap_err();
}

#[test]
fn ref_arg_cast_vec_u8() {
    let ref_arg = vec![0u8, 1u8];
    assert_eq!(Vec::<u8>::ref_arg_cast(&ref_arg).unwrap(), ref_arg);
}
//
// #[test]
// fn ref_arg_cast_vec_variant_to_variant() {
//     let ref_arg = vec![dbus::arg::Variant(0u8)];
//     assert_eq!(
//         Vec::<dbus::arg::Variant<u8>>::ref_arg_cast(&ref_arg).unwrap(),
//         ref_arg
//     );
// }

#[test]
fn ref_arg_cast_vec_variant_to_u8() {
    let ref_arg = vec![dbus::arg::Variant(0u8)];
    assert_eq!(Vec::<u8>::ref_arg_cast(&ref_arg).unwrap(), vec![0u8]);
}

#[test]
fn ref_arg_cast_hash_map() {
    let mut ref_arg = HashMap::new();
    ref_arg.insert("key".to_owned(), 0u8);
    assert_eq!(HashMap::ref_arg_cast(&ref_arg).unwrap(), ref_arg);
}
//
// #[test]
// fn ref_arg_cast_hash_map_variant() {
//     let mut ref_arg = HashMap::new();
//     ref_arg.insert("key".to_owned(), dbus::arg::Variant(0u8));
//     assert_eq!(HashMap::ref_arg_cast(&ref_arg).unwrap(), ref_arg);
// }

#[test]
fn ref_arg_cast_hash_map_variant_to_u8() {
    let mut ref_arg = HashMap::new();
    ref_arg.insert("key".to_owned(), dbus::arg::Variant(0u8));
    let mut expected = HashMap::new();
    expected.insert("key".to_owned(), 0u8);
    assert_eq!(HashMap::ref_arg_cast(&ref_arg).unwrap(), expected);
}

#[test]
fn ref_arg_cast_hash_map_variant_to_vec_u8() {
    let mut ref_arg = HashMap::new();
    ref_arg.insert("key".to_owned(), dbus::arg::Variant(vec![0u8, 1u8]));
    let mut expected = HashMap::new();
    expected.insert("key".to_owned(), vec![0u8, 1u8]);
    assert_eq!(HashMap::ref_arg_cast(&ref_arg).unwrap(), expected);
}
