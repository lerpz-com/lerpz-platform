use crate::components::Text;
use crate::theme::use_theme;

use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div>
            <Text size="4xl" weight="bold">
                "Dashboard"
            </Text>
            <Text size="4xl" weight="bold">
                "Welcome to the dashboard!"
            </Text>
            <ToggleThemeButton />
        </div>
    }
}

#[island]
fn ToggleThemeButton() -> impl IntoView {
    let (_, set_theme) = use_theme();

    view! {
        <button
            on:click=move |_| {
                set_theme.update(|theme| {
                    theme.toggle();
                });
            }
        >
            "Toggle Theme"
        </button>
    }
}
