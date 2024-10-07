use warp::Filter;
use std::env;
use crate::data_ingestor::DataIngestor;
use crate::generator::Generator;
use crate::prompt_template::PromptTemplate;
use crate::state_machine::StateMachine;
use serde::Deserialize;
use crate::errors::AppError; // We'll define a custom error type in a new module
use warp::http::StatusCode;

pub async fn run_server() {
    // Load server configuration from environment variables
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // Build routes
    let routes = api_routes();

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

#[derive(Deserialize)]
struct Document {
    content: String,
    filename: String,
}

async fn handle_process_document(doc: Document) -> Result<impl warp::Reply, warp::Rejection> {
    // Initialize components
    let data_ingestor = DataIngestor::new();
    let prompt_template = PromptTemplate::new();
    let mut state_machine = StateMachine::new();
    let generator = Generator::new();

    // 1. Ingest the document
    let parsed_content = data_ingestor
        .parse_document(&doc.content, &doc.filename)
        .await
        .map_err(|e| warp::reject::custom(AppError::IngestionError(e)))?;

    // 2. Update state machine to Parsing state
    state_machine.transition(AgentState::Parsing);

    // 3. Generate modified content
    let prompt = prompt_template.generate_prompt(&parsed_content);
    let generated_content = generator
        .generate_text(&prompt)
        .await
        .map_err(|e| warp::reject::custom(AppError::GenerationError(e)))?;

    // 4. Validate the generated content
    state_machine.transition(AgentState::Validating);
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
    state_machine.transition(AgentState::Completed);

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

// Error handling
impl warp::reject::Reject for AppError {}
