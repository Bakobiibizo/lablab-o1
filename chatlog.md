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