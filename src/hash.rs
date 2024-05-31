use xxhash_rust::xxh3::Xxh3;

pub struct Hasher;

impl Hasher {
    pub fn hash_code(input: &str) -> f32 {
        (gen_hash_code(input) % 101) as f32 / 100.0
    }
}

fn gen_hash_code(input: &str) -> u64 {
    let mut hasher = Xxh3::new();
    hasher.update(input.as_bytes());
    hasher.digest()
}

#[cfg(test)]
mod test {
    use crate::hash::Hasher;

    #[test]
    fn generate_same_hash_code_every_time() -> Result<(), Box<dyn std::error::Error>> {
        let hash_code = Hasher::hash_code("018fcf36-d39b-705c-a800-dc8bdc5964be");
        assert_eq!(0.31, hash_code);
        Ok(())
    }
}
