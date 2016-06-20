extern crate kwon_vm;

use kwon_vm::*;

fn main() {

    println!("creating new assembly");
    let mut assembly = Assembly::new();

    println!("adding a string");
    assembly.add_string("this is just a tribute");
    assembly.add_string("you got to believe me");
    assembly.add_string("and i wish you were there");
    assembly.add_string("just a matter of opinion");

    println!("write assembly to disk");
    assembly.write_to("/Users/vengarioth/Workspace/kwon/test_assembly.kas");

    println!("load assembly from disk");
    let assembly2 = Assembly::load_from("/Users/vengarioth/Workspace/kwon/test_assembly.kas");

}
