use crate::common_types::Category;

impl From<Category> for u8 {
    fn from(val: Category) -> Self {
        Self::from(&val)
    }
}

impl From<&Category> for u8 {
    fn from(val: &Category) -> Self {
        match val {
            Category::Normal => 0,
            Category::Shortcut => 1,
            Category::Unrestricted => 2,
        }
    }
}

impl serde::Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.into())
    }
}