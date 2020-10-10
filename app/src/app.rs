use crate::components::taskbox::TaskBox;
use anyhow::Error;
use taskboard_core_lib::{commands::CreateTaskCommand, uuid::Uuid, ProjectTasks, Task};
use wasm_bindgen::JsValue;
use yew::{
    format::Json,
    format::Nothing,
    prelude::*,
    services::fetch::FetchTask,
    services::fetch::Request,
    services::fetch::Response,
    services::{ConsoleService, FetchService},
    web_sys,
};

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

pub struct Model {
    link: ComponentLink<Self>,
    project_id: Uuid,
    project_title: String,
    tasks: Option<Vec<Task>>,
    ft: Option<FetchTask>,
    fetch_status: FetchStatus,
}

enum FetchStatus {
    Loading,
    Completed,
    Failed,
}

pub enum Msg {
    Add,
    Added,
    Update(Task),
    FetchCompleted(ProjectTasks),
    FetchFailed,
}

impl Model {
    fn fetch_tasks(&mut self) {
        self.fetch_status = FetchStatus::Loading;

        match TASK_SERVICE_URL {
            Some(url) => {
                let req = Request::get(&format!("{}/project-tasks/{}", url, self.project_id))
                    .body(Nothing)
                    .unwrap();

                let callback = self.link.callback(
                    |res: Response<Json<Result<ProjectTasks, anyhow::Error>>>| {
                        if let (meta, Json(Ok(body))) = res.into_parts() {
                            if meta.status.is_success() {
                                return Msg::FetchCompleted(body);
                            }
                        }
                        Msg::FetchFailed
                    },
                );

                self.ft = Some(FetchService::fetch(req, callback).unwrap());
            }
            None => ConsoleService::error("Unable to fetch tasks because the URL is not set"),
        }
    }

    fn add_task(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or(JsValue::from_str("No window avilable"))?;
        let title = window
            .prompt_with_message("Enter task name")?
            .ok_or("No task name specified")?;
        let estimate: Option<u8> =
            match window.prompt_with_message("Enter estimate (or leave blank)")? {
                Some(est) => {
                    if est.is_empty() {
                        None
                    } else {
                        Some(
                            est.parse::<u8>()
                                .map_err(|err| JsValue::from(err.to_string()))?,
                        )
                    }
                }
                None => None,
            };

        let command = CreateTaskCommand {
            project_id: self.project_id,
            title,
            estimate,
        };

        let req = Request::post(&format!("{}/task", TASK_SERVICE_URL.unwrap()))
            .header("Content-Type", "application/json")
            .body(Json(&command))
            .map_err(|_| JsValue::from("Failed to build post request"))?;

        let task = FetchService::fetch(
            req,
            self.link.callback(|res: Response<Result<String, Error>>| {
                if res.status().is_success() {
                    Msg::Added
                } else {
                    Msg::FetchFailed
                }
            }),
        )
        .map_err(|_| JsValue::from("Failed to send post request for adding task"))?;

        self.ft = Some(task);

        Ok(())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            project_id: Uuid::nil(), // TODO - get from query param
            project_title: String::from("tmp"),
            tasks: None,
            ft: None,
            fetch_status: FetchStatus::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => self
                .add_task()
                .unwrap_or_else(|err| ConsoleService::error(&err.as_string().unwrap())),
            Msg::Added => self.fetch_tasks(),
            Msg::Update(task) => ConsoleService::log(&format!("Should be updating {}", task.title)),
            Msg::FetchCompleted(tasks) => {
                self.tasks = Some(tasks.tasks);
                self.fetch_status = FetchStatus::Completed;
                return true;
            }
            Msg::FetchFailed => {
                self.fetch_status = FetchStatus::Failed;
                return true;
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::log("App changed");
        false
    }

    fn rendered(&mut self, first_render: bool) {
        ConsoleService::log("App rendered");
        if first_render {
            self.fetch_tasks();
        }
    }

    fn view(&self) -> Html {
        let to_taskbox = |model: &Task| {
            html! {
                <TaskBox onchange=self.link.callback(|x| Msg::Update(x)) data=model />
            }
        };

        let task_list = match self.tasks.clone() {
            Some(tasks) => html! {
                <ul>
                {tasks.iter().map(|t| to_taskbox(t)).collect::<Html>()}
                </ul>
            },
            None => html! {
                <ul></ul>
            },
        };

        let status_message = match self.fetch_status {
            FetchStatus::Loading => "Loading...",
            FetchStatus::Completed => "",
            FetchStatus::Failed => "Failed to fetch tasks 😭",
        };

        html! {
            <main>
            <h1>{ &format!("Taskboard for {}", self.project_title) }</h1>
            <div class="status-msg">{ status_message }</div>
            <button id="newtask-btn" onclick=self.link.callback(|_| Msg::Add)>{ "new"} </button>
            {task_list}
            </main>
        }
    }

    fn destroy(&mut self) {}
}
