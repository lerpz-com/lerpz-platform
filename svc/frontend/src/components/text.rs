use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Variant {
    #[tw(class = "text-slate-900 dark:text-slate-100", default)]
    Default,
    #[tw(class = "text-slate-500 dark:text-slate-400")]
    Dimmed,
}

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Size {
    #[tw(class = "text-sm")]
    Sm,
    #[tw(class = "text-md", default)]
    Md,
    #[tw(class = "text-lg")]
    Lg,
    #[tw(class = "text-xl")]
    Xl,
    #[tw(class = "text-2xl")]
    #[strum(serialize = "2xl")]
    Xl2,
    #[tw(class = "text-3xl")]
    #[strum(serialize = "3xl")]
    Xl3,
    #[tw(class = "text-4xl")]
    #[strum(serialize = "4xl")]
    Xl4,
    #[tw(class = "text-5xl")]
    #[strum(serialize = "5xl")]
    Xl5,
    #[tw(class = "text-6xl")]
    #[strum(serialize = "6xl")]
    Xl6,
    #[tw(class = "text-7xl")]
    #[strum(serialize = "7xl")]
    Xl7,
}

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TextWeight {
    #[tw(class = "font-thin")]
    Thin,
    #[tw(class = "font-extralight")]
    Extralight,
    #[tw(class = "font-light")]
    Light,
    #[tw(class = "font-normal", default)]
    Normal,
    #[tw(class = "font-medium")]
    Medium,
    #[tw(class = "font-semibold")]
    Semibold,
    #[tw(class = "font-bold")]
    Bold,
    #[tw(class = "font-extrabold")]
    Extrabold,
    #[tw(class = "font-black")]
    Black,
}

#[component]
pub fn Text(
    children: Children,
    #[prop(optional, into)] variant: Cow<'static, str>,
    #[prop(optional, into)] size: Cow<'static, str>,
    #[prop(optional, into)] weight: Cow<'static, str>,
    #[prop(optional, into)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    let variant = Variant::from_str(&variant).unwrap_or_default();
    let size = Size::from_str(&size).unwrap_or_default();
    let weight = TextWeight::from_str(&weight).unwrap_or_default();

    view! {
        <p class=tw_merge!(variant, size, weight, class)>
            {children()}
        </p>
    }
}
