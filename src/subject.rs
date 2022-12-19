use strum::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Subject {
    Biology,
    Chemistry,
    Math,
    German,
    English,
    Polish,
    ComputerScience,
    Geography,
    Economy,
    Tok,
}
