use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use zbus::zvariant::{Signature, Type};

#[derive(Serialize, Deserialize, Type, Debug)]
// #[zvariant(signature = "(s)")]
pub struct DBox<T: Type> {
	json: String,
	#[serde(skip_serializing, default)]
	phantom: PhantomData<T>,
}

impl<'de, T: Clone + Serialize + Deserialize<'de> + Type> DBox<T> {
	pub fn new(value: &T) -> DBox<T> {
		DBox {
			json: serde_json::to_string(value).unwrap(),
			phantom: PhantomData,
		}
	}

	pub fn unwrap(&'de self) -> T {
		print!("{}", self.json);
		serde_json::from_str(&self.json).unwrap()
	}
}

// impl<T> Type for DBox<T> {
// 	fn signature() -> Signature<'static> {
// 		Signature::from_str_unchecked("s")
// 	}
// }
//
// impl<T> Serialize for DBox<T> {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: Serializer,
// 	{
// 		let json = serde_json::to_string(self).unwrap();
// 		serializer.serialize_str(json.as_str())
// 	}
// }
//
// impl<'de, T> Deserialize<'de> for DBox<T> {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 	where
// 		D: Deserializer<'de>,
// 	{
// 		let json = deserializer.deserialize_string(DBoxVisitor)?;
// 		Ok(serde_json::from_str(json.as_str()).unwrap())
// 	}
// }
//
// struct DBoxVisitor;
//
// impl<'de> Visitor<'de> for DBoxVisitor {
// 	type Value = String;
//
// 	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		formatter.write_str("A json string")
// 	}
//
// 	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
// 	where
// 		E: serde::de::Error,
// 	{
// 		Ok(String::from(v))
// 	}
// }
