use yew::{html, Component, ComponentLink};

pub struct Home {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
            <h1>{ "Taskboard" }</h1>
            <p>{ "Work-in-progress application for managing tasks and tracking progress." }</p>
            <p>
             { "It is not yet possible to create projects or search for projects in the UI...
                Use the project-service REST API, then enter /{project-id} to manage tasks for it." }
            </p>
            </>
        }
    }
}
