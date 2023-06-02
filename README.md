
## [rs-keygen](https://docs.rs/keygenx/0.1.72/keygenx/)  
Open-source, simple, customizable and convenient **Key Generator** (in development).

 [![Version](https://img.shields.io/badge/keygen-0.1.71-gray)]()
 [![Bugs](https://img.shields.io/badge/bugs-fixed-gray)]() 
 ![Issues](https://img.shields.io/badge/issues-goto-gray) 
 ![Authors](https://img.shields.io/badge/author-@alexanderqmv-gray) 
 ## Features
* Splitting the code into separate `mod`ules
* Added a new hashing structure - `Hasher`
* Bugs fixed

 ## Review
 ### Generator
 * **`gen_one`** - **Generate one key** (**returns** **`Result<String, &'static str`**)
 * **`gen`** - **Generates the `nth` number of keys (-`n` is passed in the argument)**
 
### Hasher
* **`hash_with_sha256` - parses key-string to sha256 hash**
 
* **`hash_with_md5` - parses key-string to md5 hash**


### Cargo.toml
```toml
keygenx = { version = ">= 0.1.72", features = ["crypto", "tests"] }
```
 ```rs
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
    /// `space` - a flag that can
    /// enable/disable space - ` ` to key gen.
    space: bool,
}
 ```
 
 ## [Documentation](https://docs.rs/keygenx/0.1.72/keygenx/) 
 The main doc is available at `docs.rs`.
 
 ## Contributing
**`KeyGen` open for contribution.** 

**Contact Us: qmvscorpio@gmail.com**
