use leptos::*;
use leptos_meta::*;

#[component]
pub fn LandingPage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <Title text="Welcome to Leptos"/>

        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Home"/>
        <h1>"Home"</h1>
    }
}