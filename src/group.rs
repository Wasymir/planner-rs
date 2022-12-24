use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::sync::MutexGuard;
use std::hash::{Hash, Hasher};

use derive_new::new;

use petgraph::{graphmap::GraphMap,Undirected};

use crate::{student::*, subject::*};

#[derive(Debug, Eq, new, Clone)]
pub struct Group {
    pub id: usize,
    pub subjects: HashSet<Subject>,
    pub students: HashSet<usize>,
    pub perfect: bool,
}

// A compare function which takes only subjects into account.
// This way we can avoid duplicates in a HashSet of groups.
impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.subjects.eq(&other.subjects)
    }
}

// A hash function whose evaluation is solely based on subjects.
impl Hash for Group {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the subjects set into a vector and sort the elements.
        let mut sorted_subjects: Vec<_> = Vec::from_iter(self.subjects.clone());
        sorted_subjects.sort();

        // Hash the elements of the sorted subjects set individually.
        for subject in sorted_subjects {
            subject.hash(state);
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {{ id: {}, subjects: {:?}, students: {:?}, perfect: {} }}", self.id, self.subjects, self.students, self.perfect)
    }
}

fn recursive_construction(graph: &mut GraphMap<Subject,usize,Undirected>, current: &mut HashSet<Subject>, subjects: &MutexGuard<HashMap<Subject,HashSet<usize>>>,
        students_num: usize, groups: &mut HashSet<Group>) {
    let mut c: bool = false;
    for subject in subjects.keys() {
        // We check if the subject is connected to every single other one in the current HashSet in the graph representation of subjects
        let mut b: bool = false;
        for other in current.iter() {
            if !graph.contains_edge(*subject, *other) {
                b = true;
                break;
            }
        }

        // If that's the case, we copy the HashSet and pass its contents to the next recursive call
        if !b {
            let mut cp: HashSet<Subject> = HashSet::new();
            for item in current.iter() {
                cp.insert(item.clone());
            }
            cp.insert(*subject);
            recursive_construction(graph,&mut cp, subjects, students_num, groups);
            c = true;
        }
    }

    /*
     If no recursive function was called, we know that this subject group was "maximal",
     meaning that we aren't able to expand it any further and we insert it into the Group HashSet
     to ensure that there are no duplicates
    */
    if !c {
        let mut students: HashSet<usize> = HashSet::new();
        current.iter().for_each(|subject| students.extend(subjects.get(subject).unwrap()));
        let mut cp: HashSet<Subject> = HashSet::new();
        for item in current.iter() {
            cp.insert(item.clone());
        }
        let len: usize = students.len();
        groups.insert(Group::new(groups.len(), cp, students, students_num == len));
    }
}

pub fn group() -> HashSet<Group> {
    let subjects = SUBJECTS.lock().unwrap();
    // Graph creation
    let mut graph: GraphMap<Subject,usize,Undirected> = GraphMap::new();
    subjects.keys().map(|subject| graph.add_node(*subject)).count();

    for subject in subjects.keys() {
        for other in subjects.keys().filter(|s| **s != *subject) {
            if subjects
                .get(subject)
                .unwrap()
                .intersection(subjects.get(other).unwrap())
                .next()
                .is_none()
            {
                graph.add_edge(*subject, *other, 1);
            }
        }
    }

    let mut groups: HashSet<Group> = HashSet::new();
    // Creating groups based on the graph representation of subjects
    recursive_construction(&mut graph, &mut HashSet::new(), &subjects, STUDENTS.lock().unwrap().len(), &mut groups);

    // Printing the result for testing and debugging purposes
    println!("{:?}", groups);

    groups
}
