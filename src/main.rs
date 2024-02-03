use std::{error::Error, process, fs};
use askama::Template;

#[derive(Debug)]
struct QuestionResponse {
    question: String,
    responses: Vec<Response>
}

#[derive(Debug)]
struct Response {
    user: String,
    score: String, 
    comment: String,
    color: String
}

#[derive(Template)]
#[template(path = "form_responses.html")]
struct FormResponsesTemplate {
    question_responses: Vec<QuestionResponse>
}

fn score_to_color(score: &str) -> String {
    match score {
        "Strong Disagree" => "#D32F2F".to_string(), // Red
        "Disagree" => "#E57373".to_string(),
        "Neutral" => "#FFB300".to_string(), // Yellow/Middle
        "Agree" => "#81C784".to_string(),
        "Strong Agree" => "#388E3C".to_string(), // Green
        "Strong Yes" => "#388E3C".to_string(), // Green
        "Yes" => "#81C784".to_string(),
        "No" => "#E57373".to_string(),
        "Strong No" => "#D32F2F".to_string(), // Red
        _ => "#ffffff".to_string(), // Fallback color
    }
}

fn extract_and_capitalize_first_name(email: &str) -> String {
    // Split the email address at the '@' symbol and take the first part.
    let first_part = email.split('@').next().unwrap_or("");

    // Further split by '.' in case the email is in 'first.last@' format.
    let first_name = first_part.split('.').next().unwrap_or("");

    // Capitalize the first letter of the first name and return it.
    first_name
        .char_indices()
        .fold(String::new(), |mut acc, (i, c)| {
            if i == 0 {
                acc.push(c.to_ascii_uppercase());
            } else {
                acc.push(c);
            }
            acc
        })
}


fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("./data/data.csv")?;

    let headers = rdr.headers()?;

    // init vector of question responses
    let mut question_responses: Vec<QuestionResponse> = Vec::new();

    // for each question, populate a new QR with question
    for i in 0..headers.len() {
        if i < 2 { continue; }  // skip timestamp/email
          
        // don't add comment questions
        if i % 2 == 0  { 
            question_responses.push(QuestionResponse{ 
                question: headers[i].to_string(), 
                responses: Vec::new()
            });
        }
    }

    // each row here is a full response from one person
    for response in rdr.records() {

        let valid_response = response?;

        // look at each individual response 
        let mut user = String::new();
        let mut score = String::new();
        let mut comment = String::new();
        for i in 0..valid_response.len() {
            let valid_response_string = valid_response[i].to_string(); 
             // skip timestamp
            if i == 0 { 
                continue; 
            } else if i == 1 {
                user = valid_response_string;
            } else if i % 2 == 0 { // question response
                score = valid_response_string;
            } else { // comment
                comment = valid_response_string; 

                let idx = (i - 3) / 2; // conversion from column to question num
                question_responses[idx].responses.push(Response {
                    user: extract_and_capitalize_first_name(&user),
                    score: score.clone(),
                    comment: comment.clone(),
                    color: score_to_color(&score)
                });
            }
        }
    }

    let template = FormResponsesTemplate { question_responses };
    let rendered = template.render()?;
    fs::write("output.html", &rendered)?;
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
 

