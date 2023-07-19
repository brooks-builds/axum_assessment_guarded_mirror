use validator::Validate;

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
