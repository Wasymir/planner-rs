use crate::student::*;
use group::group;
use group::GroupRelation;
use student::Student;
use subject::Subject::*;

#[macro_use]
extern crate lazy_static;

mod group;
mod student;
mod subject;

fn main() {
    println!("Reading student lists...");
    let students = vec![
        Student::new(
            0,
            None,
            vec![BiologySL, ChemistryHL]
        ),
        Student::new(
            1,
            None,
            vec![GermanBHL, PolishALiteratureHL]
        ),
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
        Student::new(
            4,
            None,
            vec![BiologyHL, ChemistryHL, MathematicsAASL]
        ),
        Student::new(
            5,
            None,
            vec![MathematicsAAHL, EnglishALanguageAndLiteratureSL, GermanBSL]
        ),
        Student::new(
            6,
            None,
            vec![MathematicsAAHL, PhysicsHL, SpanishBSL]
        ),
        Student::new(
            7,
            None,
            vec![MathematicsAASL, ComputerScienceHL, PhysicsSL]
        ),
    ];
    println!("Initialization...");
    init(&students);
    /*
     We create a relation graph which represents a situation where non of the subjects
     were removed from any of the groups. For each simulation the program will copy this
     relation and work its way from it.
    */
    let original_relation = GroupRelation::new(group());
}
