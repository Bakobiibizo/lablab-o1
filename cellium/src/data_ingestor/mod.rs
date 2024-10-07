use std::path::PathBuf;

pub struct DataIngestor {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

impl DataIngestor {
    pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            input_path,
            output_path,
        }
    }

    pub async fn ingest_documents(&self) {
        // TODO: Implement logic to parse documents and mirror file structure
    }
}