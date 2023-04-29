use leptos::*;
use leptos_meta::{Title, TitleProps};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
pub fn FetchGet(cx: Scope) -> impl IntoView {
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
        <Title text="Fetch Get Example"/>

        <div>
            <p class="text-red-600">{msg_view}</p>
        </div>
    }
}




// Not actually coded yet
async fn post_msgs(_count: u32) -> Result<Vec<String>, FetchError> {
    let res = reqwasm::http::Request::post(&format!(
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

// Fetch post example
#[component]
pub fn FetchPost(cx: Scope) -> impl IntoView {

    view! { cx,
        <Title text="Fetch Post Example"/>
        <h1>"Settings"</h1>
        <p>"Fetch post"</p>
    }
}