use serde_json::json;

use crate::dto::request::AutocompleteRequest;

impl From<&AutocompleteRequest> for serde_json::Value {
    fn from(req: &AutocompleteRequest) -> serde_json::Value {
        json!({
            "suggest" : {
                "poi-suggestions" : {
                  "prefix" : &req.query,
                  "completion" : {
                    "field": "name.suggest",
                    "skip_duplicates": true
                  }
                }
              }
            }
        )
    }
}
