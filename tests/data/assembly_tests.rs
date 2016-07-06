use std::fs;
use kwon_vm::data::assembly::*;

#[test]
fn create_assembly_with_strings() {
    let mut assembly = Assembly::new();

    assembly.add_string("Some other string.");

    let string = "This is just a tribute and you got to believe me.";
    let string_utf8 = string.as_bytes();

    assembly.add_string("And another.");

    let string_address = assembly.add_string(string);

    assembly.write_to("./test_assembly.kas");

    let assembly2 = Assembly::load_from("./test_assembly.kas");

    assert_eq!(assembly2.get_string(string_address).unwrap(), string_utf8);

    fs::remove_file("./test_assembly.kas").unwrap();
}
