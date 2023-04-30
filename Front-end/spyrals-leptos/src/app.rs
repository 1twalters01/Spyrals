use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod pages;
use pages::home::*;
use pages::accounts::*;
use pages::subscriptions::*;
use pages::settings::*;
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
                    <Route path="home/" view=|cx| view! { cx, <Home/> }/>

                    // Account
                    <Route path="account/register/" view=|cx| view! { cx, <Register/> }/>
                    <Route path="account/activate/" view=|cx| view! { cx, <Activate/> }/>
                    <Route path="account/login/" view=|cx| view! { cx, <Login/> }/>
                    <Route path="account/oauth/:id/" view=|cx| view! { cx, <Oauth/> }/>
                    <Route path="account/logout/" view=|cx| view! { cx, <Logout/> }/>
                    <Route path="account/forgot-username/" view=|cx| view! { cx, <ForgotUsername/> }/>
                    <Route path="account/password-reset/" view=|cx| view! { cx, <PasswordReset/> }/>
                    <Route path="account/password-reset-token/:uid/:token/" view=|cx| view! { cx, <PasswordResetToken/> }/>

                    // Settings
                    <Route path="settings/change-email/" view=|cx| view! { cx, <ChangeEmail/> }/>
                    <Route path="settings/change-password/" view=|cx| view! { cx, <ChangePassword/> }/>
                    <Route path="settings/change-username/" view=|cx| view! { cx, <ChangeUsername/> }/>
                    <Route path="settings/close-account/" view=|cx| view! { cx, <CloseAccount/> }/>
                    <Route path="settings/themes/" view=|cx| view! { cx, <Themes/> }/>
                    <Route path="settings/premium/" view=|cx| view! { cx, <Premium/> }/>
                    <Route path="settings/2FA/" view=|cx| view! { cx, <TwoFactorAuth/> }/>

                    // Subscriptions
                    <Route path="subscriptions/" view=|cx| view! { cx, <Subscriptions/> }/>

                    // Fetch examples
                    <Route path="fetch/get/" view=|cx| view! { cx, <FetchGet/> }/>
                    <Route path="fetch/post/" view=|cx| view! { cx, <FetchPost/> }/>
                </Routes>
            </main>
        </Router>
    }
}