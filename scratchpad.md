# Scratchpad

## Project Status

### Completed

1. **Project Structure Setup**
   - Organized the project into modules corresponding to the major components.
   - Created directories and scaffolded files for:
     - `api`
     - `data_ingestor`
     - `prompt_template`
     - `state_machine`
     - `generator`

2. **Crate Dependencies**
   - Updated `Cargo.toml` with necessary dependencies:
     - Async runtime (`tokio`)
     - HTTP server (`warp`)
     - Environment variable management (`dotenv`)
     - Logging (`env_logger`, `log`)
     - HTTP client (`reqwest`)
     - Parsing libraries (`html5ever`, `scraper`, `rustpython-parser`, `swc_ecma_parser`, `tree-sitter`)
     - Error handling (`thiserror`)

3. **Module Scaffolding**
   - **API Module (`api/mod.rs`):** Implemented server setup and `/process` endpoint.
   - **Data Ingestor (`data_ingestor/mod.rs`):** Implemented `parse_document` and `validate_content` methods with parsing logic.
   - **Prompt Template (`prompt_template/mod.rs`):** Loaded templates from `docs/extra_templates`.
   - **Generator (`generator/mod.rs`):** Implemented `generate_text` method to interact with OpenAI's API.
   - **Error Handling (`errors.rs`):** Defined custom `AppError` enum.

4. **Document Processing Logic**
   - Implemented `handle_process_document` in `api/mod.rs` integrating all components.
   - Added state transitions using `StateMachine`.

5. **Parsing and Validation Logic**
   - Integrated parsing for HTML, Python, JavaScript, and TypeScript files.
   - Implemented syntax validation for each file type.

6. **Environment Variables**
   - Configured the application to use environment variables for configuration:
     - `OPENAI_API_KEY`
     - `OPENAI_BASE_URL`
     - `OPENAI_MODEL`
     - `DATA_DIR`
     - `SERVER_HOST`
     - `SERVER_PORT`
     - `LOG_LEVEL`

### Pending

1. **Testing the API**
   - Need to write unit and integration tests for the `/process` endpoint.
   - Ensure that all components work together as expected.

2. **Adversarial LLM Checking**
   - Implement a secondary LLM to validate the generated content.

3. **Sandbox Testing**
   - Create a sandbox environment to safely execute and test code snippets.

4. **Error Handling Enhancements**
   - Map custom errors to appropriate HTTP status codes and messages.
   - Improve error messages for better debugging and user feedback.

5. **Documentation**
   - Add Rustdoc comments to all public structs, enums, and functions.
   - Provide usage examples in the documentation.

6. **Regular Updates**
   - Continue updating `scratchpad.md` with progress and decisions.

## Notes and Decisions

- **Parsing Libraries:**
  - Using `html5ever` and `scraper` for HTML parsing.
  - `rustpython-parser` for Python files.
  - `swc_ecma_parser` for JavaScript/TypeScript parsing.

- **Validation Criteria:**
  - Modified documents must be syntactically correct and reintegrate without errors.
  - Considering adversarial LLM checking and sandbox testing for additional validation.

- **Prompt Templates:**
  - Integrated extra templates from `docs/extra_templates`.
  - The templates provide personas and tasks for different agents.

- **State Machine:**
  - Manages agent states: `Ingesting`, `Parsing`, `Generating`, `Validating`, `Completed`.

## Questions

- **Testing Strategy:**
  - Determine the best approach for unit and integration testing of asynchronous code.
  - Decide on mock data and scenarios for testing the `/process` endpoint.

- **Adversarial LLM Integration:**
  - How should the adversarial LLM be integrated into the processing pipeline?
  - Define the criteria it will use to validate the generated content.

- **Sandbox Environment:**
  - Explore options for sandboxing code execution in Rust.
  - Ensure safety and security when executing untrusted code.

---

### Next Steps

1. **Testing the API**

   - **Unit Tests:**
     - Use `warp::test` utilities to write tests for the `/process` endpoint.
     - Test with different input files (HTML, Python, JavaScript, TypeScript).
     - Verify that the API returns the expected responses.

   - **Integration Tests:**
     - Simulate end-to-end processing of documents.
     - Test error handling by providing malformed inputs.

2. **Implement Adversarial LLM Checking**

   - **Generator Module:**
     - Extend the `Generator` to interface with a secondary LLM.
     - The adversarial LLM will analyze the generated content for correctness.

   - **Processing Pipeline:**
     - Integrate the adversarial check after content generation and before validation.

3. **Develop Sandbox Testing Module**

   - **Sandbox Module:**
     - Create a new module (e.g., `sandbox/mod.rs`) for code execution.
     - Use virtualization or isolated environments to run code safely.

   - **Integration:**
     - Incorporate sandbox testing into the validation process.

4. **Enhance Error Handling**

   - **HTTP Status Codes:**
     - Map `AppError` variants to appropriate HTTP status codes in `api/mod.rs`.
     - Provide meaningful error messages in API responses.

   - **Logging:**
     - Use logging at different levels (`info`, `debug`, `error`) to aid development.

5. **Documentation**

   - **Comments:**
     - Add Rustdoc comments to all public functions and structs.
     - Explain the purpose and usage of each component.

   - **Examples:**
     - Provide code examples in the documentation where appropriate.

6. **Update `scratchpad.md` Regularly**

   - Record progress, decisions, and any obstacles encountered.
   - Use it as a collaborative tool to track the development process.

---

By addressing these tasks, we can ensure that the API is tested thoroughly and aligns with the development plan. Focusing on testing and documentation will solidify the foundation of the project before moving on to additional features.

Let me know if there are any adjustments or if you have preferences for the next steps.