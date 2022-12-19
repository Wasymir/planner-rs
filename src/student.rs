use std::collections::HashSet;

use derive_new::new;

use crate::subject::Subject;

#[derive(Debug, Hash, new, PartialEq, Eq)]
pub struct Student {
    pub id: usize,
    pub name: Option<String>,
    pub subjects: Vec<Subject>,
}
