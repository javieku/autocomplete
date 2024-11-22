use crate::dto::response::AutocompleteResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Source {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Suggest {
    #[serde(rename = "poi-suggestions")]
    poi_suggestions: Vec<PoiSuggestion>,
}

#[derive(Deserialize, Debug)]
struct PoiSuggestion {
    options: Vec<OptionEntry>,
}

#[derive(Deserialize, Debug)]
struct OptionEntry {
    _source: Source,
}

impl From<Suggest> for AutocompleteResponse {
    fn from(suggest: Suggest) -> Self {
        let mut result = Vec::new();
        for suggestion in suggest.poi_suggestions {
            for option in suggestion.options {
                result.push(option._source.name);
            }
        }

        AutocompleteResponse::new(result)
    }
}
