use crate::trigger::Trigger;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct TriggerSet(pub BTreeSet<Box<dyn Trigger>>);

impl TriggerSet {
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Box<dyn Trigger>> {
        self.0.iter()
    }
}

impl Debug for TriggerSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.0.iter()).finish()
    }
}

#[macro_export]
macro_rules! triggerSet {
    ( $( $x:expr ),* ) => ({
        use $crate::trigger::Trigger;
        use $crate::trigger::TriggerSet;
        {
            let mut temp_set = std::collections::BTreeSet::new();
            $(
                let boxed: std::boxed::Box<dyn Trigger + 'static> = std::boxed::Box::new($x);
                temp_set.insert(boxed);
            )*
            TriggerSet(temp_set)
        }
    });
}
