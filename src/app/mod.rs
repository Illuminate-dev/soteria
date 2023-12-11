mod options;

pub struct App {
    args: crate::args::Args,
}

impl App {
    pub fn new(args: crate::args::Args) -> Self {
        Self {
            args
        }
    }

    pub fn banner(&self) {
        //    _____       __            _      
        //   / ___/____  / /____  _____(_)___ _
        //   \__ \/ __ \/ __/ _ \/ ___/ / __ `/
        //  ___/ / /_/ / /_/  __/ /  / / /_/ / 
        // /____/\____/\__/\___/_/  /_/\__,_/  
        println!("    _____       __            _      ");
        println!("   / ___/____  / /____  _____(_)___ _");
        println!("   \\__ \\/ __ \\/ __/ _ \\/ ___/ / __ `/");
        println!("  ___/ / /_/ / /_/  __/ /  / / /_/ / ");
        println!(" /____/\\____/\\__/\\___/_/  /_/\\__,_/");
    }

    pub fn main_screen(&self) {
        self.banner();
        println!("");
        println!("");

        let opts: Vec<(usize, options::Options)> = options::Options::all().into_iter().enumerate().collect(); 

        for (i, mode) in &opts {
            println!("[{}]: {}", i+1, mode.to_string());
        }
        
        print!("Choose a number: ");
        let choice = crate::input();
        let num = match choice.parse::<u32>() {
            Err(e) => panic!("Panic with error {e}"),
            Ok(n) => n
        };

        if num == 0 || num > opts.len() as u32 {
            panic!("invalid number");
        }

        opts[num as usize-1].1.run().expect("error running"); 


    }

    
}