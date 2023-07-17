#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct CodingEditorRequest {
    pub name: Option<String>,
    pub editor_type: Option<CodingEditorType>,
    pub rating: Option<u8>,
    pub installed: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum CodingEditorType {
    GUI,
    TUI,
}
