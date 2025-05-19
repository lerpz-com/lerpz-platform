use leptos::{logging::debug_warn, prelude::*};
use leptos_meta::{Html, provide_meta_context};
use strum::{Display, EnumString};
use web_sys::window;

const THEME_STORAGE_KEY: &str = "Lerpz-Theme";

#[derive(Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Clone)]
pub struct ThemeCtx(pub ReadSignal<Theme>, pub WriteSignal<Theme>);

impl Theme {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use std::str::FromStr;

                let theme = window()
                    .and_then(|w| w.local_storage().ok())
                    .and_then(|storage| storage?.get_item(THEME_STORAGE_KEY).ok())
                    .flatten()
                    .unwrap_or_default()
                    .to_string();

                Theme::from_str(&theme).unwrap_or_default()
            } else if #[cfg(feature = "ssr")] {
                Theme::Dark
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

impl ThemeCtx {
    pub fn toggle(&self) {
        self.1.update(|theme| theme.toggle());
    }
}

pub fn use_theme() -> ThemeCtx {
    match use_context::<ThemeCtx>() {
        None => {
            debug_warn!(
                "ThemeContext was not found. Did you forget to add the
                `<ThemeProvider />` component?"
            );
            let (theme, set_theme) = signal(Theme::new());
            let theme_ctx = ThemeCtx(theme, set_theme);
            provide_context(theme_ctx.clone());
            theme_ctx
        }
        Some(theme) => theme,
    }
}

#[island]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    provide_meta_context();

    let (theme, set_theme) = signal(Theme::new());
    let theme_ctx = ThemeCtx(theme, set_theme);
    provide_context(theme_ctx.clone());

    Effect::new(move || {
        window()
            .and_then(|w| w.local_storage().ok())
            .and_then(|storage| {
                let theme = &theme.get().to_string();
                storage?.set_item(THEME_STORAGE_KEY, theme).ok()
            });
    });

    let theme = theme_ctx.0;

    view! {
        <Html {..} data-theme={theme.get().to_string()} />
        {children()}
    }
}
