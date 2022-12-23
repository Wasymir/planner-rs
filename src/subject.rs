use strum::{EnumIter, EnumString, ToString, IntoEnumIterator};
use std::collections::{HashMap, HashSet};
use crate::student::*;
use std::sync::Mutex;

#[derive(EnumIter, PartialEq, Eq, Clone, Copy, Debug, Hash, EnumString, ToString)]
pub enum Subject {
    PolishALiteratureHL,
    PolishALiteratureSL,
    EnglishALanguageAndLiteratureHL,
    EnglishALanguageAndLiteratureSL,
    EnglishBHL,
    GermanBHL,
    GermanBSL,
    GermanAbInitioSL,
    FrenchBHL,
    FrenchBSL,
    FrenchAbInitioSL,
    SpanishAbInitioSL,
    SpanishBHL,
    SpanishBSL,
    EconomicsHL,
    EconomicsSL,
    GeographyHL,
    GeographySL,
    HistoryHL,
    HistorySL,
    BiologyHL,
    BiologySL,
    ChemistryHL,
    ChemistrySL,
    PhysicsHL,
    PhysicsSL,
    ComputerScienceHL,
    ComputerScienceSL,
    MathematicsAAHL,
    MathematicsAASL,
    MathematicsAISL,
    TOK,
}

impl Subject {
    pub fn frequency(self) -> i32 {
        match self.to_string().as_str() {
            "TOK" => 2,
            _ if self.to_string().ends_with("HL") => 6,
            _ if self.to_string().ends_with("SL") => 4,
            _ => panic!(),
        }
    }
}

lazy_static! {
    pub static ref SUBJECTS: Mutex<HashMap<Subject, HashSet<usize>>> = {
        let subjects = HashMap::new();
        subjects.into()
    };
}

pub fn init_subjects() {
    let students = STUDENTS.lock().unwrap();
    let mut subjects = SUBJECTS.lock().unwrap();
    for subject in Subject::iter() {
        let students_iter = students.values();
        let set = HashSet::from_iter(
            students_iter
                .filter(|s| s.subjects.contains(&subject))
                .map(|s| s.id),
        );
        
        //jeżeli nie ma uczniów, którzy wybrali ten przedmiot, wtedy go nie dodajemy na listę
        if !set.is_empty() {
            subjects.insert(subject,set);
        }
    }
}
