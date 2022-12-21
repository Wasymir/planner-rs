use strum::{EnumIter, EnumString, ToString};

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
