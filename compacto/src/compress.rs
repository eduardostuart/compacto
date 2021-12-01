use seahash::hash;
use serde_json::{Map, Value};
use std::{collections::HashMap, vec};

use crate::{error::Error, Result};

#[derive(Debug)]
pub struct Compressor {
    refs: HashMap<String, (usize, Value)>,
    input: Value,
    output: Value,
}

impl Default for Compressor {
    fn default() -> Self {
        Self {
            refs: HashMap::new(),
            output: Value::default(),
            input: Value::default(),
        }
    }
}

impl Compressor {
    /// Construct a new Compressor using a serde::Value and default values
    ///
    /// Example
    /// ```
    /// let json = serde_json::from_str(r#"{"my": "json"}"#).unwrap();
    /// let compacto = compacto::Compressor::new(json);
    /// println!("{:?}", compacto.compress());
    /// ```
    pub fn new(input: Value) -> Self {
        Self {
            input,
            ..Compressor::default()
        }
    }

    /// Construct a new Compressor using a string JSON value
    /// The string value will be deserialized into `serde_json::Value`
    ///
    /// Example
    /// ```
    /// let compacto = compacto::Compressor::new_from_str(r#"{"my": "json"}"#).unwrap();
    /// println!("{:?}", compacto.compress());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self> {
        let input = serde_json::from_str(value)?;

        Ok(Self {
            input,
            ..Compressor::default()
        })
    }

    /// Compress JSON
    ///
    /// compress will return a JSON array containing:
    /// 1. the original JSON structure using only indexes pointing to the reference list
    /// 2. a list of values (the reference table). The list should have no duplicate values
    pub fn compress(mut self) -> Result<Value> {
        let json = self.input.clone();

        let new_json = match json {
            Value::Array(ref a) => self.get_array_value(a),
            Value::Object(ref o) => self.get_object_value(o),
            _ => self.get_number_index(&json),
        }?;

        // Build the reference table
        let mut refs: Vec<Value> = Vec::new();
        refs.resize_with(self.refs.len(), Default::default);
        for (_, (index, value)) in self.refs.iter() {
            refs[*index] = value.clone();
        }

        Ok(Value::Array(vec![new_json, Value::Array(refs)]))
    }

    // Get hash value from the value and index that will be used as reference
    pub(self) fn get_hash_index_for_value(&mut self, ref_value: &Value) -> Result<(String, usize)> {
        // Get representation for each value type
        let value = match ref_value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => format!("bool:{}", b),
            Value::Number(n) => format!("number:{}", n),
            Value::String(s) => s.to_string(),
            _ => return Err(Error::UnknownJSONValueRef(ref_value.clone())),
        };

        let hash = format!("{:x}", hash(value.as_bytes()));

        Ok(match self.refs.get(&hash) {
            Some((index, _)) => (hash.clone(), *index),
            None => {
                let index = self.refs.len();
                self.refs.insert(hash.clone(), (index, ref_value.clone()));
                (hash.clone(), index)
            }
        })
    }

    pub(self) fn get_array_value(&mut self, value: &[Value]) -> Result<Value> {
        let items: Result<Vec<Value>> = value
            .iter()
            .map(|v| match v {
                Value::Array(ref a) => self.get_array_value(a),
                Value::Object(ref o) => self.get_object_value(o),
                _ => self.get_number_index(v),
            })
            .collect();

        Ok(Value::Array(items?))
    }

    pub(self) fn get_object_value(&mut self, value: &Map<String, Value>) -> Result<Value> {
        let mut map: Map<String, Value> = Map::new();
        for (k, v) in value.clone().iter() {
            let (_, index) = self.get_hash_index_for_value(&Value::String(k.to_string()))?;

            let value = match v.clone() {
                Value::Array(ref a) => self.get_array_value(a),
                Value::Object(ref o) => self.get_object_value(o),
                _ => self.get_number_index(v),
            }?;

            map.insert(index.to_string(), value);
        }
        Ok(Value::Object(map))
    }

    pub(self) fn get_number_index(&mut self, value: &Value) -> Result<Value> {
        let (_, index) = self.get_hash_index_for_value(value)?;
        Ok(Value::Number(index.into()))
    }
}

/// Compress JSON from a string
///
/// # Example
///
/// ```
/// let json = r#"[{ "id": "123", "name": "Eduardo" }, {"id": "456", "name": "Eduardo"}]"#;
/// let output = compacto::compress_json(json).unwrap();
/// println!("{:#?}", output);
/// ```
pub fn compress_json(json: &str) -> Result<Value> {
    Compressor::new_from_str(json)?.compress()
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> Value {
        serde_json::from_str(
            r#"{
                "users": [
                    {"user": {"id": 1,"name": "eduardo","age": null}},
                    {"user": {"id": 2,"name": "jose","age": 90}}], 
                "page": 1
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn should_be_able_to_compress_json() {
        let result = Compressor::new(sample()).compress().unwrap();
        assert_eq!(true, result.is_array());
        assert_eq!(true, result.get(0).unwrap().is_object());
        assert_eq!(true, result.get(1).unwrap().is_array());
    }

    #[test]
    fn should_not_contain_duplicated_values() {
        let json = Compressor::new(sample()).compress().unwrap();
        let refs = json.as_array().unwrap().get(1).unwrap().as_array().unwrap();

        assert_eq!(
            vec![
                Value::String("page".to_string()),
                Value::Number(1.into()),
                Value::String("users".to_string()),
                Value::String("user".to_string()),
                Value::String("age".to_string()),
                Value::Null,
                Value::String("id".to_string()),
                Value::String("name".to_string()),
                Value::String("eduardo".to_string()),
                Value::Number(90.into()),
                Value::Number(2.into()),
                Value::String("jose".to_string())
            ],
            refs.clone()
        );
    }

    #[test]
    fn should_compress_and_create_reference_of_values() {
        let json = serde_json::from_str(r#"{"id": "123", "123":"id"}"#).unwrap();
        let result = Compressor::new(json).compress().unwrap();

        let mut expected = Map::new();
        expected.insert("0".to_string(), 1.into());
        expected.insert("1".to_string(), 0.into());

        assert_eq!(Value::Object(expected), result.get(0).unwrap().clone());

        assert_eq!(
            Value::Array(vec![
                Value::String("123".to_string()),
                Value::String("id".to_string()),
            ]),
            result.get(1).unwrap().clone()
        );
    }
}
