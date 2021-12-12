use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub code: String,
    pub short: String,
    pub long: String,
    pub image_url: String,
    pub data: Rc<Vec<u8>>, // to generate give item
}
