use leptos::prelude::*;

use crate::cmps::Text;

#[component]
pub fn RegisterPage() -> impl IntoView {
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
