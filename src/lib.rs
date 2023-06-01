//! # KeyGen v0.1.71 (beta)
//! Simple, customizable and convenient Key Generator. (in development).
//!
//! By using `KeyGen` you can generate the keys of any size,enable/disable symbols, numbers, spaces, uppercase/lowercase letters, etc.
//! # Features
//!
//! * Bug fixes
//! * Code optimization
//! * New features added (`space` state, `gen` method)
//! Thank you for you help & support!
//!
//! # Example
//! ```
//! fn main() {
//!     let mut gen_range = KeyGen::new()
//!         .length(11)
//!         .numbers(true)
//!         .symbols(true)
//!         .uppercase(true)
//!         .lowercase(true)
//!         .space(true);
//!
//!    let res = gen_range
//!         .gen_one()
//!         .unwrap();
//!    println!("{res}");
//!    let res2 = gen_range
//!         .gen(5)
//!         .unwrap();
//!    println!("{res2:?}");
//!}
//! ```
//! # Hasher 
//! `Hasher` -  is a tool that allows you to get a hash of the generated key using 2 algorithms for this: `Sha256` & `Md5`. Connects using `features = "crypto"`
//! Dependencies: `rust-crypto = "0.2.36"`
//! ## Example
//! ```
//! fn main() { 
//!    /*...*/
//!    let mut hasher = crypto::hash::Hasher::new();
//!    let res = hasher.hash_by_sha256(&key_str);
//!    println!("{}", res);
//! }
//! ```


pub mod generator;
#[cfg(feature = "test")]
pub mod tests;
#[cfg(feature = "crypto")]
pub mod crypto;
