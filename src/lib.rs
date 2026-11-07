//! SigmaOS-iptables — iptables/netfilter model.
//!
//! Models iptables chains, tables (filter/nat/mangle/raw),
//! rule matching criteria, and targets. Deterministic and std-only.

pub mod table;
pub mod chain;
pub mod target;
pub mod criteria;
pub mod rule;

pub use table::{Table, TableType};
pub use chain::{IptablesChain, ChainHook};
pub use target::{Target, TargetAction};
pub use criteria::{MatchCriteria, MatchType};
pub use rule::{IptablesRule, RuleMatch};
