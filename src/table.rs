//! Table model.
use crate::chain::IptablesChain;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType { Filter, Nat, Mangle, Raw, Security }

#[derive(Debug, Clone)]
pub struct Table {
    pub ttype: TableType,
    pub chains: Vec<IptablesChain>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table() {
        let t = Table { ttype: TableType::Filter, chains: vec![] };
        assert_eq!(t.ttype, TableType::Filter);
    }
}
