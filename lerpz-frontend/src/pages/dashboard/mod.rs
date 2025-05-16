use crate::components::Text;

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

const CATEGORIES: [Category; 1] = [
    Category {
        path: "/overview",
        name: "Overview",
        routes: &[
            Route {
                path: "",
                name: "Dashboard",
                icon: "icon-dashboard",
            },
            Route {
                path: "/users",
                name: "Dashboard",
                icon: "icon-dashboard",
            },
            Route {
                path: "/groups",
                name: "Dashboard",
                icon: "icon-dashboard",
            }
        ]
    }
];

#[component]
pub fn DashboardPage() -> impl IntoView {

    view! {
        <div>
            {CATEGORIES.iter()
            .map(|c| view! { <Category name=c.name routes=c.routes /> })
            .collect_view()}
        </div>
    }
}

#[component]
fn Category(
    name: &'static str,
    routes: &'static [Route],
) -> impl IntoView {
    view! {
        <div>
            <Text size="xs">
                {name}
            </Text>
            {routes.iter().map(|r| view! {
                <Route name=r.name />
            }).collect_view()}
        </div>
    }
}

#[component]
fn Route(
    name: &'static str,
) -> impl IntoView {
    view! {
        <Text size="xs">
            {name}
        </Text>
    }
}
