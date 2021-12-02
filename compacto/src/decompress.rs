use serde_json::{Map, Value};

use crate::{error::Error, Result};

#[derive(Debug)]
pub struct Decompressor {
    refs: Vec<Value>,
    input: Value,
    output: Value,
}

impl Decompressor {
    /// Construct a new Decompressor using a a JSON string and default values
    ///
    /// Example
    /// ```
    /// let mut compacto = compacto::Decompressor::new(r#"[{"0":1,"1": 0},["a","b"]]"#).unwrap();
    /// println!("{:?}", compacto.decompress().unwrap());
    /// ```
    pub fn new(value: &str) -> Result<Self> {
        let input = serde_json::from_str(value)?;

        Ok(Self {
            input,
            refs: Vec::new(),
            output: Value::default(),
        })
    }

    /// Decompress JSON
    ///
    /// decompress will first check if the input is an array and its length is equal to two.  
    /// If it does not match the "criteria to decompress," the original input will be returned.
    /// If the input matches the criteria, we rebuilt the JSON using the reference table.
    pub fn decompress(&mut self) -> Result<String> {
        if !self.input.is_array() || self.input.as_array().unwrap().len() != 2 {
            return Ok(self.input.to_string());
        }

        self.refs = match self.input.get(1) {
            Some(values) => values.as_array().unwrap().clone(),
            None => Vec::new(),
        };

        let result = match self.input.get(0) {
            Some(value) => {
                let output = match value {
                    Value::Array(a) => self.find_array_value_by_ref(a),
                    Value::Object(o) => self.find_object_value_by_ref(o),
                    _ => self.find_value_by_ref(value),
                }?;

                output.to_string()
            }
            None => Value::Null.to_string(),
        };

        Ok(result)
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
            _ => Err(Error::UnknownJSONValueRef(value.to_string())),
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
pub fn decompress_json(json: &str) -> Result<String> {
    Decompressor::new(json)?.decompress()
}

#[cfg(test)]
mod test {
    use super::*;

    const OUTPUT_SAMPLE: &str = include_str!("../../samples/test-samples/output.json");
    const INPUT_SAMPLE: &str = include_str!("../../samples/test-samples/input.json");

    #[test]
    fn should_return_valid_json() -> crate::Result<()> {
        let result = Decompressor::new(OUTPUT_SAMPLE)?.decompress()?;
        let output: Value = serde_json::from_str(&result)?;
        assert_eq!(true, output.is_object());
        Ok(())
    }

    #[test]
    fn should_be_equal_to_original_json() -> crate::Result<()> {
        let result = Decompressor::new(OUTPUT_SAMPLE)?.decompress()?;
        let result_value: Value = serde_json::from_str(&result)?;
        let expect: Value = serde_json::from_str(INPUT_SAMPLE)?;
        assert_eq!(expect, result_value);
        Ok(())
    }

    #[test]
    fn should_return_original_input_if_not_array() -> crate::Result<()> {
        let result = Decompressor::new(r#"{"ok":"ok"}"#)?.decompress()?;
        let result_value: Value = serde_json::from_str(&result)?;
        let mut expected = Map::new();
        expected.insert("ok".to_string(), Value::String("ok".to_string()));
        assert_eq!(Value::Object(expected), result_value);
        Ok(())
    }

    #[test]
    fn should_return_original_input_if_not_array_expected_length() -> crate::Result<()> {
        let result = Decompressor::new(r#"[{"ok":"ok"}]"#)?.decompress()?;
        let result_obj: Value = serde_json::from_str(&result)?;
        let mut expected = Map::new();
        expected.insert("ok".to_string(), Value::String("ok".to_string()));
        assert_eq!(Value::Array(vec![Value::Object(expected)]), result_obj);
        Ok(())
    }
}
