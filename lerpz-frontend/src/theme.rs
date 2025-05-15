use leptos::prelude::*;
use leptos_meta::Html;
use strum::EnumString;

#[derive(EnumString, Clone, Default)]
#[strum(serialize_all = "snake_case")]
pub enum ThemeContext {
    #[default]
    Light,
    Dark,
}

impl ThemeContext {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use std::str::FromStr;
                use leptos::web_sys::window;

                let theme = window()
                    .and_then(|w| w.local_storage().ok())
                    .and_then(|storage| storage?.get_item("theme").ok())
                    .flatten()
                    .unwrap_or_else(|| "light".to_string());

                ThemeContext::from_str(&theme).unwrap_or_default()
            } else if #[cfg(feature = "ssr")] {
                ThemeContext::Light
            }
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            ThemeContext::Light => ThemeContext::Dark,
            ThemeContext::Dark => ThemeContext::Light,
        };
    }
}

pub fn provide_theme_context() {
    if use_context::<ThemeContext>().is_none() {
        provide_context(ThemeContext::new());
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap_or_else(|| {
        panic!("ThemeContext not found. Did you forget to call `provide_theme_context()`?")
    })
}

#[component]
pub fn Theme() -> impl IntoView {
    let theme = use_theme();
    let theme_class = match theme {
        ThemeContext::Light => "light",
        ThemeContext::Dark => "dark",
    };

    view! {
        <Html {..} data-theme=theme_class />
    }
}
