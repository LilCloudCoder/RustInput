use std::io::{self, Write};
use std::str::FromStr;

pub struct Input {
    pub prompt: String,
}

impl Input {
    pub fn new(prompt: &str) -> Self {
        Self { prompt: prompt.to_string() }
    }

    // Internal: print prompt and read a line
    fn read_line(&self) -> String {
        print!("{}", self.prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        input
    }

    // Internal: repeatedly parse using FromStr with custom error message
    fn parse_loop<T: FromStr>(&self, invalid_msg: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        loop {
            print!("{}", self.prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            match trimmed.parse::<T>() {
                Ok(val) => return val,
                Err(_) => println!("Invalid input, enter a valid number."),
            }
        }
    }

    pub fn float<T: FromStr>(&self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.int::<T>()
    }

    pub fn string(&self) -> String {
        print!("{}", self.prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn bool(&self) -> bool {
        loop {
            let val = self.string();
            match val.to_lowercase().as_str() {
                "yes" | "y" | "true" | "1" => return true,
                "no" | "n" | "false" | "0" => return false,
                _ => println!("Please enter yes/no or true/false"),
            }
        }
    }
}

// Helper function
pub fn input(prompt: &str) -> Input {
    Input::new(prompt)
}