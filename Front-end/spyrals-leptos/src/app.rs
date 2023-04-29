use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>


        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/fetch" view=|cx| view! { cx, <Fetch/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}




#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Msg {
    message: String,
}

#[derive(Error, Clone, Debug)]
pub enum FetchError {
    #[error("Error loading data from serving.")]
    Request,
    #[error("Error deserializaing message data from request.")]
    Json
}

// async fn fetch_msgs() -> Result<Vec<String>, FetchError> {
async fn fetch_msgs(_count: u32) -> Result<Vec<String>, FetchError> {
    let res = reqwasm::http::Request::get(&format!(
        "http://127.0.0.1:8000/",
    ))
    .send()
    .await
    .map_err(|_| FetchError::Request)?

    // convert it to JSON
    .json::<Vec<Msg>>()
    .await
    .map_err(|_| FetchError::Json)?

    // extract the message field for each message
    .into_iter()
    .map(|msg| msg.message)
    .collect::<Vec<_>>();
    Ok(res)
}


#[component]
pub fn Fetch(cx: Scope) -> impl IntoView {
    let (count, _set_count) = create_signal::<u32>(cx, 0);
    let msgs = create_local_resource(cx, count, fetch_msgs);

    let msg_view = move || {
        msgs.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|s| view! { cx, <span>{s}</span> })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! { cx,
        <div>
            <p class="text-red-600">{msg_view}</p>
        </div>
    }
}