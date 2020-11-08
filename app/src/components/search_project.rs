use taskboard_core_lib::Project;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::Response,
    services::{
        fetch::{FetchTask, Request},
        ConsoleService, FetchService,
    },
};

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

pub struct SearchProject {
    link: ComponentLink<Self>,
    search_query: String,
    matches: Vec<Project>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    SetSearch(String),
    PerformSearch,
    SearchCompleted(Vec<Project>),
    SearchFailed(String),
}

impl Component for SearchProject {
    type Message = Msg;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            search_query: String::from(""),
            matches: Vec::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetSearch(query) => self.search_query = query,
            Msg::PerformSearch => {
                let req = Request::get(format!(
                    "{}/search/{}",
                    PROJECT_SERVICE_URL.unwrap(),
                    self.search_query
                ))
                .body(Nothing)
                .expect("failed to build request");

                let callback = self.link.callback(
                    |res: Response<Json<Result<Vec<Project>, anyhow::Error>>>| {
                        if let (meta, Json(Ok(body))) = res.into_parts() {
                            if meta.status.is_success() {
                                return Msg::SearchCompleted(body);
                            }
                        }
                        Msg::SearchFailed(String::from("An error occured"))
                    },
                );

                self.ft = FetchService::fetch(req, callback).ok();
            }
            Msg::SearchCompleted(matches) => self.matches = matches,
            Msg::SearchFailed(message) => ConsoleService::error(&message),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_input = self.link.callback(|e: InputData| Msg::SetSearch(e.value));

        let handle_submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::PerformSearch
        });

        let to_li = |project: &Project| {
            html! {
                <li><a href={format!("/{}", project.id)}>{ &project.name }</a></li>
            }
        };

        let matches = self.matches.iter().map(|p| to_li(p)).collect::<Html>();

        html! {
            <>
            <h3>{ "Search for an existing project" }</h3>
            <form onsubmit={handle_submit}>
                <label for="search-project-field">{ "Project name" }</label>
                <input type="text" id="search-project-field" name="query" value={&self.search_query} oninput={handle_input} required={true}/>
                <input type="submit" value="Search"/>
            </form>
            <ul>
                {matches}
            </ul>
            </>
        }
    }
}
