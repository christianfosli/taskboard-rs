use taskboard_core_lib::Project;
use yew::prelude::*;

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[function_component(CreateProject)]
pub fn create_project() -> Html {
    let name = use_state(|| "");

    let handle_input = Callback::from(|e: InputEvent| name.set(&e.data().unwrap()));
    let handle_submit = Callback::from(|e: FocusEvent| {
        e.prevent_default();
        todo!("Create the project...");
    });

    let error: UseStateHandle<Option<String>> = use_state(|| None);
    let error_html = match *error {
        Some(e) => html! {
            <div class="error">
            { e }
            </div>
        },
        None => html! {},
    };

    let created: UseStateHandle<Option<Project>> = use_state(|| None);
    let created_html = match *created {
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
            <input required={true} type="text" id="create-project-name-field" name="name" value={*name} oninput={handle_input} />
            <input type="submit" />
        </form>
        {error_html}
        {created_html}
        </>
    }
}

//pub struct CreateProject {
//    link: ComponentLink<Self>,
//    name: String,
//    ft: Option<FetchTask>,
//    created: Option<Project>,
//    error: Option<String>,
//}

//pub enum Msg {
//    SetName(String),
//    Create,
//    SetCreated(Project),
//    SetError(String),
//}

//    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        match msg {
//            Msg::SetName(name) => {
//                self.name = name;
//            }
//            Msg::Create => {
//                let command = CreateProjectCommand {
//                    name: self.name.clone(),
//                };
//
//                Request::post(PROJECT_SERVICE_URL.unwrap())
//                    .header("Content-Type", "application/json")
//                    .body(Json(&command))
//                    .map(|req| {
//                        let callback = self.link.callback(
//                            |res: Response<Json<Result<Project, anyhow::Error>>>| {
//                                if let (meta, Json(Ok(body))) = res.into_parts() {
//                                    if meta.status.is_success() {
//                                        return Msg::SetCreated(body);
//                                    }
//                                }
//                                Msg::SetError(String::from("An error occured"))
//                            },
//                        );
//
//                        self.ft = FetchService::fetch(req, callback).ok();
//                    })
//                    .unwrap_or_else(|err| {
//                        self.link.send_message(Msg::SetError(err.to_string()));
//                    });
//
//                return false;
//            }
//            Msg::SetCreated(project) => {
//                self.created = Some(project);
//                self.error = None;
//            }
//            Msg::SetError(message) => {
//                log::error!("Create project failed: {}", &message);
//                self.error = Some(message);
//            }
//        }
//        true
//    }
