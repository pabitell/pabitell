// translations related stuff
use fluent_bundle::{bundle::FluentBundle, FluentArgs, FluentResource};
use include_dir::{include_dir, Dir, DirEntry};
use intl_memoizer::concurrent::IntlLangMemoizer;
use lazy_static::lazy_static;
use pabitell_lib::translations::{self, get_available_locales, get_bundle};
use std::collections::HashMap;

pub static RESOURCES: Dir = include_dir!("resources/");

lazy_static! {
    static ref BUNDLES: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>> = {
        let mut res = HashMap::new();
        for lang in get_available_locales(&RESOURCES).expect("failed to list translations") {
            match get_bundle(&RESOURCES, lang.clone(), "doggie_and_kitie-cake") {
                Err(err) => panic!("failed to load translations: {}", err),
                Ok(bundle) => {
                    res.insert(lang.to_string(), bundle);
                }
            }
        }
        res
    };
}

pub fn get_message(msgid: &str, langid: &str, args: Option<FluentArgs>) -> String {
    if let Some(bundle) = BUNDLES.get(langid) {
        translations::get_message(bundle, msgid, args).unwrap_or(msgid.to_string())
    } else {
        msgid.to_string()
    }
}
