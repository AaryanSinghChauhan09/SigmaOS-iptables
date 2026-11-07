//! Chain model.
use crate::rule::IptablesRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainHook { Input, Output, Forward, Prerouting, Postrouting }

#[derive(Debug, Clone)]
pub struct IptablesChain {
    pub name: String,
    pub hook: Option<ChainHook>,
    pub rules: Vec<IptablesRule>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain() {
        let c = IptablesChain { name: "INPUT".into(), hook: Some(ChainHook::Input), rules: vec![] };
        assert_eq!(c.name, "INPUT");
    }
}
