use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpHeaders<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Default for HttpHeaders<'buf> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<'buf> HttpHeaders<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for HttpHeaders<'buf> {
    fn from(header_str: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in header_str.split("\r\n") {
            if let Some(i) = sub_str.find(":") {
                let key = &sub_str[..i];
                let value_string = &sub_str[i + 1..];
                let value_iter = value_string.split(',');

                for value in value_iter {
                    if !value.is_empty() {
                        data.entry(key)
                            .and_modify(|entry: &mut Value| match entry {
                                Value::Single(prev_entry) => {
                                    *entry = Value::Multiple(vec![prev_entry, value]);
                                }
                                Value::Multiple(vec) => vec.push(value),
                            })
                            .or_insert(Value::Single(value));
                    }
                }
            }
        }

        Self { data }
    }
}
