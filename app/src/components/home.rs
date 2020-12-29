use yew::{html, Component, ComponentLink};

use crate::components::{create_project::CreateProject, search_project::SearchProject};

pub struct Home {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
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
            <p>{ "Work-in-progress application for managing tasks and tracking progress." }</p>
            < SearchProject />
            < CreateProject />
            </>
        }
    }
}
