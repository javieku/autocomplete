//use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AutocompleteRequest {
    query: String,
    country_code: String,
    location: Geocoodinates,
}

#[derive(Deserialize, Debug)]
pub struct Geocoodinates {
    latitude: f32,
    longitude: f32,
}
