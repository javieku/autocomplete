use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AutocompleteResponse {
    suggestions: Vec<String>,
}

impl AutocompleteResponse {
    pub fn new(suggestions: Vec<String>) -> Self {
        AutocompleteResponse { suggestions }
    }
}
