#[allow(unused_variables)]
use std::collections::HashMap;

pub struct School {
    grade_student: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grade_student: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_student
            .entry(grade)
            .or_insert(Vec::new())
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grade_list = self.grade_student.keys().cloned().collect::<Vec<u32>>();
        grade_list.sort();
        grade_list
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(students) = self.grade_student.get(&grade) {
            let mut students = students.clone();
            students.sort();
            return Some(students)
        }
        return None
    }
}
