use taskboard_core_lib::{commands::CreateProjectCommand, Project};
use yew::{
    format::Json,
    prelude::*,
    services::fetch::Response,
    services::{
        fetch::{FetchTask, Request},
        FetchService,
    },
};

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

pub struct CreateProject {
    link: ComponentLink<Self>,
    name: String,
    ft: Option<FetchTask>,
    created: Option<Project>,
    error: Option<String>,
}

pub enum Msg {
    SetName(String),
    Create,
    SetCreated(Project),
    SetError(String),
}

impl Component for CreateProject {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::from(""),
            ft: None,
            created: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetName(name) => {
                self.name = name;
            }
            Msg::Create => {
                let command = CreateProjectCommand {
                    name: self.name.clone(),
                };

                let req = Request::post(PROJECT_SERVICE_URL.unwrap())
                    .header("Content-Type", "application/json")
                    .body(Json(&command))
                    .expect("failed building request");

                let callback =
                    self.link
                        .callback(|res: Response<Json<Result<Project, anyhow::Error>>>| {
                            if let (meta, Json(Ok(body))) = res.into_parts() {
                                if meta.status.is_success() {
                                    return Msg::SetCreated(body);
                                }
                            }
                            Msg::SetError(String::from("An error occured"))
                        });

                self.ft = FetchService::fetch(req, callback).ok();
                return false;
            }
            Msg::SetCreated(project) => {
                self.created = Some(project);
                self.error = None;
            }
            Msg::SetError(message) => {
                self.error = Some(message);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_input = self.link.callback(|e: InputData| Msg::SetName(e.value));

        let handle_submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Create
        });

        let error = match &self.error {
            Some(e) => html! {
                <div class="error">
                { e }
                </div>
            },
            None => html! {},
        };

        let created = match &self.created {
            Some(p) => html! {
                <a href={format!("/{}", p.id)}>{ &format!("Project {} created successfully", p.name) }</a>
            },
            None => html! {},
        };

        html! {
            <>
            <h3>{ "Create a new project" }</h3>
            <form onsubmit={handle_submit}>
                <label for="create-project-name-field">{ "Project name" }</label>
                <input required={true} type="text" id="create-project-name-field" name="name" value={&self.name} oninput={handle_input} />
                <input type="submit" />
            </form>
            {error}
            {created}
            </>
        }
    }
}
