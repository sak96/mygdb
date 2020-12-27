use self::interpreter::Interpreter;

pub struct Debugger {
    binary: String,
    interpreter: Interpreter,
}

impl Debugger {
    pub fn new(binary: &str) -> Self {
        Self {
            binary: binary.to_string(),
            interpreter: Interpreter::new(&format!("{}> ", env!("CARGO_PKG_NAME"))),
        }
    }
    pub fn run(&mut self) {
        while let Ok(line) = self.interpreter.read_line() {
            println!("perform '{}' on '{}'", line, self.binary);
        }
    }
}

pub(super) mod interpreter {
    use rustyline::{error::ReadlineError, Editor};

    pub struct Interpreter {
        prompt: String,
        reader: Editor<()>,
    }

    impl Interpreter {
        pub fn new(prompt: &str) -> Self {
            let reader = Editor::<()>::new();
            Self {
                prompt: prompt.to_string(),
                reader,
            }
        }

        pub fn read_line(&mut self) -> Result<String, ReadlineError> {
            match self.reader.readline(self.prompt.as_str()) {
                Ok(line) => {
                    self.reader.add_history_entry(&line);
                    Ok(line)
                }
                err => err,
            }
        }
    }
}
