use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct AutocompleteRequest {
    #[garde(length(min = 1, max = 50))]
    pub query: String,
    #[garde(custom(is_valid_iso_two_letter_country_code))]
    pub country_code: String,
    #[garde(dive)]
    pub location: Geocoodinates,
}

#[derive(Deserialize, Debug, Validate)]
pub struct Geocoodinates {
    #[garde(range(min=-90f32, max=90f32))]
    pub latitude: f32,
    #[garde(range(min=-180f32, max=180f32))]
    pub longitude: f32,
}

fn is_valid_iso_two_letter_country_code(country_code: &str, _: &()) -> garde::Result {
    if !rust_iso3166::ALPHA2_MAP.contains_key(&country_code.to_uppercase()) {
        return Err(garde::Error::new("invalid country code"));
    }
    Ok(())
}
