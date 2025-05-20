use leptos::{logging::debug_warn, prelude::*};
use leptos_meta::{Html, provide_meta_context};
use strum::{Display, EnumString};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, window};

const THEME_STORAGE_KEY: &str = "Lerpz-Theme";

#[derive(Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

#[derive(Clone)]
pub struct Theme(pub ReadSignal<ThemeKind>, pub WriteSignal<ThemeKind>);

impl ThemeKind {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use std::str::FromStr;

                let cookie_str = window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
                    .and_then(|d| d.cookie().ok());

                match cookie_str {
                    Some(cookie) => {
                        cookie
                            .split(';')
                            .find(|c| c.trim().starts_with(THEME_STORAGE_KEY))
                            .and_then(|c| c.split('=').nth(1))
                            .and_then(|c| ThemeKind::from_str(c).ok())
                            .unwrap_or_default()
                    },
                    None => ThemeKind::default(),
                }
            } else if #[cfg(feature = "ssr")] {
                ThemeKind::default()
            }
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };
    }
}

impl Theme {
    pub fn toggle(&self) {
        self.1.update(|theme| theme.toggle());
    }
}

pub fn use_theme() -> Theme {
    match use_context::<Theme>() {
        None => {
            debug_warn!(
                "ThemeContext was not found. Did you forget to add the
                `<ThemeProvider />` component?"
            );
            let (theme, set_theme) = signal(ThemeKind::new());
            let theme_ctx = Theme(theme, set_theme);
            provide_context(theme_ctx.clone());
            theme_ctx
        }
        Some(theme) => theme,
    }
}

#[island]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    provide_meta_context();

    let (theme, set_theme) = signal(ThemeKind::new());
    let theme_ctx = Theme(theme, set_theme);
    provide_context(theme_ctx.clone());

    Effect::new(move || {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
            .and_then(|d| d.set_cookie(&theme.get().to_string()).ok());
    });

    view! {
        <Html {..} data-theme={move || theme.get().to_string()} />
        {children()}
    }
}
