use anyhow::{anyhow, Result};
use fluent_bundle::{bundle::FluentBundle, FluentArgs, FluentResource};
use include_dir::{Dir, DirEntry};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::str::{from_utf8, FromStr};
use unic_langid::LanguageIdentifier;

pub fn read_language_data(
    resoure_dir: &Dir,
    id: &LanguageIdentifier,
    translation_name: &str,
) -> Result<String> {
    let file = resoure_dir
        .get_file(format!("{}/{}.ftl", id.to_string(), translation_name))
        .ok_or_else(|| anyhow!("'{}' translation not found", translation_name))?;
    Ok(from_utf8(file.contents)?.to_string())
}

pub fn get_available_locales(resoure_dir: &Dir) -> Result<Vec<LanguageIdentifier>> {
    resoure_dir
        .find("*")
        .map_err(|err| anyhow!("{}", err))?
        .filter_map(|e| {
            if let DirEntry::Dir(dir) = e {
                Some(
                    LanguageIdentifier::from_str(dir.path().file_name()?.to_str()?)
                        .map_err(|err| anyhow!("{}", err)),
                )
            } else {
                None
            }
        })
        .collect()
}

pub fn get_bundle(
    resoure_dir: &Dir,
    lang: LanguageIdentifier,
    translation_name: &str,
) -> Result<FluentBundle<FluentResource, IntlLangMemoizer>> {
    let available = get_available_locales(resoure_dir)?;

    if !available.contains(&lang) {
        return Err(anyhow!("{} was not found in available languages", lang));
    }

    let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
    let data = read_language_data(resoure_dir, &lang, translation_name)?;
    let resource = FluentResource::try_new(data)
        .map_err(|_| anyhow!("Failed to parse flt file for language {}", lang))?;
    bundle.add_resource_overriding(resource);

    Ok(bundle)
}

pub fn get_message(
    bundle: &FluentBundle<FluentResource, IntlLangMemoizer>,
    msgid: &str,
    args: Option<FluentArgs>,
) -> Result<String> {
    let mut errors = vec![];
    let msg = bundle
        .get_message(msgid)
        .ok_or_else(|| anyhow!("Message `{}` was not found.", msgid))?;
    let pattern = msg
        .value()
        .ok_or_else(|| anyhow!("Message `{}` has no value.", msgid))?;
    Ok(bundle
        .format_pattern(pattern, args.as_ref(), &mut errors)
        .into())
}
