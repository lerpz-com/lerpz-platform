use crate::cmps::Text;
use crate::hooks::ThemeProvider;
use crate::pages::*;

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_router::components::ParentRoute;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr">
            <head>
                <title>"Lerpz - Identity Platform"</title>
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
            <body class="bg-light dark:bg-dark h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <Router>
                <Routes fallback=Fallback transition=true>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/login") view=LoginPage/>
                    <Route path=path!("/register") view=RegisterPage/>
                    <ParentRoute path=path!("/dashboard") view=DashboardLayout>
                        <Route path=path!("") view=DashboardOverview/>
                        <Route path=path!("/users") view=DashboardUsers/>
                        <Route path=path!("/groups") view=DashboardGroups/>
                    </ParentRoute>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

#[component]
pub fn Fallback() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-full">
            <Text size="xl" weight="bold">"404"</Text>
            <Text size="lg" weight="medium">"Not Found"</Text>
        </div>
    }
}
