use axum_assessment_guarded_mirror::types::{CodingEditorRequest, CodingEditorType};
use eyre::Result;
use reqwest::Client;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn mirrors_validated_json() -> Result<()> {
    let url = format!("{BASE_URL}/mirror");
    let client = Client::new();
    let coding_editor = CodingEditorRequest {
        name: Some("Helix".to_owned()),
        editor_type: Some(CodingEditorType::TUI),
        rating: Some(5),
        installed: Some(true),
    };
    let response = client.post(url).json(&coding_editor).send().await?;
    let status = response.status();
    let expected_status = 200;

    assert_eq!(status, expected_status);

    let response_coding_editor = response.json::<CodingEditorRequest>().await?;

    assert_eq!(response_coding_editor, coding_editor);

    Ok(())
}

#[tokio::test]
async fn responds_with_400_when_nothing_is_passed_in() -> Result<()> {
    let url = format!("{BASE_URL}/mirror");
    let client = Client::new();
    let response = client.post(url).send().await?;
    let status = response.status();
    let expected_status = 400;

    assert_eq!(status, expected_status);

    Ok(())
}

#[tokio::test]
async fn not_guard_unrelated_route() -> Result<()> {
    let url = format!("{BASE_URL}/unrelated");
    let response = reqwest::get(url).await?;
    let status = response.status();
    let expected_status = 200;

    assert_eq!(status, expected_status);
    Ok(())
}

#[tokio::test]
async fn responds_with_400_when_required_part_of_json_is_not_passed_in() -> Result<()> {
    let url = format!("{BASE_URL}/mirror");
    let client = Client::new();
    let json = CodingEditorRequest {
        name: Some("Helix".to_owned()),
        ..Default::default()
    };
    let response = client.post(url).json(&json).send().await?;
    let status = response.status();
    let expected_status = 400;

    assert_eq!(status, expected_status);

    Ok(())
}
