mod groups;
mod overview;
mod users;

use std::sync::LazyLock;

use crate::{components::Text, hooks::use_theme};
pub use crate::pages::dashboard::{groups::*, overview::*, users::*};

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

static CATEGORIES: LazyLock<&[Category]> = LazyLock::new(|| {
    &[Category {
        path: "",
        name: "Overview",
        routes: &[
            Route {
                path: "",
                name: "Dashboard",
                icon: "fa-solid fa-gauge",
            },
            Route {
                path: "/users",
                name: "Users",
                icon: "fa-solid fa-users",
            },
            Route {
                path: "/groups",
                name: "Groups",
                icon: "fa-solid fa-user-group",
            },
        ],
    }]
});

fn full_path(category: &'static Category, route: &'static Route) -> String {
    let prefix = "/dashboard";

    if route.path.is_empty() {
        format!("{}{}", prefix, category.path.to_string())
    } else {
        format!(
            "{}{}{}",
            prefix,
            category.path.to_string(),
            route.path.to_string()
        )
    }
}

#[component]
pub fn DashboardLayout() -> impl IntoView {
    view! {
        <ThemeButton />
        <div class="px-4 py-8">
            {CATEGORIES.iter()
            .map(|category| view! {
                <DashboardCategory category />
            })
            .collect_view()}
        </div>
    }
}

#[component]
fn DashboardCategory(category: &'static Category) -> impl IntoView {
    view! {
        <div>
            <Text size="xs">
                {category.name}
            </Text>
            {category.routes.iter().map(|route| view! {
                <DashboardRoute category route />
            }).collect_view()}
        </div>
    }
}

#[component]
fn DashboardRoute(category: &'static Category, route: &'static Route) -> impl IntoView {
    view! {
        <a href=full_path(category, route) class="px-2 py-4">
            <Text size="xs">
                <i class=format!("aspect-square mr-4 {}", route.icon) />
                {route.name}
            </Text>
        </a>
    }
}

#[island]
fn ThemeButton() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    view! {
        <button
            class="px-2 py-4"
            on:click=move |_| set_theme.update(|t| t.toggle())
        >
            <Text size="xs">
                {move || format!("Toggle: {}", theme.get().to_string())}
            </Text>
        </button>
    }
}
