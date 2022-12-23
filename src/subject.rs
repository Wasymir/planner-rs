use crate::student::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use strum::{EnumIter, EnumString, IntoEnumIterator, ToString};

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
        let students = STUDENTS.lock().unwrap();
        let mut subjects = HashMap::new();
        for subject in Subject::iter() {
            let set = HashSet::from_iter(
                students.values()
                    .filter(|s| s.subjects.contains(&subject))
                    .map(|s| s.id),
            );

            // if there are no students that have chosen this subject, it's not added to list
            if !set.is_empty() {
                subjects.insert(subject,set);
            }
        }

        subjects.into()
    };
}
