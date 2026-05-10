use yew::prelude::*;      
use web_sys::HtmlInputElement;      

#[derive(Clone, PartialEq)]

struct Project {
    title: String,
    description: String,
    link: String,
    stack: String,
}

#[derive(Clone, PartialEq)]
enum ProjectChoice {
    Nano_RAG,
    Database_engine,
    RTOS,
    
}

impl ProjectChoice {
    fn to_project(&self) -> Project {
        match self {
            ProjectChoice::Nano_RAG => Project {
                title: "High performance C++ RAG Engine".to_string(),
                description: "Containerized local LLM inference engine built from scratch in C++ -- Zero Python overhead.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/Nano-RAG-CPP".to_string(),
                stack: "C++, Llama.cpp, GGUF, Docker, Linux, RAG".to_string(),
            },
            ProjectChoice::Database_engine => Project {
                title: "Database Engine in C.".to_string(),
                description: "Minimal in-memory database engine inspired by SQLite and MySQL internals.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/DataBase_Engine_in_C".to_string(),
                stack: "C, DBMS, REPL, Binary Search, Parsing".to_string(),

            }
            ProjectChoice::RTOS => Project {
                title: "Embedded RTOS Simulator.".to_string(),
                description: "Full RTOS simulator implementing OS internals from scratch.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/Embedded-RTOS-Simulator-C-Learning-Project".to_string(),
                stack: "C, Makefile, Scheduler, IPC, Semaphores, Interrupts".to_string(),
            }
        }
    }
}


#[derive(Debug)]
enum Cmd {
    Help,
    AboutMe,
    Projects,
    Research,
    Experience,
    Skills,
    Exit,
}

#[function_component(App)]
pub fn app() -> Html {
    // MEMORY
    // create a list (Vec) of strings to store the terminal history

    let history = use_state(|| vec![String::from("Welcome. Type 'help' to see the commands.")]);

    let input_cmd = use_state(|| String::new());


    /*
     on_input: update  ip_cmd aas user types
     use web_sys::HtmlInputElement;      
    */ 

    let on_input = {
        let input_clone= input_cmd.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_info();
            input_clone.set(input.value());
        }) 
    };

    let enter_cmd = {
        let history_clone = history.clone();
        let input_clone = input_cmd.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let user_cmd = (*input_clone).trim().to_lowercase();
                let mut new_history = (*history_clone).clone();

                // Add what the user typed to the screen
                new_history.push(format!("hitesh@portfolio:~$ {}", user_cmd));

                match user_cmd.as_str() 
            }
        })
    }


    html! {
        <div class = "terminal">
        {
            // to display memory
            history.iter().map(|line| {
                html! { <div class = "line"> { line }</div> }
            }).collect::<Html>()
        }
        </div>
    }
    
}

fn main() {
    yew::Renderer::<App>::new().render();
}