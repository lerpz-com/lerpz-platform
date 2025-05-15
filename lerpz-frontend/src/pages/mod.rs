pub mod dashboard;

use leptos::prelude::*;

pub use crate::pages::dashboard::Dashboard;

use crate::components::Text;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Text weight="bold" size="2xl">
            "Lerpz"
        </Text>
        <a href="/dashboard">
            <Text weight="bold">
                "Dashboard"
            </Text>
        </a>
    }
}
