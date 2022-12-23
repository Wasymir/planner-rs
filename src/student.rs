use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use derive_new::new;

use crate::subject::Subject;

#[derive(Debug, new, PartialEq, Eq, Clone)]
pub struct Student {
    pub id: usize,
    pub name: Option<String>,
    pub subjects: Vec<Subject>,
}

//Wydajniejsza funkcja hashująca, która bazuje jedynie na id ucznia
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

pub fn init_students(students: &Vec<Student>) {
    let mut students_static = STUDENTS.lock().unwrap();
    students.iter().map(|student| students_static.insert(student.id, student.clone())).count();
}
