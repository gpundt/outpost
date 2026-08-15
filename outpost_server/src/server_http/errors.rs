use serde::Serialize;

/// Struct to organize an individual http query endpoint error
#[derive(Serialize)]
pub struct QueryError {
    table: String,
    function: String,
    error: String,
}
impl QueryError {
    /// Function to create a new http query endpoint error
    pub fn new(table: String, function: String, error: String) -> Self {
        Self {
            table,
            function,
            error,
        }
    }
    /// Function to serialize an individual QueryError into JSON, or return a SerializeError
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

/// Struct to organize an individual serialization attempt error
#[derive(Serialize)]
pub struct SerializeError {
    function: String,
    error: String,
}
impl SerializeError {
    /// Function to create a new serialization attempt error object
    pub fn new(function: String, error: String) -> Self {
        Self { function, error }
    }

    /// Function to serialize an individual Serialization error
    pub fn jsonify(&self) -> String {
        match serde_json::to_string(self) {
            Ok(j) => j,
            Err(_) => return r#"{ "error": "failed to serialze SerializeError" }"#.to_string(),
        }
    }
}
