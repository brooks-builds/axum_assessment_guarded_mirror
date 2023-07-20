use validator::Validate;

// We're adding the Validate derive macro for validating the struct in the middleware. Note that even the required fields are type Option. This is so that the deserialization will succeed even if necessary data is missing. We want to catch that in the validation step rather than the deserialization step.
//
// We could create another struct that doesn't have the Optional options so that the route handler doesn't have to deal with the options. That is outside the scope of this assessment though.
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Default, Validate)]
pub struct CodingEditorRequest {
    #[validate(length(min = 1), required)]
    pub name: Option<String>,
    #[validate(required)]
    pub editor_type: Option<CodingEditorType>,
    #[validate(range(min = 1, max = 5))]
    pub rating: Option<u8>,
    #[validate(required)]
    pub installed: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum CodingEditorType {
    GUI,
    TUI,
}
