use serde::Serialize;

#[derive(Serialize)]
pub struct QueryError {
    table: String,
    function: String,
    error: String,
}
impl QueryError {
    pub fn new(table: String, function: String, error: String) -> Self {
        Self {
            table,
            function,
            error,
        }
    }
    pub fn jsonify(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(j) => j,
            Err(e) => {
                return SerializeError::new("QueryError.jsonify".to_string(), e.to_string())
                    .jsonify();
            }
        }
    }
}

#[derive(Serialize)]
pub struct SerializeError {
    function: String,
    error: String,
}
impl SerializeError {
    pub fn new(function: String, error: String) -> Self {
        Self { function, error }
    }

    pub fn jsonify(&self) -> String {
        match serde_json::to_string(self) {
            Ok(j) => j,
            Err(_) => return r#"{ "error": "failed to serialze SerializeError" }"#.to_string(),
        }
    }
}
