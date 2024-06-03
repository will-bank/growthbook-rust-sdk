use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

pub enum HashCodeVersion {
    V1,
    V2,
}

impl From<i64> for HashCodeVersion {
    fn from(value: i64) -> Self {
        match value {
            2 => Self::V2,
            _ => Self::V1,
        }
    }
}

impl From<Option<i64>> for HashCodeVersion {
    fn from(option: Option<i64>) -> Self {
        match option {
            None => Self::V1,
            Some(value) => Self::from(value),
        }
    }
}

pub struct HashCode;

impl HashCode {
    pub fn hash_code(input: &str, seed: &str, version: HashCodeVersion) -> f32 {
        match version {
            HashCodeVersion::V1 => Self::hash_v1(input, seed),
            HashCodeVersion::V2 => Self::hash_v2(input, seed),
        }
    }

    fn hash_v1(input: &str, seed: &str) -> f32 {
        let concatenated = format!("{}{}", input, seed);

        let mut hasher = FnvHasher::default();
        concatenated.hash(&mut hasher);
        let hash_value = hasher.finish();

        let thousand = 1000;
        let remainder = hash_value % thousand;

        remainder as f32 / 1000.0
    }

    fn hash_v2(input: &str, seed: &str) -> f32 {
        let concatenated = format!("{}{}", seed, input);
        let first = Self::fnv1a_32(concatenated.as_bytes());

        let first_as_string = format!("{}", first);
        let second = Self::fnv1a_32(first_as_string.as_bytes());

        let ten_thousand = 10000;
        let remainder = (second as u64) % ten_thousand;

        remainder as f32 / 10000.0
    }

    fn fnv1a_32(data: &[u8]) -> u32 {
        let mut hasher = FnvHasher::default();
        hasher.write(data);
        hasher.finish() as u32
    }
}

#[cfg(test)]
mod test {
    use crate::hash::{HashCode, HashCodeVersion};

    #[test]
    fn hash_v1() -> Result<(), Box<dyn std::error::Error>> {
        let hash_code = HashCode::hash_code(
            "018fcf36-d39b-705c-a800-dc8bdc5964be",
            "seed",
            HashCodeVersion::V1,
        );
        assert_eq!(0.137, hash_code);
        Ok(())
    }

    #[test]
    fn hash_v2() -> Result<(), Box<dyn std::error::Error>> {
        let hash_code = HashCode::hash_code(
            "018fcf36-d39b-705c-a800-dc8bdc5964be",
            "seed",
            HashCodeVersion::V2,
        );
        assert_eq!(0.1382, hash_code);
        Ok(())
    }
}
