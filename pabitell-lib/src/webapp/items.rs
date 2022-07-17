use std::{cmp, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefaultAction {
    // The one which will be triggered once big picture is clicked
    Give,
    Use,
    Scan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub code: String,
    pub short: String,
    pub long: String,
    pub image_url: String,
    pub give_data: Option<Rc<Vec<u8>>>,
    pub use_data: Option<Rc<Vec<u8>>>,
    pub scan: bool,
    pub default: DefaultAction,
    pub last_event: Option<usize>,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // Sort in desc order by last_event
        // and then by name in asc
        let self_value = self.last_event.unwrap_or(0);
        let other_value = other.last_event.unwrap_or(0);
        if self_value > other_value {
            Some(cmp::Ordering::Less)
        } else if self_value < other_value {
            Some(cmp::Ordering::Greater)
        } else if self.short < other.short {
            Some(cmp::Ordering::Less)
        } else if self.short > other.short {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Equal)
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Sort in desc order by last_event
        // and then by name in asc
        let self_value = self.last_event.unwrap_or(0);
        let other_value = other.last_event.unwrap_or(0);
        if self_value > other_value {
            cmp::Ordering::Less
        } else if self_value < other_value {
            cmp::Ordering::Greater
        } else if self.short < other.short {
            cmp::Ordering::Less
        } else if self.short > other.short {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
}
