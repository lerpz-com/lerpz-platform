use leptos::{logging::debug_warn, prelude::*};
use leptos_meta::{Html, provide_meta_context};
use strum::EnumString;

#[derive(EnumString, Clone, Default)]
#[strum(serialize_all = "snake_case")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

const THEME_STORAGE_KEY: &str = "theme";

impl Theme {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use std::str::FromStr;
                use web_sys::window;

                let theme = window()
                    .and_then(|w| w.local_storage().ok())
                    .and_then(|storage| storage?.get_item(THEME_STORAGE_KEY).ok())
                    .flatten()
                    .unwrap_or_default()
                    .to_string();

                Theme::from_str(&theme).unwrap_or_default()
            } else if #[cfg(feature = "ssr")] {
                Theme::Light
            }
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

pub fn use_theme() -> (ReadSignal<Theme>, WriteSignal<Theme>) {
    match use_context::<(ReadSignal<Theme>, WriteSignal<Theme>)>() {
        None => {
            debug_warn!(
                "Theme context not found. Did you forget to add the `<Theme />` component?"
            );
            let theme_signal = signal(Theme::new());
            provide_context(theme_signal.clone());
            theme_signal
        }
        Some(theme) => theme,
    }
}

#[island]
pub fn Theme() -> impl IntoView {
    provide_meta_context();

    let (theme, set_theme) = signal(Theme::new());
    provide_context((theme, set_theme));

    let (theme_class, set_theme_class) = signal("light");

    Effect::new(move || {
        let theme = theme.get();
        set_theme_class.update(|class| {
            *class = match theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            }
        });
    });

    view! {
        <Html {..} data-theme=theme_class />
    }
}
