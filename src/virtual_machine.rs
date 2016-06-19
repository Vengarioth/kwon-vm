use assembly::Assembly;
use runtime::interpreter::Interpreter;

pub struct VirtualMachine {
    assembly: Assembly,
    interpreter: Interpreter
}

impl VirtualMachine {
    pub fn new(assembly: Assembly) -> Box<VirtualMachine> {

        let interpreter = Interpreter::new();

        return Box::new(VirtualMachine{
            assembly: assembly,
            interpreter: interpreter
        });
    }

    pub fn run(&mut self) {
        loop {
            self.interpreter.fetch(&self.assembly);
            self.interpreter.decode();
            if !self.interpreter.execute() {
                break;
            }
        }
    }
}
