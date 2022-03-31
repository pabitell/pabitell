use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    pub name: String,
    pub title_slug: String,
    pub description_slug: String,
    pub img_url: String,
    pub target_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub name: String,
    pub title_slug: String,
    pub description_slug: String,
    pub img_url: String,
    pub chapters: Vec<Chapter>,
}

lazy_static! {
    pub static ref BOOKS: Vec<Book> = {
        vec![Book {
            name: "doggie_and_kitie".to_owned(),
            title_slug: "doggie_and_kitie-title".to_owned(),
            description_slug: "doggie_and_kitie-description".to_owned(),
            img_url: "images/doggie_and_kitie.svg".to_owned(),
            chapters: vec![
                Chapter {
                    name: "cake".to_owned(),
                    title_slug: "doggie_and_kitie-cake-title".to_owned(),
                    description_slug: "doggie_and_kitie-cake-description".to_owned(),
                    img_url: "images/doggie_and_kitie-cake.svg".to_owned(),
                    target_url: "/doggie_and_kitie/cake/".to_owned(),
                },
                Chapter {
                    name: "doll".to_owned(),
                    title_slug: "doggie_and_kitie-doll-title".to_owned(),
                    description_slug: "doggie_and_kitie-doll-description".to_owned(),
                    img_url: "images/doggie_and_kitie-doll.svg".to_owned(),
                    target_url: "/doggie_and_kitie/doll/".to_owned(),
                },
            ],
        }]
    };
}

lazy_static! {
    pub static ref LANGUAGES: Vec<&'static str> = vec!["en", "cs"];
}
