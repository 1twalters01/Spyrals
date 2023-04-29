use leptos::*;
use leptos_meta::*;

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    // If I can get the url param then I will replace the lower options
    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
    }
}

#[component]
pub fn ChangeEmail(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Change email"</p>
    }
}

#[component]
pub fn ChangePassword(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Change password"</p>
    }
}

#[component]
pub fn ChangeUsername(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Change username"</p>
    }
}

#[component]
pub fn CloseAccount(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Close account"</p>
    }
}

#[component]
pub fn Themes(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Themes"</p>
    }
}

#[component]
pub fn Premium(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Premium"</p>
    }
}

#[component]
pub fn TwoFactorAuth(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <p>"Two factor authentication"</p>
    }
}