use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(query_string: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in query_string.split("&") {
            let mut key = sub_str;
            let mut value = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|entry: &mut Value| match entry {
                    Value::Single(prev_entry) => {
                        *entry = Value::Multiple(vec![prev_entry, value]);
                    }
                    Value::Multiple(vec) => vec.push(value),
                })
                .or_insert(Value::Single(value));
        }

        Self { data }
    }
}
