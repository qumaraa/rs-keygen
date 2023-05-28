//! # KeyGen v0.1.1 (beta)
//! Simple customizable and convenient Key Generator. (in development).
//! 
//! By using `KeyGen` you can generate the keys of any size,enable/disable symbols and numbers and much more that will appear in the future.
//! # Example
//! ```
//! fn main() { 
//!     let mut kg = KeyGen::new()
//!         .length(10) // by default `length == 8`
//!         .symbols(true) // by default `symbols == false`
//!         .numbers(true); // by default `numbers == false`
//!     let gen_key = kg.gen_one().unwrap();
//!    
//!     println!("Generated key: {gen_key}")
//! }
//! 
//! 
//! ```
use rand::{
    thread_rng, 
    Rng
};

/// [`KeyGen`] - struct contains the main logic of Key Generator
pub struct KeyGen {
    /// `length` - key length
    length: usize,
    /// `symbols` - a flag that can
    /// enable/disable symbols to key gen.
    symbols: bool,
    /// `numbers` - a flag that can
    /// enable/disable numbers to key gen.
    numbers: bool,
    /// `uppercase` - a flag that can
    /// enable/disable uppercase symbols to key gen.
    uppercase: bool,
    /// `lowercase` - a flag that can
    /// enable/disable lowercase symbols to key gen.
    lowercase: bool,
}

/// [`KeyGen`] implementation
impl KeyGen {
    /// constructor which returns `Self` with default parameters.
    pub fn new() -> Self {
        KeyGen {
            /// `Default` length of `key` = 8
            length: 0,
            // `Default` `symbols` state = false
            symbols: false,
            // `Default` `numbers` state = false
            numbers: false,
            // `Default` `uppercase` state = false
            uppercase: false,
            // `Default` `lowercase` state = false
            lowercase: false,
        }
    }
    /// changes the value  of `length` to value from parameter
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
    /// changes the boolean value (state) of `symbols` to value from parameter
    pub fn symbols(mut self, symbols: bool) -> Self {
        self.symbols = symbols;
        self
    }
    /// changes the boolean value (state)  of `symbols` to value from parameter
    pub fn numbers(mut self, numbers: bool) -> Self {
        self.numbers = numbers;
        self
    }
    /// changes the boolean value (state)  of `uppercase` to value from parameter
    pub fn uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }
    /// changes the boolean value (state)  of `lowercase` to value from parameter
    pub fn lowercasecase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }
    /// generates the random chars and collects 
    /// them into `String` and returns as `Result<T,E>`
    #[inline]
    pub fn gen_one(&mut self) -> Result<String, &'static str> {
        if self.length == 0 {
            return Err("length of the password should be more than `0`");
        } 
        let mut rng = thread_rng();
        let mut chars = String::new();
        // if `self.numbers == true` push the numbers into `chars` (String)
        if self.numbers {
            chars.push_str("1234567890");
        }
        // if `self.symbols == true` push the symbols into `chars` (String)
        if self.symbols {
            chars.push_str("!@#$%^&*()-+/[].?:");
        }
        // if `self.uppercase == true` push the uppercase letters into `chars` (String)
        if self.uppercase { 
            chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        // if `self.lowercase == true` push the lowercase letters into `chars` (String)
        if self.lowercase {
            chars.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        // if none of the attributes are marked as true, we return an error
        if !self.numbers 
            && !self.symbols
            && !self.uppercase
            && !self.lowercase {
                return Err("At least one state should be `true`.");
            }
            
        let key: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
        Ok(key)
    }
}
#[cfg(test)]
mod tests 
{
    use super::*;
    #[test]
    fn test_keygen_def() {
        let mut kg = KeyGen::new();
        let gen_key = kg.gen_one().unwrap();
        assert_eq!(gen_key.len(), 8);
        assert_eq!(kg.length, 8);
        assert_eq!(kg.symbols, false);
        assert_eq!(kg.numbers, false);
    }

    #[test]
    fn test_keygen() { 
        let mut kg = KeyGen::new()
            .length(10)
            .symbols(true)
            .numbers(true);
        let gen_key = kg.gen_one().unwrap();
        assert_eq!(gen_key.len(), 10);
        assert_eq!(kg.length, 10);
        assert_eq!(kg.symbols, true);
        assert_eq!(kg.numbers, true);
    }

}
