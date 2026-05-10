use yew::prelude::*;      
use web_sys::HtmlInputElement;      

#[derive(Clone, PartialEq)]

struct Project {
    title: String,
    description: String,
    link: String,
    stack: String,
}



impl Project {
    // No &self => returns all projects at once
    fn get_all() -> Vec<Project> {
        vec![
            Project {
                title: "High-Performance C++ RAG Engine".to_string(),
                description: "Local LLM inference built from scratch. Zero Python overhead.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/Nano-RAG-CPP".to_string(),
                stack: "C++, Llama.cpp, GGUF, Docker, Linux".to_string(),
            },
            Project {
                title: "Embedded RTOS Simulator".to_string(),
                description: "Full RTOS implementing scheduling, IPC, semaphores from scratch.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/Embedded-RTOS-Simulator-C-Learning-Project".to_string(),
                stack: "C, Makefile, Scheduler, IPC, Semaphores".to_string(),
            },
            Project {
                title: "Mini Database Engine in C".to_string(),
                description: "SQLite-inspired REPL engine with binary search lookups.".to_string(),
                link: "https://github.com/hitesh-bhatnagar/DataBase_Engine_in_C".to_string(),
                stack: "C, DBMS, REPL, Binary Search, Parsing".to_string(),
            },
        ]
    }
}


#[derive(Clone, PartialEq)]
struct Research {
    title: String,
    tag: String,
    publisher: String,
    link: String,
}



impl Research {
    fn get_all() -> Vec<Research> {
        vec![
            Research {
                title: "Design of Logistic Distance Metric Based Robust Adaptive Filter for Active Noise Control".to_string(),
                tag: "Deep Learning, Signal Processing".to_string(),
                publisher: "Digital Signal Processing (Elsevier Journal), 2026".to_string(),
                link: "#".to_string(), // Replace with actual link when published
            },
            Research {
                title: "A Novel Three-Layer Hybrid Cryptographic Framework using Enhanced Classical Ciphers".to_string(),
                tag: "Cryptography".to_string(),
                publisher: "ICAAIC 2025, IEEE Conference, India".to_string(),
                link: "#".to_string(), // Replace with actual link
            },
            Research {
                title: "Graph-Based Zero-Day IoT Botnet Detection".to_string(),
                tag: "Machine Learning, IoT Security".to_string(),
                publisher: "NEleX 2026, VIT (Provisionally Accepted)".to_string(),
                link: "#".to_string(), // Replace with actual link
            }
        ]
    }
}

#[derive(Clone, PartialEq)]
struct Experience {
    role: String,
    company: String,
    duration: String,
    tasks: Vec<String>,
}

impl Experience {
    // No &self here => called as Experience::get_all()
    fn get_all() -> Vec<Experience> {
        vec![
            Experience {
                role: "IT Risk Advisory Intern".to_string(),
                company: "Aumyaa Consulting Services LLP".to_string(),
                duration: "Feb 2026 - Present".to_string(),
                tasks: vec![
                    "Executed ITGC audits across Access, Change Management, and IT Operations for ONGC and BMW.".to_string(),
                    "Performed UAR, SOD, and approvals testing ensuring roles were provisioned correctly.".to_string(),
                    "Analyzed change requests and job logs, identifying gaps in monitoring controls.".to_string(),
                    "Built Python-based utilities for PII masking, audit trail review, and anomaly flagging.".to_string(),
                ],
            },
            Experience {
                role: "Research Intern - Deep Learning & Audio".to_string(),
                company: "IIT Goa".to_string(),
                duration: "May 2025 - Jul 2025".to_string(),
                tasks: vec![
                    "Pioneered a hybrid Deep Learning Active Noise Cancellation (ANC) model (SFANC).".to_string(),
                    "Incorporated adaptive learning-rate scheduling and momentum updates in PyTorch.".to_string(),
                    "Designed a lightweight CNN for noise-type classification and filter switching.".to_string(),
                    "Attained performance gains with MSE=2.9e-5, RMSE=0.00537, and SNR=27.10 dB.".to_string(),
                ],
            },
        ]
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

impl Cmd {
    fn from_str(input: &str) -> Option<Cmd> {
        match input {
            "help" => Some(Cmd::Help),
            "about" => Some(Cmd::AboutMe),
            "projects" => Some(Cmd::Projects),
            "research" => Some(Cmd::Research),
            "experience" => Some(Cmd::Experience),
            "skills" => Some(Cmd::Skills),
            "exit" => Some(Cmd::Exit),
            _ => None,
        }
    }
}



#[function_component(App)]
pub fn app() -> Html {
    // MEMORY
    // create a list (Vec) of strings to store the terminal history
    // use_state: React hook to store changeable data (Vec<String> = list of lines)
    // || vec![...] = initial value (welcome message)
    let history = use_state(|| vec![html! { <div class="welcome">{"Welcome. Type 'help' to begin."}</div> }]);

    let input_cmd = use_state(|| String::new());


    /*
     on_input: update  ip_cmd aas user types
     use web_sys::HtmlInputElement;      
    */ 

    let on_input = {
        let input_clone= input_cmd.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_clone.set(input.value());
        }) 
    };

    let on_enter = {
        let history = history.clone();      // copy history state
        let input_cmd = input_cmd.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {

                let user_input = (*input_cmd).trim().to_string();
                if !user_input.is_empty() {
                    let mut new_history = (*history).clone();       // copy current history

                    new_history.push(html! { <div class="line"><span class="prompt">{"hitesh@portfolio:~$ "}</span>{ &user_input }</div> });


                    // parsing the cmd 
                    let parsed_cmd = Cmd::from_str(&user_input);

                    match parsed_cmd {
                        Some(Cmd::Help) => {
                            new_history.push(html! {
                                <div class="output">
                                    <p>{ "Commands: help | about | skills | projects | research | experience | clear" }</p>
                                </div>
                            });
                        }

                        Some(Cmd::AboutMe) => {
                            new_history.push(html! {
                                <div class="output">
                                    <p>{ "Hi, I'm Hitesh Bhatnagar it's pleasure to meet you !!" }</p>
                                    <p>{ "B.Tech ECE @ VIT Vellore | GPA: 8.37/10" }</p>
                                    <p>{ "Focused on AI/ML and Systems Programming" }</p>
                                    <p>{ "hbhatnagar917@gmail.com | +91-9267995320" }</p>
                                </div>
                            });
                        }

                        Some(Cmd::Skills) => {
                            new_history.push(html! {
                                <div class="output">
                                    <p>{ "Languages   : Rust, C, C++, Python, Java, Verilog, MATLAB, SQL, Bash" }</p>
                                    <p>{ "AI / ML     : PyTorch, TensorFlow, Keras, OpenCV, Scikit-learn" }</p>
                                    <p>{ "Tools       : Git, Docker, Linux, VS Code, Vivado, ModelSim" }</p>
                                    <p>{ "Concepts    : RTOS, DSP, DBMS, OOP, Data Structures, Cryptography" }</p>
                                </div>
                            });
                        }

                        Some(Cmd::Projects) => {
                            for i in Project::get_all() {       // calling Type::func() {Type == Struct}
                                
                                new_history.push(html! {
                                    <div class="output">
                                        <p><b>{i.title}</b></p>
                                        <p>{ &i.description }</p>
                                        <p>{ format!("Stack: {}", &i.stack)}</p>
                                        <p>{ "Link: " }<a href={i.link.clone()} target="_blank">{i.link}</a></p>
                                    </div>
                                });
                            }
                        }

                        Some(Cmd::Research) => {
                            for i in Research::get_all() {
                                new_history.push(html! {
                                    <div class="output">
                                        <p><b>{i.title}</b></p>
                                        <p><i>{i.publisher}</i></p>
                                        <p>{ format!("Tags: {}", i.tag) }</p>
                                        <p>{ "Link: " }<a href={i.link.clone()} target="_blank">{ "View Paper" }</a></p>
                                    </div>   
                                });   
                            }
                        }

                        Some(Cmd::Experience) => {
                            for exp in Experience::get_all() {  // Same pattern!
                                new_history.push(html! {
                                    <div class="output">
                                        <p style="color: #38bdf8;"><b>{ exp.role }</b>{ " @ " }{ exp.company }</p>
                                        <p style="font-size: 0.9em; color: #64748b;">{ exp.duration }</p>
                                        <ul style="margin-top: 5px; margin-bottom: 15px; padding-left: 20px;">
                                            {
                                                exp.tasks.iter().map(|task| {
                                                    html! { <li style="margin-bottom: 4px;">{ task }</li> }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                    </div>
                                });
                            }
                        }
                        
                        Some(Cmd::Exit) => {
                            new_history.push(html! { <div class="output">{"Closing session... "}</div> });
                        }

                        None => {
                            if user_input == "clear" {
                                new_history.clear();
                                new_history.push(html! { <div class="welcome">{"Terminal cleared. Type 'help' to begin."}</div> });
                            } else {
                                new_history.push(html! { <div class="error">{ format!("Command not found: {}", user_input) }</div> });
                            }
                        }

                    }

                    history.set(new_history);       // save memory
                    input_cmd.set(String::new());       // clear input space
                }
            }
        })
    };


html! {
        <div class="terminal-container">
            <div id="terminal">
                {
                    history.iter().map(|line| {
                        // Notice the .clone() here! We must clone Html in Yew.
                        html! { <div class="history-entry"> { line.clone() } </div> }
                    }).collect::<Html>()
                }
                
                // THE INPUT BOX
                <div class="input-line">
                    <span class="prompt">{ "hitesh@portfolio:~$ " }</span>
                    <input 
                        type="text" 
                        value={(*input_cmd).clone()} 
                        oninput={on_input} 
                        onkeydown={on_enter} 
                        autofocus=true 
                    />
                </div>
            </div>
        </div>

    
}

}

fn main() {
    yew::Renderer::<App>::new().render();
}