use std::path::PathBuf;
use std::env;

pub struct DataIngestor {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

impl DataIngestor {
    pub fn new() -> Self {
        let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
        let input_path = PathBuf::from(&data_dir);
        let output_path = PathBuf::from(format!("{}_output", &data_dir));

        Self {
            input_path,
            output_path,
        }
    }

    pub async fn ingest_documents(&self) {
        // TODO: Implement logic to parse documents and mirror file structure
    }
}