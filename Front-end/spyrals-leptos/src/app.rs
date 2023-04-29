use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod pages;
use pages::home::*;
use pages::accounts::*;
use pages::fetch::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <LandingPage/> }/>

                    // Account
                    <Route path="account/register" view=|cx| view! { cx, <Register/> }/>
                    <Route path="account/activate" view=|cx| view! { cx, <Activate/> }/>
                    <Route path="account/login" view=|cx| view! { cx, <Login/> }/>
                    <Route path="account/oauth/:id" view=|cx| view! { cx, <Oauth/> }/>
                    <Route path="account/logout" view=|cx| view! { cx, <Logout/> }/>
                    <Route path="account/password-reset" view=|cx| view! { cx, <PasswordReset/> }/>
                    <Route path="account/password-reset-token" view=|cx| view! { cx, <PasswordResetToken/> }/>

                    <Route path="fetch/example" view=|cx| view! { cx, <Fetch/> }/>
                </Routes>
            </main>
        </Router>
    }
}