use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::hash::Hash;
use std::sync::MutexGuard;

use derive_new::new;
use lazy_static::__Deref;
use strum::IntoEnumIterator;

use crate::{student::*, subject::*,graphs::{graph::Graph}};

#[derive(Debug, new, PartialEq, Eq, Clone)]
pub struct Group {
    pub id: usize,
    pub subjects: HashSet<Subject>,
    pub students: HashSet<usize>,
    pub perfect: bool,
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {{ id: {}, subjects: {:?}, students: {:?}, perfect: {} }}", self.id, self.subjects, self.students, self.perfect)
    }
}

fn recursive_construction(graph: &mut Graph<Subject>, current: &mut HashSet<Subject>, subjects: &MutexGuard<HashMap<Subject,HashSet<usize>>>,
        students_num: usize, groups: &mut Vec<Group>) {
    let mut c: bool = false;
    for subject in subjects.keys() {
        //sprawdzamy czy przedmiot jest połączony ze wszystkimi, które są obecnie w current
        let mut b: bool = false;
        for other in current.iter() {
            if !graph.get_adjacent_vertices(*subject).contains(other) {
                b = true;
                break;
            }
        }

        //jeżeli jest połączony, kopiujemy zawartość current i przekazujemy
        //do kolejnego wywołania
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

    //jeżeli nie wywołaliśmy nic, wiadomo, że jest to grupa "maksymalna" i w związku z tym
    //dodajemy ją do listy grup
    if !c {
        let mut students: HashSet<usize> = HashSet::new();
        current.iter().for_each(|subject| students.extend(subjects.get(subject).unwrap()));
        let mut cp: HashSet<Subject> = HashSet::new();
        for item in current.iter() {
            cp.insert(item.clone());
        }
        let len: usize = students.len();
        groups.push(Group::new(groups.len(), cp, students, students_num == len));
    }
}

pub fn group() -> Vec<Group> {
    let subjects = SUBJECTS.lock().unwrap();
    //konstrukcja grafu
    let mut graph: Graph<Subject> = Graph::new();
    subjects.keys().map(|subject| graph.add_vertex(*subject)).count();

    for subject in subjects.keys() {
        for other in subjects.keys().filter(|s| **s != *subject) {
            if subjects
                .get(subject)
                .unwrap()
                .intersection(subjects.get(other).unwrap())
                .next()
                .is_none()
            {
                graph.add_edge(*subject, *other);
            }
        }
    }

    let mut groups: Vec<Group> = Vec::new();
    //tutaj będziemy tworzyć odpowiednie grupy na podstawie grafu
    //korzystając z rekurencyjnego podejścia
    recursive_construction(&mut graph, &mut HashSet::new(), &subjects, STUDENTS.lock().unwrap().len(), &mut groups);

    //na potrzeby testowania wypisujemy wszystkie grupy w konsoli
    println!("{:?}", groups);

    groups
}
