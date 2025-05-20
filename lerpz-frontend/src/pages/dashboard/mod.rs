use crate::{
    components::Text,
    hooks::{Theme, use_theme},
};

use leptos::prelude::*;

struct Category {
    path: &'static str,
    name: &'static str,
    routes: &'static [Route],
}

struct Route {
    path: &'static str,
    name: &'static str,
    icon: &'static str,
}

const CATEGORIES: &'static [Category] = &[Category {
    path: "/overview",
    name: "Overview",
    routes: &[
        Route {
            path: "/dashboard",
            name: "Dashboard",
            icon: "fa-solid fa-gauge",
        },
        Route {
            path: "/dashboard",
            name: "Users",
            icon: "fa-solid fa-users",
        },
        Route {
            path: "/dashboard",
            name: "Groups",
            icon: "fa-solid fa-user-group",
        },
    ],
}];

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <ThemeToggleButton />
        <div class="px-4 py-8">
            {CATEGORIES.iter()
            .map(|c| view! {
                <DashboardCategory
                    _path=c.path
                    name=c.name
                    routes=c.routes
                />
            })
            .collect_view()}
        </div>
    }
}

#[component]
fn DashboardCategory(
    _path: &'static str,
    name: &'static str,
    routes: &'static [Route],
) -> impl IntoView {
    view! {
        <div>
            <Text size="xs">
                {name}
            </Text>
            {routes.iter().map(|r| view! {
                <DashboardRoute
                    path=r.path
                    icon=r.icon
                    name=r.name
                />
            }).collect_view()}
        </div>
    }
}

#[component]
fn DashboardRoute(path: &'static str, icon: &'static str, name: &'static str) -> impl IntoView {
    view! {
        <a href=path class="px-2 py-4">
            <Text size="xs">
                <i class=format!("aspect-square mr-4 {}", icon) />
                {name}
            </Text>
        </a>
    }
}

#[island]
fn ThemeToggleButton() -> impl IntoView {
    let Theme(theme, set_theme) = use_theme();

    view! {
        <button on:click=move |_| set_theme.update(|theme| theme.toggle())>
            {move || {format!("Toggle Theme {}", theme.get())}}
        </button>
    }
}
