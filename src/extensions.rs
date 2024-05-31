use serde_json::Value;

use crate::error::{GrowthbookError, GrowthbookErrorCode};

pub trait ConvertToUsize {
    fn convert_to_usize(&self) -> Result<usize, GrowthbookError>;
}

pub trait ConvertToString {
    fn convert_to_string(&self) -> Result<String, GrowthbookError>;
}

pub trait FoldVecString {
    fn fold_to_string(&self) -> String;
}

impl ConvertToUsize for String {
    fn convert_to_usize(&self) -> Result<usize, GrowthbookError> {
        self.replace('.', "")
            .parse::<usize>()
            .map_err(GrowthbookError::from)
    }
}

impl ConvertToUsize for Value {
    fn convert_to_usize(&self) -> Result<usize, GrowthbookError> {
        let string = self.convert_to_string()?.replace('.', "");
        string.parse().map_err(GrowthbookError::from)
    }
}

impl ConvertToString for Value {
    fn convert_to_string(&self) -> Result<String, GrowthbookError> {
        self.as_str().map(String::from).ok_or(GrowthbookError::new(
            GrowthbookErrorCode::SerdeDeserialize,
            &format!("Failed to convert value={self} to str"),
        ))
    }
}

impl FoldVecString for Vec<String> {
    fn fold_to_string(&self) -> String {
        self.iter().fold(String::new(), |s1, s2| s1 + s2)
    }
}

impl ConvertToUsize for Vec<String> {
    fn convert_to_usize(&self) -> Result<usize, GrowthbookError> {
        self.fold_to_string().convert_to_usize()
    }
}
