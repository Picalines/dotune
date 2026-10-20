use std::collections::HashMap;
use std::sync::Arc;

use thiserror::Error;

use super::system_info::CURRENT_DIR;
use super::{AbsolutePath, AbsolutePathBuf};

#[derive(Debug)]
pub struct Inputs {
    params: HashMap<String, String>,
    cwd: Arc<AbsolutePath>,
}

#[derive(Debug, Error)]
pub enum InputsCreationError {
    #[error("input parameter '{0}' was provided more than once")]
    DuplicateParameter(String),
}

#[derive(Debug, Error)]
pub enum InputReadError {
    #[error("input parameter '{0}' was not provided")]
    MissingParameter(String),
    #[error("input parameter '{0}' cannot be empty for the requested type")]
    EmptyValue(String),
}

impl Inputs {
    pub fn new(
        params: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, InputsCreationError> {
        let mut values = HashMap::new();
        for (name, value) in params {
            if values.contains_key(&name) {
                return Err(InputsCreationError::DuplicateParameter(name));
            }
            values.insert(name, value);
        }

        Ok(Self {
            params: values,
            cwd: CURRENT_DIR.clone(),
        })
    }

    pub fn string(&self, name: &str) -> Result<&str, InputReadError> {
        self.params
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| InputReadError::MissingParameter(name.to_owned()))
    }

    pub fn path(&self, name: &str) -> Result<AbsolutePathBuf, InputReadError> {
        let value = self.string(name)?;
        if value.is_empty() {
            return Err(InputReadError::EmptyValue(name.to_owned()));
        }

        Ok(self.cwd.join(value))
    }
}
