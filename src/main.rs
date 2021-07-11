use std::env::args;
mod quiz;
pub use crate::quiz::quizengine::Quiz;

fn main() {
    
    match args().nth(1) {
       Some(quiz_file) => { 
        if let Ok(quiz) = Quiz::new(quiz_file.as_str()) {
            quiz.run();
        }
       },
       None => { println!("I need a quiz file!") } 
    }
}