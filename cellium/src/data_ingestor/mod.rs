use std::path::PathBuf;
use std::env;
use rustpython_parser::parser::{parse_program, ParseError};
use rustpython_parser::ast;
use scraper::Html;
use swc_ecma_parser::{Parser, StringInput, Syntax, EsConfig, TsConfig};
use swc_common::{input::SourceFileInput, FileName, SourceMap};

#[derive(Debug, Clone)]
pub struct PythonParser {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

impl PythonParser{
    pub fn new() -> Self {
        Self {
            input_path: PathBuf::from(env::var("DATA_DIR").expect("DATA_DIR not set")),
            output_path: PathBuf::from(format!("{}_output", env::var("DATA_DIR").expect("DATA_DIR not set"))),
        }
    }
    pub fn parse(&self, content: &str, source_file: &str) -> ast::ModModule {
        let ast: ast::ModModule = parse_program(content, "<embedded>")
            .map_err(|e: ParseError| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        ast
    }
}

#[derive(Debug, Clone)]
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

    pub fn parse(&self, content: &str, filename: &str) -> ast::ModModule {
        let ast = PythonParser::new().parse(content, filename);
        Ok(ast.clone())
    }

    pub async fn parse_document(
        &self,
        content: &str,
        filename: &str,
    ) -> Result<String, std::io::Error> {
        let cm = SourceMap::default();
        let source_file = cm.new_source_file(FileName::Custom(filename.to_string()).into(), content.to_string().into());

        match filename.split('.').last().unwrap_or_default() {
            "html" | "htm" => {
                Html::parse_document(content);
                Ok(content.to_string())
            },
            "py" => {
                let ast = parse_program(content, "<embedded>")
                    .map_err(|e: ParseError| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
                Ok(ast.to_string())
            },
            "ts" | "tsx" | "js" | "jsx" => {
                let syntax = if filename.ends_with("ts") || filename.ends_with("tsx") {
                    Syntax::Typescript(TsConfig { tsx: filename.ends_with("tsx"), ..Default::default() })
                } else {
                    Syntax::Es(EsConfig { jsx: filename.ends_with("jsx"), ..Default::default() })
                };
                let mut parser = Parser::new(syntax, SourceFileInput::from(&*source_file), None);
                parser.parse_module()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)));
                Ok(content.to_string())
            },
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported file type")),
        }
    }

    pub fn validate_content(&self, content: &str, filename: &str) -> bool {
        if filename.ends_with(".html") || filename.ends_with(".htm") {
            // Validate HTML
            Html::parse_document(content);
            // If parsing succeeds, consider it valid
            true
        } else if filename.ends_with(".py") {
            // Validate Python
            parse_program(content, "<embedded>").is_ok()
        } else if filename.ends_with(".ts") || filename.ends_with(".js") {
            // Validate TypeScript/JavaScript
            let syntax = if filename.ends_with(".ts") {
                Syntax::Typescript(Default::default())
            } else {
                Syntax::Es(EsConfig::default())
            };
            let mut parser = Parser::new(
                syntax,
                StringInput::new(content, swc_common::BytePos(0), swc_common::BytePos(0)),
                None,
            );
            if parser.parse_module().is_ok() {
                true
            } else {
                false
            }
        } else {
            // Unsupported file type
            false
        }
    }
}