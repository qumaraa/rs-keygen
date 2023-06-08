
use rand::{thread_rng, Rng};
/// [`KeyGen`] - struct contains the main logic of Key Generator

pub struct KeyGen {
    /// `length` - key length
    pub length: usize,
    /// `symbols` - a flag that can enable/disable symbols to key gen.
    pub symbols: bool,
    /// `numbers` - a flag that can enable/disable numbers to key gen.
    pub numbers: bool,
    /// `uppercase` - a flag that can enable/disable uppercase symbols to key gen.
    pub uppercase: bool,
    /// `lowercase` - a flag that can enable/disable lowercase symbols to key gen.
    pub lowercase: bool,
    /// `space` - a flag that can enable/disable space - ` ` to key gen.
    pub space: bool,
}

/// [`KeyGen`] implementation
impl KeyGen {
    /// constructor which returns `Self` with default parameters.
    pub fn new() -> Self {
        KeyGen {
            /// `Default` length of `key` = 8
            length: 0,
            /// `Default` `symbols` state = false
            symbols: false,
            /// `Default` `numbers` state = false
            numbers: false,
            /// `Default` `uppercase` state = false
            uppercase: false,
            /// `Default` `lowercase` state = false
            lowercase: false,
            /// `Default` `space` state = false
            space: false,
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
    pub fn lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }
    /// changes the boolean value (state)  of `space` to value from parameter
    pub fn space(mut self, space: bool) -> Self {
        self.space = space;
        self
    }
    /// generates the random chars and collects
    /// them into `String` and returns as `Result<T,E>`
    #[inline]
    pub fn gen_one(&mut self) -> Result<String, &'static str> {
        let chars = self.try_str().unwrap();

        let mut rng = thread_rng();
        let key: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
        Ok(key)
    }

    /// Generates `n` passwords of type `usize` and returns as `Result<Vec<String>, &'static str>`
    #[inline]
    pub fn gen(&mut self, n: usize) -> Result<Vec<String>, &'static str> {
        let chars = self.try_str().unwrap();
        let mut key: Vec<String> = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..n {
            let k: String = (0..self.length)
                .map(|_| {
                    let idx = rng.gen_range(0..chars.len());
                    chars.chars().nth(idx).unwrap()
                })
                .collect();
            key.push(k);
        }
        Ok(key)
    }
    /// `try_str` contains one logic for both `gen_one()` and `gen()`.
    pub fn try_str(&mut self) -> Result<String, &'static str> {
        if self.length == 0 {
            return Err("length of the password should be more than `0`");
        }

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

        // if `self.space == true` push the space into `chars` (String)
        if self.space {
            chars.push_str(" ");
        }
        // if none of the attributes are marked as true, we return an error
        if !self.numbers && !self.symbols && !self.uppercase && !self.lowercase {
            return Err("At least one state should be `true`.");
        }
        Ok(chars)
    }
}
