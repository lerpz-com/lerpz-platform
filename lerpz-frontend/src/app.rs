use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html
            lang="en"
            dir="ltr"
            class="dark scrollbar-thin scrollbar-track-transparent
            scrollbar-thumb-slate-500 dark:scrollbar-thumb-slate-600
            hover:scrollbar-thumb-slate-400"
        >
            <head>
                <title>"Lerpz"</title>
                <meta charset="utf-8"/>
                <meta name="description" content="Lerpz website, created using Leptos!"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="manifest" href="/manifest.json"/>
                // Fontawesome icons
                <link rel="stylesheet" href="/fontawesome/css/all.min.css"/>
                // Tailwind generated stylesheet
                <link rel="stylesheet" id="leptos" href="/pkg/lerpz-frontend.css"/>
                // Leptos stuff
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true islands_router=true/>
            </head>
            <body class="bg-light dark:bg-dark mx-auto px-8 max-w-5xl min-h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="my-8">
                <Routes fallback=|| view! { <div>"loading"</div> } transition=true>
                    <Route path=path!("/") view=|| view! { <div>"Test"</div> }/>
                </Routes>
            </main>
        </Router>
    }
}
