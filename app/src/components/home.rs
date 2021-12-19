use log::error;
use yew::prelude::*;

use crate::components::{create_project::CreateProject, search_project::SearchProject};

#[function_component(Home)]
pub fn home() -> Html {
    let document = web_sys::window().unwrap().document().unwrap();

    let description = document
        .query_selector("head > meta[name=description]")
        .unwrap_or(None)
        .and_then(|el| el.get_attribute("content"))
        .unwrap_or_else(|| {
            error!("Failed to get app description from document head");
            String::default()
        });

    let heads_up = "🚧 This is mostly a proof-of-concept to play with some fun technology.
            Feel free to add your own projects/tasks,
            but be aware that they will be publically accessible and may be edited
            or removed by others 🚧"
        .to_string();

    html! {
        <>
        <p>{ &description }</p>
        <p class="box-wip">{ &heads_up }</p>
        < SearchProject />
        < CreateProject />
        </>
    }
}
