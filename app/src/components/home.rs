use log::error;
use yew::{html, utils::document, Component, ComponentLink};

use crate::components::{create_project::CreateProject, search_project::SearchProject};

pub struct Home {
    _link: ComponentLink<Self>,
    description: String,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Query description from html document to avoid repetition.
        let description = document()
            .query_selector("head > meta[name=description]")
            .unwrap_or(None)
            .and_then(|el| el.get_attribute("content"))
            .unwrap_or_else(|| {
                error!("Failed to get app description from document head");
                String::default()
            });

        Self {
            _link: link,
            description,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
            <p>{ &self.description }</p>
            < SearchProject />
            < CreateProject />
            </>
        }
    }
}
