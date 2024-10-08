use warp::http::StatusCode;
use warp::test::request;
use serde_json::json;
use cellium::api::{api_routes, Document};
use cellium::errors::AppError;
use tokio;

#[tokio::test]
async fn test_process_endpoint_valid_html() {
    // Prepare test data
    let doc = Document {
        content: "<html><body><p>Hello, world!</p></body></html>".to_string(),
        filename: "test.html".to_string(),
    };

    // Convert the Document to JSON
    let doc_json = serde_json::to_string(&doc).unwrap();

    // Build API filter
    let api = cellium::api_routes();

    // Make a POST request to /process
    let resp = request()
        .method("POST")
        .path("/process")
        .header("Content-Type", "application/json")
        .body(doc_json)
        .reply(&api)
        .await;

    // Assert the response status is OK
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse the response body
    let resp_doc: Document = serde_json::from_slice(resp.body()).unwrap();

    // Verify that the response document has content
    assert!(!resp_doc.content.is_empty());
    assert_eq!(resp_doc.filename, "test.html");
}

#[tokio::test]
async fn test_process_endpoint_invalid_file_type() {
    // Prepare test data with unsupported file extension
    let doc = Document {
        content: "This is some content.".to_string(),
        filename: "test.unsupported".to_string(),
    };

    // Convert the Document to JSON
    let doc_json = serde_json::to_string(&doc).unwrap();

    // Build API filter
    let api = cellium::api_routes();

    // Make a POST request to /process
    let resp = request()
        .method("POST")
        .path("/process")
        .header("Content-Type", "application/json")
        .body(doc_json)
        .reply(&api)
        .await;

    // Assert the response status is 400 Bad Request
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_process_endpoint_invalid_content() {
    // Prepare test data with invalid HTML content
    let doc = Document {
        content: "<html><body><p>Unclosed tag".to_string(),
        filename: "invalid.html".to_string(),
    };

    // Convert the Document to JSON
    let doc_json = serde_json::to_string(&doc).unwrap();

    // Build API filter
    let api = cellium::api_routes();

    // Make a POST request to /process
    let resp = request()
        .method("POST")
        .path("/process")
        .header("Content-Type", "application/json")
        .body(doc_json)
        .reply(&api)
        .await;

    // Assert the response status is 400 Bad Request due to validation failure
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}