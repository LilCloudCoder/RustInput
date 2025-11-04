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
            let input = self.read_line();
            let trimmed = input.trim();
            match trimmed.parse::<T>() {
                Ok(val) => return val,
                Err(_) => println!("{}", invalid_msg),
            }
        }
    }

    /// Parse any integer type (i8..i128, u8..u128, isize, usize)
    pub fn int<T: FromStr>(&self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.parse_loop::<T>("Invalid input, enter a valid integer.")
    }

    /// Parse any float type (f32, f64)
    pub fn float<T: FromStr>(&self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.parse_loop::<T>("Invalid input, enter a valid number.")
    }

    /// Read a trimmed string
    pub fn string(&self) -> String {
        self.read_line().trim().to_string()
    }

    /// Read a single non-whitespace character
    pub fn char(&self) -> char {
        loop {
            let s = self.string();
            if let Some(c) = s.chars().next() {
                return c;
            }
            println!("Please enter at least one character.");
        }
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