USER: @cellium 
Please review the project and begin development

AGENT: Project Review and Development Proposal
---
Project Review
The cellium project is currently initialized with the following configuration:
Proposed Development Plan
Based on the project outline and requirements, here is a proposed plan to begin development:
Project Structure Setup
Organize the project into modules corresponding to the major components:
```
   cellium/
   ├── Cargo.toml
   └── src/
       ├── main.rs
       ├── api/
       │   └── mod.rs
       ├── data_ingestor/
       │   └── mod.rs
       ├── prompt_template/
       │   └── mod.rs
       ├── state_machine/
       │   └── mod.rs
       └── generator/
           └── mod.rs
```
Crate Dependencies
Update Cargo.toml with necessary dependencies:
Module Scaffolding
a. API Module (api/)
Define asynchronous request handlers for receiving and returning documents.
Set up routing and server initialization using warp.
b. Data Ingestor Module (data_ingestor/)
Define parsers for HTML, HTMX, Python, TypeScript, and JavaScript files.
Implement structures to represent parsed documents and file hierarchy mirroring.
c. Prompt Template Module (prompt_template/)
Create a template system with support for dynamic variables and state-based instructions.
Allow for easy swapping of prompts for different tasks.
d. State Machine Module (state_machine/)
Implement a state machine to manage agent contexts using the async-fsm pattern.
Define states, transitions, and actions based on the workflow steps.
e. Generator Module (generator/)
Set up communication with OpenAI's API using reqwest.
Implement functions to send prompts and receive generated text asynchronously.
Focus on API Development
Begin implementing the API module to establish the core workflow.
Set up basic endpoints and test the request-response cycle with mock data.
Testing Infrastructure
Use tokio::test for asynchronous unit tests.
Scaffold tests for each module to ensure functionality and catch regressions early.
Documentation and Scratchpad Update
Document the code using Rustdoc comments.
Update scratchpad.md with the current state and decisions made.
Key Considerations
Async Concurrency: Utilize tokio for managing asynchronous tasks and I/O operations across modules.
Error Handling: Implement robust error handling using Result and custom error types where appropriate.
Modularity: Ensure code is organized into reusable functions and modules to promote scalability and maintainability.
Configuration Management: Use environment variables for configurations like API keys and URLs, possibly with the dotenv crate.
Next Steps
Please review this development plan and let me know if it meets your expectations or if any adjustments are needed. Upon your approval, I will proceed to set up the project structure and scaffold the modules as outlined.

USER: The development plan looks good lets proceed

AGENT: Module Scaffolding
1. API Module (api/mod.rs)
Scaffold the api module with placeholders for server setup and route definitions.
2. Data Ingestor Module (data_ingestor/mod.rs)
Scaffold the data_ingestor module with structures and methods for parsing documents.
3. Prompt Template Module (prompt_template/mod.rs)
Scaffold the prompt_template module to manage prompt templates.
4. State Machine Module (state_machine/mod.rs)
Scaffold the state_machine module with an enum for agent states and a state machine struct.
5. Generator Module (generator/mod.rs)
Scaffold the generator module with functions for LLM text generation.
Next Steps
Integrate Modules into main.rs: Update main.rs to include and initialize the modules.
Focus on API Development: Begin implementing the API module to set up the server and define endpoints.
Scaffold Tests: Create test files for each module and write unit tests using #[tokio::test].
---
Proposal
With the modules scaffolded, I propose to focus on the API component next, as per the development plan. This will involve:
Setting up the Warp server and defining routes for receiving documents.
Implementing request handlers that interact with the data ingestor.
Ensuring that the API can accept documents and return responses, even if the internal logic is mocked at this stage.
Please let me know if you approve proceeding with the API implementation or if you have any adjustments to suggest.

USER: additional requirements for the api have been added to the .env file located in the root of the project. 
These are the variables:
- OPENAI_API_KEY
- OPENAI_BASE_URL
- OPENAI_MODEL
- DATA_DIR

I believe that covers what you need but let me know if i missed anything

AGENT: Acknowledging Environment Variables and Proceeding
---
Thank you for providing the additional environment variables:
OPENAI_API_KEY
OPENAI_BASE_URL
OPENAI_MODEL
DATA_DIR
These variables cover the key configurations needed for the API and Generator modules, particularly for interacting with the OpenAI API and specifying the data directories.
Possible Additional Environment Variables
To ensure smooth operation and configurability of the API, we might consider adding the following environment variables:
SERVER_HOST: The hostname or IP address for the API server to bind to (e.g., 127.0.0.1 or 0.0.0.0).
SERVER_PORT: The port number on which the API server will listen (e.g., 8080).
LOG_LEVEL: The logging level for the application (e.g., info, debug, error).
Would you like to include these additional environment variables?
---
Next Steps
With your approval, I will proceed to implement the API module, focusing on:
Server Setup: Configuring the Warp server to use SERVER_HOST and SERVER_PORT from the environment.
Endpoint Definitions: Defining endpoints for receiving documents and returning modified documents.
Environment Variable Handling: Loading environment variables securely and making them accessible throughout the application.
Integration with Other Modules: Ensuring that the API interacts correctly with the DataIngestor and other components.
Please let me know if you agree with adding the suggested environment variables or if you have any other preferences before we proceed

USER: I have added the variables as provided. The log level value is set to DEBUG, I am used to the pythonic norms but if this is not aligned with rust let me know. plan approved

Implementing the API Module
---
Based on your approval, I've proceeded to implement the API module, focusing on:
Server Setup: Configuring the Warp server to use SERVER_HOST and SERVER_PORT from the environment.
Endpoint Definitions: Defining endpoints for receiving documents and returning modified documents.
Environment Variable Handling: Loading environment variables securely and making them accessible throughout the application.
Integration with Other Modules: Ensuring that the API interacts correctly with the DataIngestor and other components.
Updates
1. cellium/src/main.rs
Updated main.rs to initialize the asynchronous runtime, load environment variables, and start the API server.
2. cellium/src/api/mod.rs
Implemented the server setup, defined the /process endpoint, and configured the server to use SERVER_HOST and SERVER_PORT.
Key Changes:
Server Configuration: The server now reads SERVER_HOST and SERVER_PORT from environment variables.
Route Definition: Added a /process endpoint that accepts a POST request with a JSON body containing the document.
Request Handler: Created handle_process_document to process incoming documents using the various components.
3. cellium/Cargo.toml
Added the dotenv crate to manage environment variables and updated warp version to 0.3.
4. cellium/src/generator/mod.rs
Updated the Generator to utilize the environment variables for API configuration.
Key Changes:
Loaded OPENAI_API_KEY, OPENAI_BASE_URL, and OPENAI_MODEL from environment variables.
Updated the Generator::new() method to initialize with these configurations.
5. cellium/src/data_ingestor/mod.rs
Adjusted DataIngestor to use the DATA_DIR environment variable.
Key Changes:
Loaded DATA_DIR from the environment variables.
Initialized input_path and output_path based on DATA_DIR.
6. Logging Setup (Optional)
To align with the Rust conventions for logging, we can use the env_logger crate and configure it with the LOG_LEVEL environment variable.
Add to Cargo.toml:
Update main.rs:
Note: The Rust logging levels are: Error, Warn, Info, Debug, and Trace. Setting LOG_LEVEL to Debug is acceptable and aligns with Rust conventions.
---
Next Steps
Implement Processing Logic:
In handle_process_document, implement the logic to process documents using the DataIngestor, StateMachine, PromptTemplate, and Generator.
Handle asynchronous operations and ensure proper error handling.
Enhance Error Handling:
Use Result and Option types effectively.
Implement custom error types if necessary.
Add Unit Tests:
Write tests for the API endpoints using warp::test utilities.
Implement tests for the other modules using #[tokio::test].
Update Documentation:
Add Rustdoc comments to public structs and functions.
Ensure code is well-documented for maintainability.
Update Scratchpad (scratchpad.md):
Document the current state of the project.
Note any decisions or assumptions made during development.
---
Commit Message
feat(api): Implement API server setup with environment variables and define /process endpoint
---
Please let me know if you have any questions or if further adjustments are needed.

USER: Everything looks good to me, please proceed.

AGENT: Implementing Document Processing Logic
---
Now that we've set up the API and have received your approval, I'll proceed to implement the processing logic in the handle_process_document function of the api module.
Updates
1. cellium/src/api/mod.rs
Implemented the logic inside handle_process_document to process incoming documents using the defined modules.
Key Changes:
Ingest Document: Called data_ingestor.parse_document to parse the incoming document content, with proper error handling.
State Machine Transition: Updated the state machine to track processing stages.
Generate Modified Content: Created a prompt using prompt_template and generated text using generator, handling errors.
Validation: Used data_ingestor.validate_content to validate the generated content.
Error Handling: Implemented custom error handling using a new AppError type.
Return Modified Document: Constructed a response containing the modified content and set the HTTP status code.
2. cellium/src/data_ingestor/mod.rs
Added methods to parse documents and validate content.
3. cellium/src/prompt_template/mod.rs
Added a method to generate prompts based on parsed content.
4. cellium/src/generator/mod.rs
Implemented the generate_text method to interact with OpenAI's API.
Key Changes:
Implemented API call to OpenAI using reqwest.
Parsed the JSON response to extract the generated text.
Handled errors appropriately.
5. cellium/src/errors.rs
Created a new module for custom error types.
Note: Add the thiserror crate to Cargo.toml:
6. cellium/src/lib.rs
Export the modules to ensure they are accessible throughout the crate.
---
Commit Message
---
Next Steps
1. Enhance Error Handling
Detailed Errors: Expand AppError to cover more specific error cases.
Error Responses: Map custom errors to appropriate HTTP status codes and messages.
2. Implement Parsing and Validation Logic
parse_document: Implement actual parsing based on file type (HTML, HTMX, Python, TypeScript, JavaScript).
validate_content: Develop logic to verify that the generated content meets the required criteria (e.g., correct tags injected, syntactically valid HTML).
3. Unit Testing
API Tests:
Use warp::test utilities to write tests for the /process endpoint.
Test cases for successful processing, validation failures, and error handling.
Module Tests:
Write unit tests for DataIngestor, PromptTemplate, Generator, and StateMachine using #[tokio::test].
4. Documentation
Comments:
Add Rustdoc comments to all public structs, enums, and functions.
Usage Examples:
Provide examples on how to use the API and modules in the documentation.
5. Update scratchpad.md
Document the current implementation status.
Note any assumptions, decisions, or areas needing further discussion.
---
Questions and Clarifications
Parsing Logic: Do you have specific parsing libraries or approaches in mind for the different file types? For example, should we use html5ever for HTML parsing?
Validation Criteria: Could you provide more details on what constitutes a valid modified document? Specific rules or checks to implement would be helpful.
Prompt Templates: Do you have existing prompt templates or examples that we should incorporate into the PromptTemplate module?
---
Please let me know if you approve of these changes and next steps or if you have any additional requirements or preferences.

USER:
Question responses:
- I am still new to the rust ecosystem so I am not fully up to speed on appropriate libraries, I do know there are a number of linters and parsers as rust is known as one of the fastest processing languages for this task, perhaps we can consider those. Otherwise i differ to your judgement. 
- Ah, great question. So the idea of this example agent is to be able to send in a functioning file to the api and have it inject the appropriate selectors for playwright to reference when running automated end to end testing. What qualifies as a valid document is one that can be inserted back to where it was grabbed from and it functions correctly with out error. To achieve this I planned to have an anversairal llm double check the output along with a sandbox shell for the agent to test the code in or at least validate that it is legal code. But I am always open to ideas if you have any. 
- I actually have a couple that we can add. They are more generic personas since there is no specific task associated with them but they are a reasonable starting place. Later on we can select tasks for the agents and build out the finite states of the task for additional context for the model performing the task. You can find them @extra_templates  

Approved changes, I only need the commit message for the last changes first

AGENT:
Commit Message
`feat(api): Implement document processing logic and error handling in handle_process_document`
---
Proceeding with Next Steps
---
Addressing Your Responses
1. Parsing Logic
Since you're open to suggestions for appropriate libraries, we'll utilize Rust crates for parsing the different file types:
HTML/HTMX Parsing: We'll use html5ever and scraper crates, which are efficient for parsing HTML documents.
Python Parsing: We'll use rustpython_parser for parsing Python files.
JavaScript/TypeScript Parsing: We'll leverage swc_ecma_parser or tree-sitter for parsing JavaScript and TypeScript code.
We'll update Cargo.toml accordingly.
2. Validation Criteria
To ensure that modified documents function correctly without errors:
Syntax Validation: We'll use linters and parsers to validate the syntax of the modified code.
Adversarial LLM Checking: We'll integrate a secondary LLM to review the output.
Sandbox Testing: We'll implement a sandbox environment to test the code, ensuring it can be reinserted without breaking functionality.
3. Prompt Templates
We'll incorporate the prompt templates you've provided in the @extra_templates directory into the PromptTemplate module. This will allow us to dynamically generate prompts based on different personas and tasks.
---
Updates
1. Updating Cargo.toml
Add the necessary dependencies for parsing and validation:
2. Implementing Parsing Logic in DataIngestor
Updated parse_document to handle different file types:
3. Implementing Validation Logic
Enhanced validate_content to perform syntax checking:
4. Updating handle_process_document in api/mod.rs
Modified the call to validate_content to include filename:
5. Incorporating Prompt Templates
Updated the PromptTemplate module to load templates from the extra_templates directory:
Note: Converted templates to JSON files for safe parsing.
6. Error Handling Update
Adjusted AppError to include parsing and validation errors:
---
Commit Message
`feat(data_ingestor): Implement parsing and validation logic for multiple file types
feat(prompt_template): Incorporate prompt templates from extra_templates directory`
---
Next Steps
1. Implement Adversarial LLM Checking
Generator Module: Update the Generator to interface with an adversarial LLM for output validation.
2. Sandbox Testing
Sandbox Module: Create a new module to safely execute and test code snippets.
3. Enhance Error Handling
Map custom errors to appropriate HTTP status codes in api/mod.rs.
4. Testing
Write unit and integration tests for the new parsing and validation logic.
5. Documentation
Update documentation to reflect changes and provide usage examples.
---
Please let me know if you approve of these changes or if you have any additional requirements or suggestions.