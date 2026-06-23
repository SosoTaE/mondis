use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
enum Methods{
    SET,
    UPDATE,
    DELETE,
    GET
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Model {
    data: Value,
    pub comment: Option<String>,
    key: String,
    method: Option<Methods>
}



impl Model {
    pub fn new(data: Value, comment: Option<String>, key: String, method: Methods) -> Self {
        Model {
            data,
            comment,
            key,
            method: Option::Some(method)
        }
    }

    pub fn get_data(&self) -> &Value {
        &self.data
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn set_method(&mut self, method: Methods) {
        self.method = Option::Some(method);
    }
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum SortDirection {
    Ascending,
    Descending,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FilterQuery {
    key: Option<String>,
    limit: Option<usize>,
    skip: Option<usize>,
    sort: Option<Value>,
    sort_direction: Option<SortDirection>,
    params: Option<Value>,
}

impl FilterQuery {
    pub fn new(key: Option<String>, limit: Option<usize>, skip: Option<usize>, sort: Option<Value>, sort_direction: Option<SortDirection>, params: Option<Value>) -> Self {
        FilterQuery { key, limit, skip, sort, sort_direction, params }
    }

    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn get_skip(&self) -> Option<usize> {
        self.skip
    }

    pub fn get_sort(&self) -> Option<&Value> {
        self.sort.as_ref()
    }

    pub fn get_sort_direction(&self) -> Option<SortDirection> {
        self.sort_direction
    }

    pub fn get_params(&self) -> Option<&Value> {
        self.params.as_ref()
    }
}