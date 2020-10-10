use taskboard_core_lib::uuid::Uuid;
use yew::{html, Component, ComponentLink};
use yew_router::{router::Router, Switch};

use crate::components::{home::Home, project::Project};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/{projectid}"]
    Project(Uuid),
    #[to = "/"]
    Index,
}

pub struct Model {}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
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
            <main>
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Project(projectid) => html! {< Project id=projectid />},
                        AppRoute::Index => html! {< Home />},
                    }
                })
            />
            </main>
            <footer>
                <p>
                { "Written in Rust by Christian Fosli | Source code on " }
                <a href="https://github.com/christianfosli/taskboard-rs">{ "GitHub" }</a>
                </p>
            </footer>
            </>
        }
    }
}
