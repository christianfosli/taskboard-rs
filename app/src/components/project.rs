use std::{cmp::Reverse, iter};

use anyhow::anyhow;
use taskboard_core_lib::{
    commands::{CreateTaskCommand, UpdateTaskCommand},
    uuid::Uuid,
    ProjectTasks, Status, Task,
};
use wasm_bindgen::JsValue;
use yew::{
    format::Json, format::Nothing, html, prelude::*, services::fetch::FetchTask,
    services::fetch::Request, services::fetch::Response, services::FetchService, web_sys,
    Component, ComponentLink, Html, ShouldRender,
};

use super::taskbox::TaskBox;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

// TODO: At the moment only FetchStatus::Failed is used to display errors to the user.
// Meaning one must check console logs to tell what actually failed.
pub struct Project {
    link: ComponentLink<Self>,
    id: Uuid,
    title: String,
    tasks: Option<Vec<Task>>,
    ft: Option<FetchTask>,
    fetch_status: FetchStatus,
    show_completed: bool,
}

enum FetchStatus {
    Loading,
    Completed,
    Failed,
}

#[derive(Debug)]
pub enum Msg {
    Add,
    Added(Task),
    Update(Task),
    UpdateSuccessful,
    Delete,
    Deleted,
    FetchCompleted(ProjectTasks),
    FetchFailed,
    ToggleShowCompleted,
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

                self.ft = FetchService::fetch(req, callback).ok();
            }
            None => log::error!("Unable to fetch tasks because the URL is not set"),
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

        let req = Request::post(&format!("{}/task/create", TASK_SERVICE_URL.unwrap()))
            .header("Content-Type", "application/json")
            .body(Json(&command))
            .map_err(|_| JsValue::from("Failed to build post request"))?;

        let callback = self
            .link
            .callback(|res: Response<Json<Result<Task, anyhow::Error>>>| {
                if let (meta, Json(Ok(body))) = res.into_parts() {
                    if meta.status.is_success() {
                        return Msg::Added(body);
                    }
                }
                Msg::FetchFailed
            });

        self.ft = FetchService::fetch(req, callback).ok();

        Ok(())
    }

    fn update_task(&mut self, task: &Task) -> Result<(), anyhow::Error> {
        log::info!("Updating task {}...", task.number);

        let command = UpdateTaskCommand {
            project_id: self.id,
            updated_task: task.clone(),
        };

        let req = Request::put(&format!("{}/task/update", TASK_SERVICE_URL.unwrap()))
            .header("Content-Type", "application/json")
            .body(Json(&command))?;

        let callback = self
            .link
            .callback(|res: Response<Result<String, anyhow::Error>>| {
                match res.status().is_success() {
                    true => Msg::UpdateSuccessful,
                    false => Msg::FetchFailed,
                }
            });

        self.ft = FetchService::fetch(req, callback).ok();

        Ok(())
    }

    fn delete_project(&mut self) -> Result<(), anyhow::Error> {
        let window = web_sys::window().ok_or(anyhow!("Window was None"))?;

        match window.confirm() {
            Ok(conf) if conf == true => {
                log::info!("Deleting project {}", self.title);
                let req = Request::delete(&format!("{}/{}", PROJECT_SERVICE_URL.unwrap(), self.id))
                    .body(Nothing)?;

                let callback =
                    self.link
                        .callback(|res: Response<Result<String, anyhow::Error>>| {
                            match res.status().is_success() {
                                true => Msg::Deleted,
                                false => Msg::FetchFailed,
                            }
                        });

                self.ft = FetchService::fetch(req, callback).ok();
            }
            Ok(_) => log::info!("Delete project aborted"),
            Err(e) => return Err(anyhow!(format!("{:?}", e))),
        }

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
            show_completed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => self
                .add_task()
                .unwrap_or_else(|e| log::error!("Error adding task: {:?}", e)),
            Msg::Added(task) => {
                self.tasks = Some(match self.tasks.clone() {
                    Some(t) => t.into_iter().chain(iter::once(task)).collect(),
                    None => vec![task],
                });
            }
            Msg::Update(task) => {
                self.update_task(&task)
                    .unwrap_or_else(|e| log::error!("Error updating task: {}", e));

                self.tasks = Some(
                    self.tasks
                        .clone()
                        .unwrap_or(Vec::new())
                        .into_iter()
                        .map(|t| {
                            if t.number == task.number {
                                task.clone()
                            } else {
                                t
                            }
                        })
                        .collect(),
                )
            }
            Msg::UpdateSuccessful => {
                log::info!("{:?}", Msg::UpdateSuccessful);
            }
            Msg::Delete => {
                self.delete_project()
                    .unwrap_or_else(|e| log::error!("Error deleting project: {}", e));
            }
            Msg::Deleted => {
                log::info!("Project deleted successfully. Redirecting to home.");

                web_sys::window().map(|win| {
                    win.location()
                        .set_href("/")
                        .unwrap_or_else(|e| log::error!("redirect error: {:?}", e))
                });
                return false;
            }
            Msg::FetchCompleted(tasks) => {
                self.title = tasks.project_name;
                self.tasks = Some(tasks.tasks);
                self.fetch_status = FetchStatus::Completed;
            }
            Msg::FetchFailed => {
                log::warn!("{:?}", Msg::FetchFailed);
                self.fetch_status = FetchStatus::Failed;
            }
            Msg::ToggleShowCompleted => self.show_completed = !self.show_completed,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        log::warn!("Not re-rendering on project change");
        false
    }

    fn rendered(&mut self, first_render: bool) {
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

        let task_list = match self.tasks.clone().map(|mut t| {
            // Display new tasks first
            t.sort_unstable_by_key(|t| Reverse(t.number));

            // Filter out completed when applicable
            t.into_iter().filter_map(|t| match self.show_completed {
                true => Some(t),
                false => match t.status {
                    Status::Done => None,
                    _ => Some(t),
                },
            })
        }) {
            // Convert to HTML
            Some(tasks) => html! {
                <ul>
                {tasks.map(|t| to_taskbox(&t)).collect::<Html>()}
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
            <h3>{ &format!("Taskboard for {}", self.title) }</h3>
            <div class="error">{ status_message }</div>
            <button id="newtask-btn" onclick=self.link.callback(|_| Msg::Add)>{ "new"} </button>
            <div>
                <label for="show-completed">{ "Show completed" }</label>
                <input
                    type="checkbox"
                    id="show-completed"
                    name="show completed tasks"
                    checked=self.show_completed
                    onchange=self.link.callback(|_| Msg::ToggleShowCompleted)
                />
            </div>
            {task_list}
            <button class="bg-danger" onclick=self.link.callback(|_| Msg::Delete)>{ "delete project" } </button>
            </>
        }
    }

    fn destroy(&mut self) {}
}
