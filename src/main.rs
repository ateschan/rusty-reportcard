use serde::{Deserialize};
use serde_json;
use reqwest;
use colored::Colorize;
use std::env;
//This is a simple program that returns all avaliable courses

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "avatar_url")]
    pic: String,
}

#[derive(Debug, Deserialize)]
struct Course {
    #[serde(rename = "course_id")]
    id: i32,

    #[serde(rename = "created_at")]
    date: String,

    #[serde(rename = "grades")]
    grades: Grades,

    course_name: Option<String>,
    assignments: Option<Vec<Assignment>>,
}

#[derive(Debug, Deserialize)]
struct Grades {
    #[serde(rename = "current_grade")]
    letter_grade: Option<String>,
    #[serde(rename = "current_score")]
    number_grade: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct CourseInfo {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Assignment {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "points_possible")]
    pts: Option<f32>,
    #[serde(rename = "due_at")]
    due: Option<String>,

    grade: Option<f32>,

    submission: Option<Submission>,
}

#[derive(Debug, Deserialize)]
struct Submission {
    #[serde(rename = "assignment_id")]
    id: i32,
    #[serde(rename = "score")]
    pts: Option<f32>,
}

//Retrieves the json using the api and api token
fn get_data(token: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("Authorization", format!("Bearer {}", token)).send()?;
    Ok(response.text()?)
}

//Retrieves just the id assosiated with the student account
fn get_user(token: &str) -> User {
    let mut user = User {
        id: 0,
        name: "".to_string(),
        pic: "".to_string(),
    };
    match get_data(&token, "https://alamo.instructure.com/api/v1/users/self/profile") {
        Ok(body) => {
            user = serde_json::from_str(&body).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    user
}

fn find_name(token: &str, course_id: i32) -> String {
    let mut name = "".to_string();
    match get_data(&token, &format!("https://alamo.instructure.com/api/v1/courses/{}", course_id)) {
        Ok(body) => {
            let course: CourseInfo = serde_json::from_str(&body).unwrap();
            name = course.name;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    return name;
}

fn process_assignments(token: &str, course_id: i32) -> Vec<Assignment> {
    let mut assignmentlist: Vec<Assignment> = Vec::new();
    let mut page_number = 1;
    loop {
        let url = format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments?page={}",
            course_id,
            page_number
        );

        match get_data(&token, &url) {
            Ok(body) => {
                let mut assignments: Vec<Assignment> = serde_json::from_str(&body).unwrap();
                if assignments.is_empty() {
                    // No more assignments, so break out of the loop
                    break;
                }
                assignmentlist.append(&mut assignments);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        page_number += 1;
    }
    return assignmentlist;
}

fn main() -> Result<(), reqwest::Error> {
    env::set_var("RUST_BACKTRACE", "1");

    let apikey = env::var("API_KEY");
    match &apikey {
        Ok(apikey) => {
            println!("Curretn API key is: >{}<", apikey.hidden());

            // You can use the key in your API requests or other operations here
        }
        Err(e) => {
            println!(
                "Set an env API Key: \n 1. Go to https://alamo.instructure.com/profile/settings \n 2. Click on the API Access Tokens tab \n 3. Click New Access Token \n 4. Enter a name for the token and click Generate Token \n 5. Copy the token and create an environment variable named API_KEY with the token as the value 
            \n WINDOWS: open cmd and paste set API_KEY=your_api_key_here \n LINUX: open terminal and paste export API_KEY=\"your_api_key_here\""
            );
        }
    }
    let key = apikey.unwrap();

    let user = get_user(&key);
    let userid = user.id.to_string();

    println!("Student ID: {}, Name: {}", userid.yellow().bold(), user.name.yellow().bold());

    match
        get_data(
            &key,
            &format!("https://alamo.instructure.com/api/v1/users/{}/enrollments", userid)
        )
    {
        Ok(body) => {
            let mut courses: Vec<Course> = serde_json::from_str(&body).unwrap();

            //Retrieves course data
            for course in courses.iter_mut() {
                //All courses should have a name, id and date
                course.course_name = Some(find_name(&key, course.id));
                let letter_grade = {
                    match &course.grades.number_grade {
                        Some(grade) => {
                            if *grade >= 90.0 {
                                "A"
                            } else if *grade >= 80.0 {
                                "B"
                            } else if *grade >= 70.0 {
                                "C"
                            } else if *grade >= 60.0 {
                                "D"
                            } else {
                                "F"
                            }
                        }
                        None => "Not Graded",
                    }
                };

                println!();
                println!(
                    "ID: {}, Course: {}, Started: {}, Grade: {}, Overall Score: {}",
                    course.id.to_string().red().bold(),
                    course.course_name.as_ref().unwrap().red().bold().underline(),
                    &course.date[..10].red().bold(),
                    letter_grade.red().bold(),
                    course.grades.number_grade.unwrap_or_default().to_string().red().bold()
                );
                println!();

                //Retrieve assignment data for each course
                match
                    get_data(
                        &key,
                        &format!(
                            "https://alamo.instructure.com/api/v1/courses/{}/assignments",
                            course.id
                        )
                    )
                {
                    Ok(_) => {
                        let assignments: Vec<Assignment> = process_assignments(&key, course.id);
                        course.assignments = Some(assignments);
                        if let Some(assignments) = &mut course.assignments {
                            for assignment in assignments.iter_mut() {
                                //All assignments should have an id and name, but not all have a due date

                                //NAME
                                print!(
                                    "{} ",
                                    assignment.name.cyan().bold()
                                );
                                if let Some(due) = &assignment.due {
                                    println!("Due date: {}", &due[..10].blue());

                                } else {
                                    println!("Due date: {}", "No Due Date".blue().clear());

                                }

                                //Retrieve submission data for each assignment
                                match
                                    get_data(
                                        &key,
                                        &format!(
                                            "https://alamo.instructure.com/api/v1/courses/{}/assignments/{}/submissions/{}",
                                            course.id,
                                            assignment.id,
                                            userid
                                        )
                                    )
                                {
                                    Ok(body) => {
                                        let submission: Submission = serde_json
                                            ::from_str(&body)
                                            .unwrap();
                                        assignment.submission = Some(submission);
                                        //All submissions should have an id assosiated with an assignment, but not all have been graded
                                        if let Some(submission) = &assignment.submission {
                                            print!(
                                                "ID: {} ",
                                                submission.id.to_string().purple().magenta()
                                            );
                                            if let Some(pts) = &submission.pts {
                                                println!(
                                                    "Submission Points: {} / {}",
                                                    pts.to_string().green(),
                                                    assignment.pts
                                                        .unwrap_or_default()
                                                        .to_string()
                                                        .green()
                                                );
                                                println!();
                                            } else {
                                                println!(
                                                    "Submission Points: {} / {}",
                                                    "Not Graded".green(),
                                                    assignment.pts
                                                        .unwrap_or_default()
                                                        .to_string()
                                                        .green()
                                                );
                                                println!();
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("Error: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
