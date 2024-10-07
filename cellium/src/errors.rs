use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Data ingestion failed: {0}")]
    IngestionError(std::io::Error),
    #[error("Content generation failed: {0}")]
    GenerationError(reqwest::Error),
    #[error("Validation failed.")]
    ValidationError,
    #[error("Unsupported file type.")]
    UnsupportedFileType,
    #[error("Parsing error: {0}")]
    ParsingError(String),
    // Add more custom error variants as needed
}