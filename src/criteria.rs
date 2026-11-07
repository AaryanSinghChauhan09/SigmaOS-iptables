//! Match criteria model.
use crate::target::Target;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType { Source, Dest, Protocol, Port, Interface }

#[derive(Debug, Clone)]
pub struct MatchCriteria {
    pub mtype: MatchType,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_criteria() {
        let m = MatchCriteria { mtype: MatchType::Protocol, value: "tcp".into() };
        assert_eq!(m.mtype, MatchType::Protocol);
    }
}
