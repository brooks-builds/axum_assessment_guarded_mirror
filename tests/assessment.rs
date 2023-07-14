use eyre::Result;
use reqwest::Client;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn mirrors_validated_json() -> Result<()> {
    let url = format!("{BASE_URL}/mirror");
    let client = Client::new();
    let coding_editor = CodingEditorRequest {
        name: "Helix".to_owned(),
        editor_type: CodingEditorType::TUI,
        rating: 5,
        installed: true,
    };
    let response = client.post(url).json(&coding_editor).send().await?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CodingEditorRequest {
    pub name: String,
    pub editor_type: CodingEditorType,
    pub rating: u8,
    pub installed: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
enum CodingEditorType {
    GUI,
    TUI,
}
