pub struct Debugger {
    binary: String,
}

impl Debugger {
    pub fn new(binary: &str) -> Self {
        Self {
            binary: binary.to_string(),
        }
    }
    pub fn run(&mut self) {
        println!("debugging the program {}", self.binary);
    }
}
