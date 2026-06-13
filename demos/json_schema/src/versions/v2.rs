//! Version 2 of the demo schema.
//!
//! Adds a `description` field to the version 1 schema, representing the kind of additive
//! schema change that `berde` migrations are designed to handle.
use super::v1;

/// The version 2 schema: an entity with a name and a description.
pub struct Schema {
    /// The entity's name.
    pub name: String,
    /// A human-readable description of the entity.
    pub description: String,
}

impl From<v1::Schema> for Schema {
    /// Upgrades a version 1 schema to version 2, adding an empty `description` field.
    ///
    /// # Parameters
    /// - `value`: the version 1 schema to upgrade.
    ///
    /// # Returns
    /// The equivalent version 2 schema, with `description` set to an empty string.
    fn from(value: v1::Schema) -> Self {
        let v1::Schema { name } = value;
        let description = String::default();
        Self { name, description }
    }
}
