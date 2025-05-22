use cookie::{Cookie, time::OffsetDateTime};
use leptos::{logging::debug_warn, prelude::*};
use leptos_meta::{Html, provide_meta_context};
use strum::{Display, EnumString};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, window};

const THEME_STORAGE_KEY: &str = "lerpz-theme";

#[derive(Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

#[derive(Clone)]
pub struct ThemeContext(pub ReadSignal<ThemeKind>, pub WriteSignal<ThemeKind>);

impl ThemeKind {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use std::str::FromStr;

                let cookie_str = window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
                    .and_then(|d| d.cookie().ok())
                    .unwrap_or_default();

                let cookie = Cookie::split_parse(cookie_str)
                    .filter_map(|c| c.ok())
                    .find(|c| c.name() == THEME_STORAGE_KEY);

                if let Some(cookie) = cookie {
                    ThemeKind::from_str(cookie.value()).unwrap_or_default()
                } else {
                    ThemeKind::default()
                }
            } else if #[cfg(feature = "ssr")] {
                // use axum::extract::Cookie;
                // use leptos_axum::extract;

                // let (Cookie, query): (Cookie) = extract().await?;
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

impl ThemeContext {
    pub fn new() -> Self {
        let (theme, set_theme) = signal(ThemeKind::new());
        ThemeContext(theme, set_theme)
    }
}

pub fn use_theme() -> (ReadSignal<ThemeKind>, WriteSignal<ThemeKind>) {
    match use_context::<ThemeContext>() {
        Some(ctx) => (ctx.0, ctx.1),
        None => {
            debug_warn!(
                "ThemeContext was not found. Did you forget to add the
                `<ThemeProvider />` component?"
            );
            let ctx = ThemeContext::new();
            provide_context(ctx.clone());
            (ctx.0, ctx.1)
        }
    }
}

#[island]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    provide_meta_context();

    let (theme, set_theme) = signal(ThemeKind::new());
    let theme_ctx = ThemeContext(theme, set_theme);
    provide_context(theme_ctx.clone());

    Effect::new(move || {
        let theme = theme.get();
        let cookie = Cookie::build((THEME_STORAGE_KEY, theme.to_string()))
            // .expires(OffsetDateTime::now_utc())
            .build();

        debug_warn!("Setting theme cookie: {:?}", &cookie);

        window()
            .and_then(|w| w.document())
            .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
            .and_then(|d| d.set_cookie(&cookie.to_string()).ok());
    });

    view! {
        <Html {..} data-theme={move || theme.get().to_string()} />
        {children()}
    }
}
