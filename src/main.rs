use crate::student::*;
use crate::subject::*;
use group::group;
use student::Student;
use subject::Subject::*;

#[macro_use]
extern crate lazy_static;

mod graphs;
mod group;
mod student;
mod subject;

pub use graphs::graph::Graph;

fn main() {
    let students = vec![
        Student::new(0, None, vec![BiologySL, ChemistryHL]),
        Student::new(1, None, vec![GermanBHL, PolishALiteratureHL]),
        Student::new(
            2,
            None,
            vec![MathematicsAAHL, EnglishALanguageAndLiteratureSL],
        ),
        Student::new(
            3,
            None,
            vec![PolishALiteratureSL, EnglishALanguageAndLiteratureSL],
        ),
        Student::new(4, None, vec![MathematicsAASL, ChemistrySL]),
    ];
    init(&students);
    group();
}
