pub mod quizengine {
    extern crate serde_json;
    use serde_json::{Map, Value};
    use std::io;
    use std::fs::read;

    fn get_string_value(obj: &Map<String, Value>, key: &str) -> String {
        match obj.get(key) {
            Some(s) => String::from(s.as_str().unwrap()),
            None => String::from("No such key")
        }
    }

    pub struct Flag {
        pub user: String,
        pub password: String
    }    
    pub struct Question {
        pub question: String,
        pub answer: String,
        pub feedback: String
    }

    impl Question {
        pub fn new(question_json: &Map<String, Value> ) -> Question {
            let question    = get_string_value(question_json, "question");
            let answer  = get_string_value(question_json, "answer");
            let feedback= get_string_value(question_json, "feedback");
            Question { question, answer, feedback }
        }
    }

    pub struct Quiz {
        pub questions: Vec<Question>,
        pub flag: Flag
    }

    impl Quiz {
        pub fn new(json_file: &str) -> Result<Quiz, &str> {
            match read(json_file) {
                Ok(raw) => {
                    let quiz_data = 
                        serde_json::from_slice::<serde_json::Map<String, Value>>(&raw)
                        .unwrap();
                    let questions: Vec<Question> = quiz_data.get("questions")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|q|
                            Question::new(
                                q.as_object().unwrap()
                            )
                        )
                        .collect();
                        
                    let flag: &serde_json::Map<String, Value> = quiz_data.get("flag")
                        .unwrap()
                        .as_object()
                        .unwrap();

                    Ok(Quiz {
                        questions,
                        flag: Flag {
                            user: String::from(
                                flag.get("user")
                                .unwrap()
                                .as_str()
                                .unwrap()
                            ),
                            password: String::from(
                                flag.get("flag")
                                .unwrap()
                                .as_str()
                                .unwrap()
                            )
                        }
                    })
                   
                },
                Err(_) => { Err("No such file") }                
            }

        }

        pub fn run(&self) {
            let stdin = io::stdin();

            let mut correct = 0_usize;

            for q in &self.questions {
                println!("{}", q.question);
                let mut response = String::new();
                stdin.read_line(&mut response).unwrap();
                if response.trim_end().to_lowercase().contains(q.answer.to_lowercase().as_str()) {
                    println!("Nice job!");
                    correct += 1;
                } else {
                    println!("{}", q.feedback);
                }
            }

            if correct == self.questions.len() {
                println!("Congratulations! Here's a flag: {}:{}", self.flag.user, self.flag.password);
            }
        }
    }
    
}