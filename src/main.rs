use group::group;
use student::Student;
use subject::Subject::*;

mod group;
mod student;
mod subject;

fn main() {
    let students = vec![
        Student::new(0, None, vec![Biology, Chemistry]),
        Student::new(1, None, vec![German, Polish]),
        Student::new(2, None, vec![Math, English]),
        Student::new(3, None, vec![Polish, English]),
        Student::new(4, None, vec![Math, Chemistry]),
    ];
    group(&students)
}
