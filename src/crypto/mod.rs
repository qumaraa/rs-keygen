pub mod Hash {
    use crypto::{
        digest::Digest,
        md5::{self, Md5},
        sha2::Sha256,
    };
    /// [`Hasher`] - hasher structure contains 2 hashing algorith methods (`hash_with_sha256()`, `hash_with_md5()`)
    pub struct Hasher {}
    
    impl Hasher {
        pub fn new() -> Self {
            Hasher {}
        }
        /// Implements `SHA256` hashing algorithm and returns `String` (Security ⭐⭐⭐⭐⭐)
        pub fn hash_with_sha256(&self, key: &String) -> String {
            let mut sha2_hasher = Sha256::new();
            sha2_hasher.input_str(key);
            sha2_hasher.result_str()
        }
        /// Implements `MD5` hashing algorithm and returns `String` (Security ⭐⭐⭐⭐)
        pub fn hash_with_md5(&self,key: &String) -> String {
            let mut md5_hasher = Md5::new();
            md5_hasher.input_str(key);
            md5_hasher.result_str()
        }
    }
}