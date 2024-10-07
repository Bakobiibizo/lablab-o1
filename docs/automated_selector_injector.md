Automated Selector Injector
Overview:
The Automated Selector Injector is a proposed service designed to enhance end-to-end test writing capabilities within large codebases, exemplified by Mitacs. It leverages generative AI to analyze HTML code and injects class selectors for interactive components. This tool aims to standardize selector types, facilitating QA specialists in creating more reliable and less fragile tests. The automation of this otherwise labor-intensive task is anticipated to save hundreds of hours of developer time, allowing for a focus on more complex problem-solving.

Proof of Concept:
The proof of concept was developed using Mixtral AI's Mixtral-7x8B model on a GPU accelerated VM. At its core, the Mixtral model is distinguished by its layered architecture, consisting of eight sub-models, each with 7 billion parameters. These sub-models, or "experts," specialize in different tasks, enabling the system to select the most appropriate pair of experts for any given job.

The entire system is powered by A100 accelerators, which are crucial for managing the substantial memory demands of the Mixtral model. This integration ensures that the system operates efficiently, handling the large-scale computations needed for advanced tasks.

Central to the system's operation is the API service, which serves as the primary interface for interacting with the model. Through this API, the system analyzes input prompts, processes them using the selected experts, and generates a corresponding response. This process is facilitated by Llama.cpp, a specialized C library that plays a pivotal role in the system. Llama.cpp functions as a machine learning library, utilizing hardware acceleration to efficiently execute complex computations. These computations involve adjusting a series of weights and biases to accurately predict the next token in a sequence, with a token representing about half an average word. This prediction is based on the contextual meaning of the input prompt and the history of previously generated tokens.

Despite its advanced capabilities, the Mixtral 8x7B is a non-fine tuned foundation model. This means it is not inherently specialized in any particular task but gains specificity through the system's design. This is where the system prompt comes into play. The system prompt, a heavily weighted set of instructions, effectively guides the model in understanding and executing the requested task. This prompt is refined through an iterative process involving a prompt crafting GPT, where each iteration tests the prompt on the model and adjusts it based on the response's accuracy and consistency.

Achieving an impressive accuracy rate of approximately 95%, the system demonstrates its effectiveness in task execution. However, there is potential for further improvement through fine-tuning with a dataset of annotated examples.

On the service side, the system boasts a comprehensive Data Ingestion system. This system is responsible for reading from a filesystem, parsing relevant data, and integrating it with the Agent Management System. The Agent Management System itself is an advanced component that constructs agent objects containing a dynamic list of prompts. These prompts can be swapped out as needed to alter the task being performed, managed by a state manager.

Additionally, the system includes a Generation Management component that handles requests for inference. This component is responsible for formatting prompts for consumption by LLM APIs, making HTTP requests, and returning the results. These results are then fed back into the data ingestion system, where the code is updated with the new responses.

Designed with modularity and extensibility in mind, the system is well-prepared for future enhancements and the addition of new tasks. The overall design philosophy ensures robust and flexible operation, accommodating evolving requirements and technological advancements.


Components:
Note that there are two options for generating model inference that are broken down in the cost comparison sheet. 
The system comprises two principal components:

Option 1:  API with Large Language Model (LLM)
This component, responsible for running inference (computing responses from given prompts), forms the core of the system. It processes HTML code and identifies interactive components, generating appropriate HTML class tags for them.
Technologies: 
Llama.cpp:  A C library that uses hardware acceleration to run inference with the model on the input prompt. 
Ollama: A lightweight API layer built in Go but implemented in NodeJS using Typescript bindings to align with our stack.
Docker: Ollama wraps the entire library and the application comes with a docker container image preconfigured to work for our use case out of the box. 
Sub-Components:
Model Inference: Handled by Llama.cpp
API: Handled by the docker container and Ollama library.
Requirements:
VM: Requires a virtual machine with a minimum of 16 gigs of RAM and preferably multiple processor cores(4+) to facilitate rapid data transfer and fully engage the accelerator.  
GPU accelerator: This requires a GPU accelerator compatible with CUDA with a minimum of 50gb of VRAM. These are not cheap but the price can be mitigated by preparing data before runtime and running the data through the system in a serialized chain before tearing down the virtual machine to avoid having the GPU running idle. 


Option 2:  Privately Hosted GPT-4 Azure Instance(LLM)
This component would have the same functionality of the previous option but all the infrastructure would be handled by Azure. It would be a privately available API that would run model inference on GPT-4 in a secure environment that would accept the same request made by our Generator module and return results it could process. An initial setup would be done, API keys would be distributed and managed and no other setup or maintenance would be required. It would be significantly easier using this option and would eliminate any specialized knowledge requirements for deployment of hardware or how to implement model inference. It also would have the bonus of providing continuous uptime while only charging us for usage. This enables us to skip data preprocessing and arbitrarily generate new html tags without having to gather enough data to justify the deployment of the system.  It uses the OpenAI standard for API requests which is the most common and approachable framework for interacting with AI APIs.

TypeScript Application: This application ingests folders, identifies HTML files, parses the code, and maps it to a JSON object. It then integrates with the Agent Management system to construct prompts for the LLM. The application is designed to operate on modest hardware, with the heavy lifting being done by the LLM API.
Technologies: 
NodeJS: The application is built using NodeJS to integrate with our stack.
Axios: The API client. Chosen arbitrarily, this can be swapped out with any other api client if needed.
Bable: Typescript parser that is used in the Ingestion model to parse typescript out of components containing HTML or JSX. The MOL code base does not contain these types of elements but I felt it was prudent to be able to handle all types of possible HTML data.
Sub-Components:
Data Ingestion: Reads and parses data from file systems, passing it to the Agent Management System.
Agent Management: Creates PromptList objects to contain prompts for dynamic task dependent context switching, VariableList objects to enable dynamic variables within the prompts for values that may change during runtime, and Agent objects that contain all the sub components and is planned to have a state management system to facilitate complex task orchestration. 
Generation Management: Contains the HTTPClient and handles formatting requests for different kinds of LLM APIs. Currently compatible with the OpenAI, Anthropic, and Ollama API's.
Requirements:
VM: To implement this as a service callable within the mol ecosystem we will need to have this mounted on a virtual machine with an api endpoint. Because the system is structured in a manner that puts all of the heavy lifting onto the LLM API component this virtual machine does not require much more than a CPU and ram to handle the requests and data parsing. 
API:  Current implementation does not have this served as an API service, it was built as a client. Some time will need to be spent developing a robust api service that aligns with our other available services to enable on demand access to the system. 

Cost:

Option 1:  API with Large Language Model (LLM)

Estimated Total / page 
18500 
$0.00003 
$0.3725 
Estimated Num. of pages 
 
 
220 
Approx. Total Cost 
 
 
$81.95 






Option 2:  Privately Hosted GPT-4 Azure Instance(LLM)


Variables 
Values 
Est. Inference Time in mins / page 
3 
Est. Total Pages 
220 
Est. Total Minutes 
 
660 
Est. Total Hours + 1 hour for setup and takedown 
12 
Est. Cost / hour 
$3.670 
Estimated Total Cost 
$44.04 

 
For a more detailed cost analysis please see Appendix A.

Benefits:

Efficiency: Automates repetitive tasks, freeing up developer time for more complex challenges.
Accuracy: Refined prompts yield approximately 95% accuracy in selector identification.
Flexibility: Modular and extensible design allows easy integration of future tasks.

Future Prospects (Blue Sky):
The robustness of the LLM API opens avenues for further automations, such as comprehensive code documentation, enhancing reference materials, and potentially could be enhanced with a vector database integrating a centralized knowledge repository for internal use. This could significantly streamline standard operating procedures and improve internal knowledge management.

Conclusion:
The Automated Selector Injector as a proof of concept represents a new tool that can be used to enhance the existing code base of Mitacs and enable a comprehensive e2e testing framework while eliminating a large amount of development hours required. It has beendesign ed with modularity in mind so additional enhancements can be done with minimal development opening up a variety of wide spread enhancements to the Mitacs ecosystem. 





APPENDIX A - Detailed Cost Analysis

Option 1: Open-Source Model with GPU VM 
 
The first option is to run an open-source model. We would recommend Mistral AI's MixtraI8x7B which is under an Apache2.0 License. It is the highest performing open-source foundation model beating GPT-3.5 on most benchmarks. While it does not score as high as GPT-4 across the board it is well suited to narrowly scoped tasks like this, and we have tested it with our own code base with almost no errors.  
 
Because it is a larger model this does require a pricey GPU, but we can prepare the data and run continuous inference to avoid incurring costs while the GPU is idle. The model can be easily deployed as an API using a library like Ollama along with typescript bindings from Model Fusion. Both libraries are under an MIT License (2). They are lightweight, dockerized and are configured to not retain logs or store information. This is a cheaper alternative but does require setup and deployment, though scripting can also automate the process with systems like Jenkins. 
 
Pricing for Azure VM with GPU: 

Instance
Core(s)
RAM
Temporary storage
GPU
Pay as you go with AHB
NC24ads A100 v4
24
220 GiB
958 GiB
1X A100
$3.670/hour

 
Variables
Values
Est. Inference Time in mins / page
3
Est. Total Pages
220
Est. Total Minutes


660
Est. Total Hours + 1 hour for setup and takedown
12
Est. Cost / hour
$3.670
Estimated Total Cost
$44.04


 
  


Option 2: Private GPT-4 API 
 
Since we are in the Azure ecosystem there is the option to use a private GPT instance through their existing API infrastructure. GPT4 is a highly performant model that makes few errors and is extremely well adapted to this prompting technique. But it does come at a slightly higher cost. The upside is minimal setup and tear down and a more approachable user experience. 
 
Pricing for private GPT-4 Instance 

Models
Context
Prompt (Per 1000 tokens)
Completion (Per 1000 tokens)
GPT-3.5-Turbo
4K
N/A
N/A
GPT-3.5-Turbo
16K
N/A
N/A
GPT-3.5-Turbo-1106
16K
$0.001
$0.002
GPT-4-Turbo
128K
$0.01
$0.03
GPT-4-Turbo-Vision
128K
$0.01
$0.03

 
The base prompt is approximately 350 tokens. An average page would be about 6000 tokens. The returned response would be the page total plus the selectors at around 300 tokens or 5% the size. 
 

Component
Token amount
Token cost
Total cost
Prompt
350
$0.00001
$0.0035
Large Page Response
9000
$0.00001
$0.09
Estimated Total / page
18500
$0.00003
$0.3725
Estimated Num. of pages




220
Approx. Total Cost




$81.95

 
With approximately 220 different pages of html elements to insert tags into it would be $81.95 for coverage of the entire MOL codebase with an upkeep cost of $0.37 per new html page added to the database. 


The system we were testing on used an A100 and the average inference time is approximately 3 minutes for a similar page to the previous option. To complete 220 pages, it would take 11 hours. That would come to $40.40 for inference, or $44.07 given an hour for setup and teardown. 
 The same 220 pages of html allowing 1 hour per page would take 220 man-hours, averaging out for complexity. 
 

