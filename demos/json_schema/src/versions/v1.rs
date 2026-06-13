//! Version 1 of the demo schema.
use super::v2;

/// The version 1 schema: an entity identified only by name.
pub struct Schema {
    /// The entity's name.
    pub name: String,
}

impl From<v2::Schema> for Schema {
    /// Downgrades a version 2 schema to version 1 by dropping the `description` field.
    ///
    /// # Parameters
    /// - `value`: the version 2 schema to downgrade.
    ///
    /// # Returns
    /// The equivalent version 1 schema, keeping only the `name` field.
    fn from(value: v2::Schema) -> Self {
        let v2::Schema { name, .. } = value;
        Self { name }
    }
}
