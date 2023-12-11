use crate::Res;

pub enum Options {
    Passwords
}

impl Options {
    pub fn all() -> Vec<Options> {
        vec![Options::Passwords]
    }
    
    pub fn run(&self) -> Res {
        println!("{}", self.to_string());
        Ok(())
    }

    pub fn to_string(&self) -> String {
        match self {
            Options::Passwords => String::from("Passwords"),
        }
    }
}
