use crate::error::Result;
use colored::Colorize;
use res_mgr::ResManager;
use serde::Deserialize;
use std::env;

mod config;
mod error;
mod res_mgr;

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

async fn get_data<T>(res_mgr: &ResManager, url: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = res_mgr.store();
    let response = client.get(url).send().await?;
    let data = response.json::<T>().await?;
    Ok(data)
}
async fn get_user(res_mgr: &ResManager) -> Result<User> {
    let user_data = get_data::<User>(
        &res_mgr,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    )
    .await?;
    Ok(user_data)
}

async fn find_name(res_mgr: &ResManager, course_id: i32) -> Result<String> {
    let course_data = get_data::<CourseInfo>(
        &res_mgr,
        &format!("https://alamo.instructure.com/api/v1/courses/{}", course_id),
    )
    .await?;
    Ok(course_data.name)
}

async fn process_assignments(res_mgr: &ResManager, course_id: i32) -> Result<Vec<Assignment>> {
    let mut assignments = Vec::new();
    let mut page_number = 1;

    loop {
        let url = format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments?page={}",
            course_id, page_number
        );

        let mut page_assignments = get_data::<Vec<Assignment>>(&res_mgr, &url).await?;
        if page_assignments.is_empty() {
            break;
        }

        assignments.append(&mut page_assignments);
        page_number += 1;
    }

    Ok(assignments)
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let res_mgr = ResManager::new().await?;

    let user = get_user(&res_mgr).await?;
    let userid = user.id.to_string();

    println!(
        "Student ID: {}, Name: {}",
        userid.yellow().bold(),
        user.name.yellow().bold()
    );

    let mut courses = get_data::<Vec<Course>>(
        &res_mgr,
        &format!(
            "https://alamo.instructure.com/api/v1/users/{}/enrollments",
            userid
        ),
    )
    .await?;

    for course in courses.iter_mut() {
        course.course_name = Some(find_name(&res_mgr, course.id).await?);

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
            course
                .course_name
                .as_ref()
                .unwrap()
                .red()
                .bold()
                .underline(),
            &course.date[..10].red().bold(),
            letter_grade.red().bold(),
            course
                .grades
                .number_grade
                .unwrap_or_default()
                .to_string()
                .red()
                .bold()
        );

        let assignments = process_assignments(&res_mgr, course.id).await?;
        course.assignments = Some(assignments);

        if let Some(assignments) = &course.assignments {
            for assignment in assignments.iter() {
                print!("{} ", assignment.name.cyan().bold());

                if let Some(due) = &assignment.due {
                    println!("Due date: {}", &due[..10].blue());
                } else {
                    println!("Due date: {}", "No Due Date".blue().clear());
                }

                let submission = get_data::<Submission>(
                    &res_mgr,
                    &format!(
                        "https://alamo.instructure.com/api/v1/courses/{}/assignments/{}/submissions/{}",
                        course.id,
                        assignment.id,
                        userid
                    ),
                ).await?;
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
