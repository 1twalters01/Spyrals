use leptos::*;
use leptos_meta::{Title, TitleProps};

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Register"/>
        <h1>"Register"</h1>
    }
}

#[component]
pub fn Activate(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Activate"/>
        <h1>"Activate"</h1>
    }
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Login"/>
        <h1>"Login"</h1>
    }
}

#[component]
pub fn Oauth(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Oauth"/>
        <h1>"Oauth"</h1>
    }
}

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Logout"/>
        <h1>"Logout"</h1>
    }
}

#[component]
pub fn PasswordReset(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Password Reset"/>
        <h1>"Password Reset"</h1>
    }
}

#[component]
pub fn PasswordResetToken(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Password Reset Token"/>
        <h1>"Password Reset Token"</h1>
    }
}