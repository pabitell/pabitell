use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LanguageSwitchProps {
    pub languages: Rc<Vec<String>>,
    pub lang: String,
    pub set_language_cb: Callback<String>,
}

#[function_component(LanguageSwitch)]
pub fn language_switch(props: &LanguageSwitchProps) -> Html {
    let cb = props.set_language_cb.clone();
    let render_lang = |e: String| {
        let cb_cloned = cb.clone();
        let cloned = e.clone();
        let onclick = Callback::from(move |_| cb_cloned.emit(cloned.clone()));
        html! {
            <a
                href="#"
                class="dropdown-item is-flex-shrink-1"
                {onclick}
            >{{ e }}</a>
        }
    };
    html! {
        <div class="dropdown is-hoverable">
            <div class="dropdown-trigger">
                <button
                    class="button"
                    aria-haspopup="true"
                    aria-controls="lang-dropdown"
                >
                    <span>{{ props.lang.clone() }}</span>
                </button>
            </div>
            <div class="dropdown-menu" role="menu" id="lang-dropdown">
                <div class="dropdown-content">
                    { for props.languages.iter().map(|e| render_lang(e.to_string())) }
                </div>
            </div>
        </div>
    }
}
