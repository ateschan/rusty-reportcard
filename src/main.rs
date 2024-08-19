use serde::{Deserialize};
use serde_json;
use reqwest;
use colored::Colorize;
use std::env;

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

fn get_data(token: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    Ok(response.text()?)
}

fn get_user(token: &str) -> Result<User, reqwest::Error> {
    let user_data = get_data(token, "https://alamo.instructure.com/api/v1/users/self/profile")?;
    let user: User = serde_json::from_str(&user_data).expect("Failed to parse user data");
    Ok(user)
}

fn find_name(token: &str, course_id: i32) -> Result<String, reqwest::Error> {
    let course_data = get_data(token, &format!("https://alamo.instructure.com/api/v1/courses/{}", course_id))?;
    let course: CourseInfo = serde_json::from_str(&course_data).expect("Failed to parse course data");
    Ok(course.name)
}

fn process_assignments(token: &str, course_id: i32) -> Result<Vec<Assignment>, reqwest::Error> {
    let mut assignments = Vec::new();
    let mut page_number = 1;

    loop {
        let url = format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments?page={}",
            course_id, page_number
        );

        let body = get_data(token, &url)?;
        let mut page_assignments: Vec<Assignment> = serde_json::from_str(&body).expect("Failed to parse assignments");

        if page_assignments.is_empty() {
            break;
        }

        assignments.append(&mut page_assignments);
        page_number += 1;
    }

    Ok(assignments)
}

fn main() -> Result<(), reqwest::Error> {
    env::set_var("RUST_BACKTRACE", "1");

    let apikey = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!(
                "Set an env API Key:
                1. Go to https://alamo.instructure.com/profile/settings
                2. Click on the API Access Tokens tab
                3. Click New Access Token
                4. Enter a name for the token and click Generate Token
                5. Copy the token and create an environment variable named API_KEY with the token as the value 
                \n WINDOWS: open cmd and paste set API_KEY=your_api_key_here 
                \n LINUX: open terminal and paste export API_KEY=\"your_api_key_here\""
            );
            return Ok(());
        }
    };

    let user = get_user(&apikey)?;
    let userid = user.id.to_string();

    println!(
        "Student ID: {}, Name: {}",
        userid.yellow().bold(),
        user.name.yellow().bold()
    );

    let course_data = get_data(
        &apikey,
        &format!("https://alamo.instructure.com/api/v1/users/{}/enrollments", userid),
    )?;
    
    let mut courses: Vec<Course> = serde_json::from_str(&course_data).expect("Failed to parse course data");

    for course in courses.iter_mut() {
        course.course_name = Some(find_name(&apikey, course.id)?);

        let letter_grade = match course.grades.number_grade {
            Some(grade) if grade >= 90.0 => "A",
            Some(grade) if grade >= 80.0 => "B",
            Some(grade) if grade >= 70.0 => "C",
            Some(grade) if grade >= 60.0 => "D",
            Some(_) => "F",
            None => "Not Graded",
        };

        println!(
            "ID: {}, Course: {}, Started: {}, Grade: {}, Overall Score: {}",
            course.id.to_string().red().bold(),
            course.course_name.as_ref().unwrap().red().bold().underline(),
            &course.date[..10].red().bold(),
            letter_grade.red().bold(),
            course.grades.number_grade.unwrap_or_default().to_string().red().bold()
        );

        let assignments = process_assignments(&apikey, course.id)?;
        course.assignments = Some(assignments);

        if let Some(assignments) = &course.assignments {
            for assignment in assignments.iter() {
                print!(
                    "{} ",
                    assignment.name.cyan().bold()
                );

                if let Some(due) = &assignment.due {
                    println!("Due date: {}", &due[..10].blue());
                } else {
                    println!("Due date: {}", "No Due Date".blue().clear());
                }

                let submission_data = get_data(
                    &apikey,
                    &format!(
                        "https://alamo.instructure.com/api/v1/courses/{}/assignments/{}/submissions/{}",
                        course.id,
                        assignment.id,
                        userid
                    ),
                )?;
                let submission: Submission = serde_json::from_str(&submission_data).expect("Failed to parse submission data");
                
                if let Some(submission_pts) = submission.pts {
                    println!(
                        "ID: {} Total: {} / {}",
                        submission.id.to_string().purple().magenta(),
                        submission_pts.to_string().green(),
                        assignment.pts.unwrap_or_default().to_string().green()
                    );
                } else {
                    println!(
                        "ID: {} Total: {} / {}",
                        submission.id.to_string().purple().magenta(),
                        "Not Graded".green(),
                        assignment.pts.unwrap_or_default().to_string().green()
                    );
                }
            }
        }
    }

    Ok(())
}
