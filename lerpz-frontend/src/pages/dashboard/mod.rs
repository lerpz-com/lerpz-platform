use crate::components::Text;

use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div>
            <Text size="4xl" weight="bold">
                "Dashboard"
            </Text>
            <Text size="4xl" weight="bold">
                "Welcome to the dashboard!"
            </Text>
        </div>
    }
}
