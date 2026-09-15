use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::Write;

struct Entry {
    name: String,
    grades: Vec<f32>,
}

fn calculate_average_grade(entry: &Entry) -> f32 {
    let mut average: f32 = 0.0;
    let mut sum: f32 = 0.0;
    let mut total_grades: f32 = 0.0;
    for grade in &entry.grades {
        sum += grade;
        total_grades += 1.0;
    }

    average = sum / total_grades;

    average
}

fn main() {

    let mut still_grading = true;
    let mut student_map: HashMap<String, Vec<f32>> = HashMap::new();

    'options: while still_grading {
        let mut choose_option = String::new();
        println!("Please select and option below");
        println!("1: Add grade");
        println!("2: Get average student grade");
        println!("3: Get class highest grade");
        println!("4: Get class lowest grade");
        println!("5: Quit grading");

        loop {
            io::stdin().read_line(&mut choose_option).expect("Failed to read line");

            let option: u32 = match choose_option.trim().parse() {
                Ok()
            };
        }

        println!();

        let mut student_name = String::new();
        let mut student_grade = String::new();

        println!("Please enter name");
        io::stdin().read_line(&mut student_name).expect("Failed to read line");

        let mut grade:f32;
        loop {
            student_grade.clear();
            println!("Please enter grade");
            io::stdin().read_line(&mut student_grade).expect("Failed to read line");

            grade = match student_grade.trim().parse() {
                Ok(grade) => grade,
                Err(e) => {
                    println!("Not a valid number try again");
                    continue;
                }
            };

            if grade >= 0.0 {
                break;
            }
        }

        student_name = String::from(student_name.trim());
        println!();

        match student_map.entry(student_name) {
            Vacant(new_entry) => {
                let new_vector =  new_entry.insert(Vec::new());
                new_vector.push(grade);
            }

            Occupied(entry) => {
                let grades = entry.into_mut(); // get mutable reference to value of hash_map
                grades.push(grade);
            }
        }
    }

    for (name, grades) in student_map {
        println!("Name: {name}");
        print!("Grades: ");
        for grade in grades {
            print!("{grade}, ");
        }
        println!();
    }
}