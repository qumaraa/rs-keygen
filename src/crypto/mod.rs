//! [`Hash`] - scope (or module) where is the `Hasher` structure located 
pub mod Hash {
    use crypto::{
        digest::Digest,
        md5::{self, Md5},
        sha2::Sha256,
    };
    //! [`Hasher`] - hasher structure contains 2 hashing algorith methods (`hash_with_sha256()`, `hash_with_md5()`)
    pub struct Hasher {}

    impl Hasher {
        pub fn new() -> Self {
            Hasher {}
        }
        //! `hash_with_sha256` -  Implements `SHA256` hashing algorithm and returns `String` (Security ⭐⭐⭐⭐⭐)
        
        //! ## Example
        //! ```
        //! fn main() {
        //! 
        //!     /* snip.. */
        //!     let mut sha256_hasher = Hasher::new();
        //!     
        //!     let res = sha256_hasher.hash_with_sha256(&str);
        //!     // do some work with `res`, for example print the value
        //!     println!("{res}"); 
        //! }
        //! 
        //! 
        pub fn hash_with_sha256(&self, key: &String) -> Option<String> {
            if key.len() == 0 {
                None
            }
            let mut sha2_hasher = Sha256::new();
            sha2_hasher.input_str(key);
            
            Some(sha2_hasher.result_str())
        }
        /// Implements `MD5` hashing algorithm and returns `String` (Security ⭐⭐⭐⭐)
        pub fn hash_with_md5(&self, key: &String) -> Option<String> {
            if key.len() == 0 {
                None
            }
            let mut md5_hasher = Md5::new();
            md5_hasher.input_str(key);
            
            Some(md5_hasher.result_str())
        }
    }
}
