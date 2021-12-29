use taskboard_core_lib::uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{error_box::ErrorBox, health::Health, home::Home, project::Project};

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

#[function_component(App)]
pub fn app() -> Html {
    let error: UseStateHandle<Option<String>> = use_state(|| None);
    let set_error = {
        let error = error.clone();
        Callback::from(move |e| error.set(e))
    };

    let switch = {
        let set_error = set_error.clone();

        move |route: &AppRoute| match route {
            AppRoute::Project { id } => html! {< Project id={*id} set_err={set_error.clone()} />},
            AppRoute::Health => html! {< Health /> },
            AppRoute::NotFound => html! { <h3> { "Page Not Found" } </h3> },
            AppRoute::Home => html! {< Home set_err={set_error.clone()} />},
        }
    };

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
                <ErrorBox err={(*error).clone()} set_err={set_error}/>
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
