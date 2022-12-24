use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use crate::subject::Subject;
use derive_new::new;
use lazy_static::lazy_static;

#[derive(Debug, new, PartialEq, Eq, Clone)]
pub struct Student {
    pub id: usize,
    pub name: Option<String>,
    pub subjects: Vec<Subject>,
}

// Better hashing function that uses only id field
impl Hash for Student {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

lazy_static! {
    pub static ref STUDENTS: Mutex<HashMap<usize, Student>> = {
        let students = HashMap::new();
        students.into()
    };
}

pub fn init(students: &Vec<Student>) {
    let mut students_static = STUDENTS.lock().unwrap();
    students
        .iter()
        .map(|student| students_static.insert(student.id, student.clone()))
        .count();
}
