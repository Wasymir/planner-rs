use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::sync::MutexGuard;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

use derive_new::new;

use petgraph::Direction::Incoming;
use petgraph::{Undirected,Directed};
use petgraph::{graphmap::GraphMap,graph::{Graph,NodeIndex,EdgeIndex},visit::EdgeRef};

use crate::{student::*, subject::*};

#[derive(Debug, Eq, new)]
pub struct Group {
    pub id: usize,
    pub subjects: HashSet<Subject>,
    pub students: HashSet<usize>,
    pub perfect: bool,
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
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

pub fn group() -> Vec<Group> {
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

    let mut group_vec: Vec<Group> = Vec::from_iter(groups);
    group_vec.sort();

    group_vec
}

/*
 This struct represents the relation between groups. In particular, it can be
 used to derive whether the students of a specific group also belong to another one.
 Putting groups from a path of the graph representing such occurances onto the timetable
 ensures that every single student who comes to school doesn't waste their time while others
 have lessons. On top of that placing groups in reverse order from a path connected to the
 one which we have just put also guarantees that to be the case.
*/

#[derive(Debug)]
pub struct GroupRelation {
    pub graph: Graph<usize,usize,Directed,u32>,
    pub groups: Vec<Group>,
}

impl Display for GroupRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GroupRelation {{ groups: {:?}, graph: {:?} }}", self.groups, self.graph)
    }
}

impl GroupRelation {
    pub fn new(groups: Vec<Group>) -> GroupRelation {
        let mut graph: Graph<usize,usize,Directed,u32> = Graph::new();
        for i in 0..groups.len() {
            graph.add_node(0);
        }
        
        for group in &groups {
            for other in &groups {
                if group != other {
                    let mut connect: bool = true;
                    for student in &group.students {
                        if !other.students.contains(student) {
                            connect = false;
                            break;
                        }
                    }

                    if connect {
                        let ind1 = NodeIndex::new(group.id);
                        let ind2 = NodeIndex::new(other.id);
                        graph.add_edge(ind1, ind2, other.students.len() - group.students.len());
                    }
                }
            }
        }

        GroupRelation { graph: graph, groups: groups }
    }

    pub fn delete_subject(&mut self, subject: Subject) {
        let subjects = SUBJECTS.lock().unwrap();
        let groups = &mut self.groups;
        for i in 0..groups.len() {
            if groups[i].subjects.contains(&subject) {
                // This group contains the deleted subject so we remove it
                groups[i].subjects.remove(&subject);

                // Next we are going to clear the list of students and reevaluate it
                groups[i].students.clear();
                let group_subjects: HashSet<Subject> = groups[i].subjects.clone();
                for other in group_subjects.iter() {
                    groups[i].students.extend(subjects.get(other).unwrap());
                }

                let graph = &mut self.graph;

                /*
                 Reevaluating all edges in the graph pointing towards the modified node.
                 As the number of students can only decrease, no new ones of this kind will be added.
                */
                let mut to_remove: Vec<EdgeIndex> = Vec::new();
                for other_id in graph.edges_directed(NodeIndex::new(groups[i].id), Incoming) {
                    let other_ind = other_id.source().index();
                    // We want to check if all students of the 'other' group are still apart of the 'group' group
                    for student in groups[other_ind].students.iter() {
                        if !groups[i].students.contains(student) {
                            // We found a student who is not apart of the group 'group'. Removing the edge
                            to_remove.push(other_id.id());
                        }
                    }
                }

                /*
                 Since deleting during an iteration is not safe, the program does it once the iteration
                 is over.
                */
                for index in to_remove {
                    graph.remove_edge(index);
                }

                /*
                 Since the number of students has decreased, we might be able to create new connections
                 if one is not established. In order to do that, we iterate through all other groups
                 which are not connected and perform a reevaluation.
                */
                for j in 0..groups.len() {
                    if i != j {
                        if graph.find_edge(NodeIndex::new(groups[i].id), NodeIndex::new(groups[j].id)) == None {
                            // Since we are here, it means that there is no edge connecting those 2 groups
                            let mut all_apart: bool = true;
                            for student in &groups[i].students {
                                if !groups[j].students.contains(&student) {
                                    all_apart = false;
                                    break;
                                }
                            }

                            if all_apart {
                                // Creating a new edge
                                graph.add_edge(NodeIndex::new(groups[i].id), NodeIndex::new(groups[j].id), groups[j].students.len() - groups[i].students.len());
                            }
                        }
                    }
                }
            }
        }
    }
}
