pub mod dashboard;

use leptos::prelude::*;

pub use dashboard::*;

use crate::cmps::Text;

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
