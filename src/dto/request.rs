use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AutocompleteRequest {
    pub query: String,
    pub country_code: String,
    pub location: Geocoodinates,
}

#[derive(Deserialize, Debug)]
pub struct Geocoodinates {
    pub latitude: f32,
    pub longitude: f32,
}
