use crate::gen::conditions::{Condition, Spend};
use clvmr::allocator::Allocator;

// These are customization points for the condition parsing and validation. The
// mempool wants to record additional information than plain consensus
// validation, so it hooks into these.
pub trait ConditionPolicy {
    fn new_spend(&mut self, spend: &mut Spend);
    fn condition(&mut self, spend: &mut Spend, c: &Condition);
    fn post_spend(&mut self, a: &Allocator, spend: &mut Spend);
}
