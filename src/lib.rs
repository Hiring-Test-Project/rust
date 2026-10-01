#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obligation {
    pub id: u64,
    pub amount_due: u64,
    pub priority: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub id: u64,
    pub applied: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    pub lines: Vec<Line>,
    pub unapplied: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DuplicateId,
}

pub fn settle(_payment: u64, _obligations: &[Obligation]) -> Result<Settlement, Error> {
    unimplemented!("settle")
}
