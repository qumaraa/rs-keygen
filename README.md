
## [rs-keygen](https://docs.rs/keygenx/0.1.1/keygenx/)  
Open-source, simple, customizable and convenient **Key Generator** (in development).

 [![Version](https://img.shields.io/badge/keygen-0.1.1-pink)]()
 [![Bugs](https://img.shields.io/badge/bugs-fixed-blue)]() 
 ![Issues](https://img.shields.io/badge/issues-goto-green) 
 ![Authors](https://img.shields.io/badge/author-@alexanderqmv-yellow)
 
 ## Review
 * **`gen_one`** - **Generate one key** (**returns** **`Result<String, &'static str`**)
 * **`gen`** - **Generates the `nth` number of keys (-`n` is passed in the argument)**
 
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
 
 ## [Documentation](https://docs.rs/keygenx/0.1.1/keygenx/) 
 The main doc is available at `docs.rs`. There you can read and discover examples and source-code of `rs-keygen` aka `keygenx`
 
 ## Contributing
**`KeyGen` open for contribution.** 

**Contact Us: qmvscorpio@gmail.com**
