use std::{error::Error, process};
use askama::Template;

struct QuestionResponse {
    question: String,
    responses: Vec<Response>
}

struct Response {
    person: String,
    score: String, 
    comment: String
}

#[derive(Template)]
#[template(path = "form_responses.html")]
struct FormResponsesTemplate {
    question_responses: Vec<QuestionResponse>
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("./data/data.csv")?;

    let headers = rdr.headers()?;

    // init vector of question responses
    let mut question_responses: Vec<QuestionResponse> = Vec::new();

    // for each question, populate a new QR with question
    for i in 0..headers.len() {
        question_responses.push(QuestionResponse{ 
            question: headers[i].to_string(), 
            responses: Vec::new()
        });
        println!("New question added :: {}", headers[i].to_string());
    }
    for result in rdr.records() {
        let qr = result?;
        println!("{:?}", qr);
    }

    let template = FormResponsesTemplate { question_responses };
    let rendered = template.render()?;
    println!("{}", rendered);
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
 

