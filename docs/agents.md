Agent Technical Overview
The system is made up of two distinct systems. The api host and agent package. The agent package is made up of a number of modules.
Agent Package
Modules
Agents:
This is primarily responsible for bring together the other modules to construct an 'agent' which will contain the instructions and state manager for completion of a task.  
Data Ingestor:
This ingests all relevant documents containing html; keeps track of their file location and parses the html out of them. The html is passed to the prompt scheduler and the file system and document data is passed to the response parser.
Prompt Template:
This houses all the prompts required to complete a task, and has placed any variables that may change dynamically during runtime. Additionally it stores a persona that is a general set of context for the model.
DynamicVariables:
Contains an variable label pair that maps to variables inserted into the prompts. This allows for dynamic updating of those variables during runtime. 
PromptScheduler:
This has the StateManager that is responsible for swapping the context of the model depending on the step of the task that it currently is on. A converted is used to convert the data into the correct format for api.
Generators:
The API client responsible for making calls to the appropriate API and routing the response to the response parser. 
ResponseParser:
This takes the original document information and the api response, parses that response and injects the new html data into the position of the original document and returns it. 
How it works
Data Ingestor
A file path is passed into the data injector which walks the file system and collects all relevant file types. It is designed to collect html, typescript and javascript documents it creates a mirror directory of the original files in their relative position as a back up and then parses the documents inside. It is configured to work with HTMLl, JSX, and all existing Angular templating systems using the Typscript parser directly to walk the document for more complex document types.
Prompt Templates
The prompts are broken into multiple sub prompts that are narrowly scoped. This is critically important to achieving a high accuracy in your responses. The prompts should focus on one particular sub task that can be briefly explained and accompanied by an example of the task request and the appropriate response. 
These prompts are a single string organized into an prompt template object with a label for future access.
Dynamic Variables
I have designed the system to enable dynamic variable injection during the request process. This is useful for data collection or other information that may dynamically change during the requests.






Prompt Scheduler
All those prompts are fed into a prompt scheduler that has a state manager to keep track of the step of the task that the system is on. In addition there is a base persona that explains to the model that it is a specialist on that particular task and impresses the importance of that task to the model. This needs to be generic so that it can be prepended to all of the prompts as a primer for the model to understand the minimal context required to complete the task. 

The prompt scheduler also intakes any outside data and incorporates that input data along with any dynamic variables using a prompt conversion function that formats it to be accepted by the API.




Generator
Those prompts are passed to the generator on a schedule that is managed by the state manager. An api call is made using a request function and awaits a response. 
Response Parser
The response is collected and passed to the response parser. 
The original document contents and positional data is collected by that data ingestor and then stored in the response parser until it is ready for injection.
The response data is parsed and the new data is extracted and then injected into the position of the original data in the original document and then placed in the correct file location. 

