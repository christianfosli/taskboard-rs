use std::iter;

use taskboard_core_lib::{commands::CreateTaskCommand, uuid::Uuid, ProjectTasks, Task};
use wasm_bindgen::JsValue;
use yew::{
    format::Json, format::Nothing, html, prelude::*, services::fetch::FetchTask,
    services::fetch::Request, services::fetch::Response, services::ConsoleService,
    services::FetchService, web_sys, Component, ComponentLink, Html, ShouldRender,
};

use super::taskbox::TaskBox;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");

pub struct Project {
    link: ComponentLink<Self>,
    id: Uuid,
    title: String,
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
    Added(Task),
    Update(Task),
    FetchCompleted(ProjectTasks),
    FetchFailed,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

impl Project {
    fn fetch_tasks(&mut self) {
        self.fetch_status = FetchStatus::Loading;

        match TASK_SERVICE_URL {
            Some(url) => {
                let req = Request::get(&format!("{}/project-tasks/{}", url, self.id))
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
            project_id: self.id,
            title,
            estimate,
        };

        let req = Request::post(&format!("{}/task", TASK_SERVICE_URL.unwrap()))
            .header("Content-Type", "application/json")
            .body(Json(&command))
            .map_err(|_| JsValue::from("Failed to build post request"))?;

        let task = FetchService::fetch(
            req,
            self.link
                .callback(|res: Response<Json<Result<Task, anyhow::Error>>>| {
                    if let (meta, Json(Ok(body))) = res.into_parts() {
                        if meta.status.is_success() {
                            return Msg::Added(body);
                        }
                    }
                    Msg::FetchFailed
                }),
        )
        .map_err(|_| JsValue::from("Failed to send post request for adding task"))?;

        self.ft = Some(task);

        Ok(())
    }
}

impl Component for Project {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: props.id,
            title: String::from("..."),
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
            Msg::Added(task) => {
                self.tasks = Some(match self.tasks.clone() {
                    Some(t) => t.into_iter().chain(iter::once(task)).collect(),
                    None => vec![task],
                });
                return true;
            }
            Msg::Update(task) => ConsoleService::log(&format!("Should be updating {}", task.title)),
            Msg::FetchCompleted(tasks) => {
                self.title = tasks.project_name;
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
            <>
            <h1>{ &format!("Taskboard for {}", self.title) }</h1>
            <div class="status-msg">{ status_message }</div>
            <button id="newtask-btn" onclick=self.link.callback(|_| Msg::Add)>{ "new"} </button>
            {task_list}
            </>
        }
    }

    fn destroy(&mut self) {}
}
