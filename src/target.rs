//! Target model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetAction { Accept, Drop, Reject, Log, Return, Jump(String) }

#[derive(Debug, Clone)]
pub struct Target {
    pub action: TargetAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target() {
        let t = Target { action: TargetAction::Accept };
        assert_eq!(t.action, TargetAction::Accept);
    }
}
