use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::Write;
use crate::GradeChoices:: {AddGrade, StudentAverage, ClassHighest, ClassLowest};

enum GradeChoices<'a> {
    AddGrade(String, f32),
    StudentAverage(& 'a String),
    ClassHighest,
    ClassLowest
}

fn handle_choice(choice: GradeChoices, student_map: &mut HashMap<String, Vec<f32>>) -> Option<f32> {
    match choice {
        AddGrade(name, grade) => {
            match student_map.entry(name) {
                Vacant(new_entry) => {
                    let new_grades = new_entry.insert(Vec::new());
                    new_grades.push(grade);

                    None
                }

                Occupied(entry) => {
                    let grades = entry.into_mut();
                    grades.push(grade);

                    None
                }
            }
        },

        StudentAverage(name)=> {
            match student_map.get_mut(name) {
                Some(grades) => {
                    let average = calculate_average(grades);

                    Some(average)
                }

                None => None
            }
        },

        ClassHighest => {
            let mut highest_grade: f32 = 0.0;

            for (_name, grades) in student_map.into_iter() {
                let current_average = calculate_average(grades);

                if current_average > highest_grade {
                    highest_grade = current_average
                }
            }

            Some(highest_grade)
        },

        ClassLowest => {
            let mut first_entry = student_map.iter_mut().next();

            let mut lowest: f32 = calculate_average(first_entry.unwrap().1);

            for (_name, grade) in student_map.into_iter() {
                let current_average = calculate_average(grade);

                if current_average < lowest {
                    lowest = current_average;
                }
            }
            Some(lowest)
        }
    }
}

fn calculate_average(grades: &mut Vec<f32>) -> f32 {
    let mut average: f32 = 0.0;
    let mut sum: f32 = 0.0;
    let mut total:f32 = 0.0;
    for grade in grades {
        sum += *grade;
        total += 1.0;
    }

    average = sum / total;

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

        let mut option: i32;
        loop {
            choose_option.clear();
            io::stdin().read_line(&mut choose_option).expect("Failed to read line");

            option = match choose_option.trim().parse() {
                Ok(num) => num,

                Err(e) => {
                    println!("Not a valid number try again: {e}");
                    continue;
                }
            };

            if option >= 1 && option <= 4 {
                break;
            } else if option == 5 {
                println!("Quitting...");
                break 'options;
            } else {
                println!("Please select a valid option");
            }
        }

        println!();

        match option {
            1 => {
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

                handle_choice(AddGrade(student_name, grade), &mut student_map);
                println!("Added graded");
            },

            2 => {
                let mut student_name = String::new();
                println!("Please enter name: ");

                io::stdin().read_line(&mut student_name).expect("Failed to read line");

                student_name = String::from(student_name.trim());

                let average = handle_choice(StudentAverage(&student_name), &mut student_map);

                match average {
                    Some(grade) => {
                        println!("The average grade for: {student_name} is: {grade}");
                    }

                    None => {
                        println!("Couldn't find student");
                    }
                }

            }
            _ => {}
        }


    }

    for (name, grades) in student_map {
        println!("Name: {name}");
        print!("Grades: ");
        for grade in grades {
            print!("{grade}, ");
        }
        println!();
        println!();
    }
}