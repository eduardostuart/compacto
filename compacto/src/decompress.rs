use serde_json::{Map, Value};

use crate::{error::Error, Result};

#[derive(Debug)]
pub struct Decompressor {
    refs: Vec<Value>,
    input: Value,
    output: Value,
}

impl Default for Decompressor {
    fn default() -> Self {
        Self {
            refs: Vec::new(),
            output: Value::default(),
            input: Value::default(),
        }
    }
}

impl Decompressor {
    /// Construct a new Decompressor using a serde::Value and default values
    ///
    /// Example
    /// ```
    /// let json = serde_json::from_str(r#"[{"0":1,"1": 0},["a","b"]]"#).unwrap();
    /// let mut compacto = compacto::Decompressor::new(json);
    /// println!("{:?}", compacto.decompress());
    /// ```
    pub fn new(input: Value) -> Self {
        Self {
            input,
            ..Decompressor::default()
        }
    }

    /// Construct a new Decompressor using a a JSON string and default values
    ///
    /// Example
    /// ```
    /// let mut compacto = compacto::Decompressor::new_from_str(r#"[{"0":1,"1": 0},["a","b"]]"#).unwrap();
    /// println!("{:?}", compacto.decompress());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self> {
        let input = serde_json::from_str(value)?;

        Ok(Self {
            input,
            ..Decompressor::default()
        })
    }

    /// Decompress JSON
    ///
    /// decompress will first check if the input is an array and its length is equal to two.  
    /// If it does not match the "criteria to decompress," the original input will be returned.
    /// If the input matches the criteria, we rebuilt the JSON using the reference table.
    pub fn decompress(&mut self) -> Result<Value> {
        if !self.input.is_array() || self.input.as_array().unwrap().len() != 2 {
            return Ok(self.input.clone());
        }

        self.refs = match self.input.get(1) {
            Some(values) => values.as_array().unwrap().clone(),
            None => Vec::new(),
        };

        Ok(match self.input.get(0) {
            Some(value) => match value {
                Value::Array(a) => self.find_array_value_by_ref(a),
                Value::Object(o) => self.find_object_value_by_ref(o),
                _ => self.find_value_by_ref(value),
            }?,
            None => Value::Null,
        })
    }

    pub(self) fn find_array_value_by_ref(&self, value: &[Value]) -> Result<Value> {
        let items: Result<Vec<Value>> = value
            .iter()
            .map(|v| match v {
                Value::Array(a) => self.find_array_value_by_ref(a),
                Value::Object(o) => self.find_object_value_by_ref(o),
                _ => self.find_value_by_ref(v),
            })
            .collect();

        Ok(Value::Array(items?))
    }

    pub(self) fn find_object_value_by_ref(&self, value: &Map<String, Value>) -> Result<Value> {
        let mut new_obj: Map<String, Value> = Map::new();

        for (obj_key, obj_value) in value.clone().iter() {
            let k = self.refs[obj_key.parse::<usize>().unwrap()]
                .as_str()
                .unwrap();

            let v = match obj_value {
                Value::Array(a) => self.find_array_value_by_ref(a),
                Value::Object(o) => self.find_object_value_by_ref(o),
                _ => self.find_value_by_ref(obj_value),
            }?;

            new_obj.insert(k.to_string(), v);
        }

        Ok(Value::Object(new_obj))
    }

    pub(self) fn find_value_by_ref(&self, value: &Value) -> Result<Value> {
        match value {
            Value::Number(n) => {
                let index = n.as_u64().unwrap() as usize;
                Ok(self.refs[index].clone())
            }
            Value::String(s) => {
                let index: usize = s.parse().unwrap();
                Ok(self.refs[index].clone())
            }
            _ => Err(Error::UnknownJSONValueRef(value.clone())),
        }
    }
}

/// Decompress JSON from a string
///
/// # Example
///
/// ```
/// let json = r#"[[{"0":1,"2":3},{"0":4,"2":3}],["id","123","name","Eduardo","456"]]"#;
/// let output = compacto::decompress_json(json).unwrap();
/// println!("{:#?}", output);
/// ```
pub fn decompress_json(json: &str) -> Result<Value> {
    Decompressor::new_from_str(json)?.decompress()
}

#[cfg(test)]
mod test {
    use crate::test_utils;

    use super::*;

    #[test]
    fn should_return_valid_json() {
        let result = Decompressor::new(test_utils::get_json_value_sample("output.json"))
            .decompress()
            .unwrap();
        assert_eq!(true, result.is_object());
    }

    #[test]
    fn should_be_equal_to_original_json() {
        let result = Decompressor::new(test_utils::get_json_value_sample("output.json"))
            .decompress()
            .unwrap();
        let expect: Value = test_utils::get_json_value_sample("input.json");
        assert_eq!(expect, result);
    }

    #[test]
    fn should_return_original_input_if_not_array() {
        let result = Decompressor::new_from_str(r#"{"ok":"ok"}"#)
            .unwrap()
            .decompress()
            .unwrap();

        let mut expected = Map::new();
        expected.insert("ok".to_string(), Value::String("ok".to_string()));
        assert_eq!(Value::Object(expected), result);
    }

    #[test]
    fn should_return_original_input_if_not_array_expected_length() {
        let result = Decompressor::new_from_str(r#"[{"ok":"ok"}]"#)
            .unwrap()
            .decompress()
            .unwrap();

        let mut expected = Map::new();
        expected.insert("ok".to_string(), Value::String("ok".to_string()));
        assert_eq!(Value::Array(vec![Value::Object(expected)]), result);
    }
}
