use taskboard_core_lib::uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{health::Health, home::Home, project::Project};

const BUILD_VERSION: Option<&'static str> = option_env!("BUILD_VERSION");

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/project/:id")]
    Project { id: Uuid },
    #[at("/healthz")]
    Health,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Project { id } => html! {< Project id={*id} />},
        AppRoute::Health => html! {< Health /> },
        AppRoute::NotFound => html! { <h3> { "Page Not Found" } </h3> },
        AppRoute::Home => html! {< Home />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <header>
            <h1>{ "Taskboard.cloud" }</h1>
            <nav>
                <Link<AppRoute> to={AppRoute::Home}>{ "🏠 Home" }</Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Health}> { "💓 Health" }</Link<AppRoute>>
            </nav>
        </header>
        <main>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </main>
        <footer>
            <p>
            { "Source code on " }
            <a href="https://github.com/christianfosli/taskboard-rs">{ "GitHub" }</a>
            { format!(" | Version {}", BUILD_VERSION.unwrap_or("???")) }
            { " | MIT License" }
            </p>
        </footer>
        </BrowserRouter>
    }
}
