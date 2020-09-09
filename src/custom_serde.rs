use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use serde::ser::{self, Serialize, Serializer};
use crate::test;

impl Serialize for protobuf::ProtobufEnumOrUnknown<test::character::Class> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i32(*self)
    }
}
