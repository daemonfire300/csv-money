use serde::{Serialize, ser::SerializeStruct};

use crate::objects::accounts::Account;

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut obj = serializer.serialize_struct("Account", 5)?;
        obj.serialize_field("client", &self.id)?;
        obj.serialize_field("available", &self.available)?;
        obj.serialize_field("held", &self.held)?;
        obj.serialize_field("total", &self.total())?;
        obj.serialize_field("locked", &self.locked)?;
        obj.end()
    }
}
