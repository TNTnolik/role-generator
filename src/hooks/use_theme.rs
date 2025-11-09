use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use web_sys::window;

const THEME_KEY:&str = "app-theme";

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark  => "dark",
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

#[hook]
pub fn use_theme() -> (Theme, Callback<MouseEvent>) {
    let theme = use_state(|| {
        if let Ok(saved) = LocalStorage::get::<String>(THEME_KEY) {
            return if saved == "dark" { Theme::Dark } else { Theme::Light };
        }

        window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map(|m| if m.matches() { Theme::Dark } else { Theme::Light })
            .unwrap_or(Theme::Light)
    });

    {
        let theme = theme.clone();
        use_effect_with(theme, move |theme| {
            let document = gloo::utils::document();
            let html = document.document_element().unwrap();
            html.set_attribute("data-theme", theme.as_str()).unwrap();
            let _ = LocalStorage::set(THEME_KEY, theme.as_str());
            || ()
        });
    }

    let toggle = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let mut new_theme = *theme;
            new_theme.toggle();
            theme.set(new_theme);
        })
    };

    (*theme, toggle)
}