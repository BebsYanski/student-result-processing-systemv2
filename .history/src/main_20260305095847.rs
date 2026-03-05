use std::io;

fn main() {
    // Welcoming Salutation
    println!("\n\n**********Student Result Processing System**********\n");

    // Inputs
    let mut student_name: String = String::new();
    let mut student_age: String = String::new();
    let mut student_subject_count: String = String::new();

    // Receive inputs from user
    println!("Please input your name:");
    io::stdin()
        .read_line(&mut student_name)
        .expect("Could not read input");

    println!("Please input your age:");
    io::stdin()
        .read_line(&mut student_age)
        .expect("Could not read input");

    println!("Please input your number of subjects:");
    io::stdin()
        .read_line(&mut student_subject_count)
        .expect("Could not read input");

    // Parse and convert inputs
    let student_age: u8 = student_age.trim().parse().unwrap();
    let student_subject_count: u8 = student_subject_count.trim().parse().unwrap();

    // Validate inputs
    if student_age <= 0 {
        println!(
            "Your Student age is {}, but cannot be less than 1 ",
            student_age
        );
        return;
    }
    if student_subject_count <= 0 {
        println!(
            "Your Student subject count is {}, but cannot be less than 1 ",
            student_subject_count
        );
        return;
    }

    // Receive scores, and calculate total score
    let mut total_score = 0;

    for i in 1..=student_subject_count {
        let mut score: String = String::new();
        println!("Input score number {i}");
        io::stdin().read_line(&mut score).unwrap();

        // let score_int = score.trim().parse::<i32>().expect("Invalid number");
        total_score = calculate_total(
            total_score,
            score.trim().parse::<i32>().expect("Invalid number"),
        );
    }
    println!("Total score: {}", total_score);

    // Calculate the student's average
    let student_avg = calculate_average(total_score, student_subject_count as i32);

    let student_grade = determine_grade(student_avg);

    display_report(
        &student_name,
        student_age,
        student_subject_count,
        total_score,
        student_avg,
        &student_grade,
    )
}

fn calculate_total(total: i32, score: i32) -> i32 {
    total + score
}
fn calculate_average(total: i32, subjects: i32) -> f32 {
    total as f32 / subjects as f32
}

fn determine_grade(average: f32) -> String {
    if average >= 80.0 {
        "Excellent".to_string()
    } else if average >= 70.0 {
        "Very Good".to_string()
    } else if average >= 60.0 {
        "Good".to_string()
    } else if average >= 50.0 {
        "Pass".to_string()
    } else {
        "Fail".to_string()
    }
}

fn display_report(name: &str, age: u8, subjects: u8, total_score: i32, average: f32, grade: &str) {
    println!("\n\n----------Student Report----------\n");
    println!(
        "
        Name: {}
        Age: {}
        Subjects: {}
        Total Score: {}
        Average: {:.2}
        Grade: {}
    ",
        name.trim(),
        age,
        subjects,
        total_score,
        average,
        grade
    );
    println!("\n---------------------------------------\n")
}
