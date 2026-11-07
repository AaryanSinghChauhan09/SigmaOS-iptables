//! Rule model.
use crate::target::Target;
use crate::target::TargetAction;
use crate::criteria::{MatchCriteria, MatchType};

#[derive(Debug, Clone)]
pub struct RuleMatch {
    pub criteria: Vec<MatchCriteria>,
}

#[derive(Debug, Clone)]
pub struct IptablesRule {
    pub match_expr: RuleMatch,
    pub target: Target,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iptables_rule() {
        let r = IptablesRule { match_expr: RuleMatch { criteria: vec![] }, target: Target { action: TargetAction::Accept } };
        assert_eq!(r.target.action, TargetAction::Accept);
    }
}
