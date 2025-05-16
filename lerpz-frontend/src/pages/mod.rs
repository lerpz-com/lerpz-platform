pub mod dashboard;

use leptos::prelude::*;

pub use crate::pages::dashboard::DashboardPage;

use crate::components::Text;

#[component]
pub fn HomePage() -> impl IntoView {
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
