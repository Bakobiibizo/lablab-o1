use warp::Filter;
use std::env;
use crate::data_ingestor::DataIngestor;
use crate::generator::Generator;
use crate::prompt_template::{self, PromptTemplate};
use crate::state_machine::{self, StateMachine};
use serde::{Deserialize, Serialize};
use crate::errors::AppError; // We'll define a custom error type in a new module
use warp::http::StatusCode;
use warp::Reply;

#[tokio::main]
async fn run_server() {
    // Load server configuration from environment variables
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // Build routes with error handling
    let routes = api_routes().recover(handle_rejection);

    // Start the server
    warp::serve(routes)
        .run((host.parse().unwrap(), port))
        .await;
}

fn api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("process")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_process_document)
}

#[derive(Deserialize, Serialize)]
struct Document {
    content: String,
    filename: String,
}

async fn handle_process_document(doc: Document) -> Result<impl warp::Reply, warp::Rejection> {
    // Initialize components
    let data_ingestor = DataIngestor::new();
    let agent_template = PromptTemplate::new();
    let mut state_machine = StateMachine::new();
    let generator = Generator::new();

    // 1. Ingest the document
    let parsed_content = data_ingestor
        .parse_document(&doc.content, &doc.filename)
        .await
        .map_err(|e| warp::reject::custom(AppError::IngestionError(e)))?;

    // 2. Update state machine to Parsing state
    state_machine.transition(state_machine::AgentState::Parsing);

    // 3. Generate modified content
    let prompt = agent_template.generate_prompt(&parsed_content, "coding_template");
    let generated_content = generator
        .generate_text(&prompt)
        .await
        .map_err(|e| warp::reject::custom(AppError::GenerationError(e)))?;

    // 4. Validate the generated content
    state_machine.transition(state_machine::AgentState::Validating);
    let is_valid = data_ingestor.validate_content(&generated_content, &doc.filename);

    if !is_valid {
        // Handle validation failure
        return Err(warp::reject::custom(AppError::ValidationError));
    }

    // 5. Return the modified document
    let response = Document {
        content: generated_content,
        filename: doc.filename,
    };

    // Update state machine to Completed state
    state_machine.transition(state_machine::AgentState::Completed);
   
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

// Error handling
impl warp::reject::Reject for AppError {}

// Add the following function for error handling
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if let Some(app_error) = err.find::<AppError>() {
        let code;
        let message;

        match app_error {
            AppError::IngestionError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Error ingesting document.";
            },
            AppError::GenerationError(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Error generating content.";
            },
            AppError::ValidationError => {
                code = StatusCode::BAD_REQUEST;
                message = "Validation failed for the generated content.";
            },
            AppError::UnsupportedFileType => {
                code = StatusCode::BAD_REQUEST;
                message = "Unsupported file type.";
            },
            AppError::ParsingError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Error parsing the document.";
            },
        }

        let json = warp::reply::json(&serde_json::json!({"error": message}));
        Ok(warp::reply::with_status(json, code))
    } else {
        // Return 500 Internal Server Error for unhandled errors
        let json = warp::reply::json(&serde_json::json!({"error": "Internal server error"}));
        Ok(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
    }
}
