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

    /// Parse a bool (y/yes/true/1 = true, n/no/false/0 = false)
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

    /// Optional parse: empty input returns None; otherwise parse T (retry on invalid)
    pub fn optional<T: FromStr>(&self) -> Option<T>
    where
        T::Err: std::fmt::Debug,
    {
        loop {
            let input = self.read_line();
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return None;
            }
            match trimmed.parse::<T>() {
                Ok(v) => return Some(v),
                Err(_) => println!("Invalid input, try again or press Enter to skip."),
            }
        }
    }

    /// Parse with default: empty input returns provided default; otherwise parse T (retry on invalid)
    pub fn default<T: FromStr>(&self, default: T) -> T
    where
        T::Err: std::fmt::Debug,
        T: Clone,
    {
        loop {
            let input = self.read_line();
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return default.clone();
            }
            match trimmed.parse::<T>() {
                Ok(v) => return v,
                Err(_) => println!("Invalid input, try again or press Enter for default."),
            }
        }
    }

    /// Restrict string input to a set of allowed choices (case-insensitive).
    /// Returns the canonical choice as provided in `allowed`.
    pub fn choices<'a>(&self, allowed: &[&'a str]) -> String {
        let lowered: Vec<String> = allowed.iter().map(|s| s.to_lowercase()).collect();
        loop {
            let v = self.string();
            let v_low = v.to_lowercase();
            if let Some((idx, _)) = lowered.iter().enumerate().find(|(_, s)| *s == &v_low) {
                return allowed[idx].to_string();
            }
            println!("Invalid choice. Allowed: {}", allowed.join(", "));
        }
    }
}

// Helper function
pub fn input(prompt: &str) -> Input {
    Input::new(prompt)
}