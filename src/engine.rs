use serde_json::Value;
use std::collections::HashMap;

pub struct Engine {
    pub data: HashMap<String, Value>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn update(&mut self, key: String, value: Value) {
        if let Some(existing_value) = self.data.get_mut(&key) {
            *existing_value = value;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<Value> {
        self.data.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn get_all(&self) -> HashMap<String, Value> {
        self.data.clone()
    }

    pub fn get_filtered(&self, filter: &crate::models::FilterQuery) -> Vec<(String, Value)> {
        let mut result: Vec<(String, Value)> = self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if let Some(key) = filter.get_key() {
            result.retain(|(k, _)| k == key);
        }

        if let Some(params) = filter.get_params() {
            result.retain(|(_, value)| self.matches(value, params));
        }

        if let Some(sort) = filter.get_sort() {
            if let Some(direction) = filter.get_sort_direction() {
                let mut decorated: Vec<(Value, (String, Value))> = result
                    .into_iter()
                    .filter_map(|entry| {
                        let key = self.find_nested(&entry.1, sort);
                        if key.is_null() {
                            None
                        } else {
                            Some((key, entry))
                        }
                    })
                    .collect();

                decorated.sort_by(|a, b| {
                    match direction {
                        crate::models::SortDirection::Ascending => Self::compare_values(&a.0, &b.0),
                        crate::models::SortDirection::Descending =>
                            Self::compare_values(&b.0, &a.0),
                    }
                });

                result = decorated
                    .into_iter()
                    .map(|(_, entry)| entry)
                    .collect();
            }
        }

        // Paginate after sorting.
        if let Some(skip) = filter.get_skip() {
            result.drain(..skip.min(result.len()));
        }

        if let Some(limit) = filter.get_limit() {
            result.truncate(limit);
        }

        result
    }

    fn matches(&self, current_data: &Value, current_query: &Value) -> bool {
        let Some(query_map) = current_query.as_object() else {
            return true;
        };

        for (key, expected) in query_map {
            let Some(data_val) = current_data.get(key) else {
                return false;
            };

            if data_val.is_object() && expected.is_object() {
                if !self.matches(data_val, expected) {
                    return false;
                }
            } else if data_val != expected {
                return false;
            }
        }

        true
    }

    fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
            _ => a.to_string().cmp(&b.to_string()),
        }
    }

    pub fn find_field(&self, current_data: &Value, query: &Value) -> Value {
        self.find_nested(current_data, query)
    }

    fn find_nested(&self, current_data: &Value, current_query: &Value) -> Value {
        if let Some(query_map) = current_query.as_object() {
            for (key, nested_query) in query_map {
                if let Some(data_val) = current_data.get(key) {
                    if data_val.is_object() && nested_query.is_object() {
                        return self.find_nested(data_val, nested_query);
                    }

                    if !data_val.is_array() {
                        return data_val.clone();
                    }
                }
            }
        }

        Value::Null
    }
}
