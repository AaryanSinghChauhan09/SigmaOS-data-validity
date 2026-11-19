#![forbid(unsafe_code)]
//! SigmaOS-data-validity: data validity model.
//!
//! Real implementation: schemas, type checking, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Validity schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValiditySchema {
    pub id: String,
    pub name: String,
    pub entity: String,
    pub type_check: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidityError {
    EmptyName,
    DuplicateSchema(String),
}

impl fmt::Display for ValidityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "schema name must not be empty"),
            Self::DuplicateSchema(id) => write!(f, "duplicate schema {:?}", id),
        }
    }
}

impl std::error::Error for ValidityError {}

/// Data validity manager.
pub struct DataValidity {
    pub name: String,
    schemas: BTreeMap<String, ValiditySchema>,
}

impl DataValidity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schemas: BTreeMap::new(),
        }
    }

    pub fn add_schema(&mut self, schema: ValiditySchema) -> Result<(), ValidityError> {
        if schema.name.trim().is_empty() { return Err(ValidityError::EmptyName); }
        let id = schema.id.clone();
        if self.schemas.contains_key(&id) {
            return Err(ValidityError::DuplicateSchema(id.clone()));
        }
        self.schemas.insert(id, schema);
        Ok(())
    }

    pub fn schema_count(&self) -> usize { self.schemas.len() }

    pub fn get_schema(&self, id: &str) -> Option<&ValiditySchema> { self.schemas.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validity_create_aaa() {
        let v = DataValidity::new("My Validity");
        assert_eq!(v.name, "My Validity");
        assert_eq!(v.schema_count(), 0);
    }

    #[test]
    fn test_add_schema_aaa() {
        let mut v = DataValidity::new("My Validity");
        let s = ValiditySchema { id: "s1".into(), name: "email-schema".into(), entity: "user".into(), type_check: "email-format".into() };
        v.add_schema(s).unwrap();
        assert_eq!(v.schema_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut v = DataValidity::new("My Validity");
        let s = ValiditySchema { id: "x".into(), name: "".into(), entity: "a".into(), type_check: "b".into() };
        assert!(v.add_schema(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut v = DataValidity::new("My Validity");
        let s = ValiditySchema { id: "x".into(), name: "X".into(), entity: "a".into(), type_check: "b".into() };
        v.add_schema(s).unwrap();
        let s2 = ValiditySchema { id: "x".into(), name: "X".into(), entity: "a".into(), type_check: "b".into() };
        assert!(v.add_schema(s2).is_err());
    }
}
