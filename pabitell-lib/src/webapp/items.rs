use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultAction {
    // The one which will be triggered once big picture is clicked
    Give,
    Use,
    Scan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub code: String,
    pub short: String,
    pub long: String,
    pub image_url: String,
    pub give_data: Option<Rc<Vec<u8>>>,
    pub use_data: Option<Rc<Vec<u8>>>,
    pub scan: bool,
    pub default: DefaultAction,
}
