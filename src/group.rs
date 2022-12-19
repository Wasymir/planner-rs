use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;

use crate::{
    student::Student,
    subject::{self, Subject},
};

pub fn group(students: &Vec<Student>) {
    let mut subjects: HashMap<Subject, HashSet<usize>> = HashMap::new();
    for subject in Subject::iter() {
        subjects.insert(
            subject,
            HashSet::from_iter(
                students
                    .iter()
                    .filter(|s| s.subjects.contains(&subject))
                    .map(|s| s.id),
            ),
        );
    }

    let mut groups: HashMap<Subject, HashSet<Subject>> =
        HashMap::from_iter(Subject::iter().map(|s| (s, HashSet::new())));

    for subject in Subject::iter() {
        for other in Subject::iter().filter(|s| *s != subject) {
            if subjects
                .get(&subject)
                .unwrap()
                .intersection(subjects.get(&other).unwrap())
                .next()
                .is_none()
            {
                groups.get_mut(&subject).unwrap().insert(other);
            }
        }
    }

    dbg!(groups);
}
