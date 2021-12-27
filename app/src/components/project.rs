use std::cmp::Reverse;

use anyhow::anyhow;
use taskboard_core_lib::{uuid::Uuid, Status, Task};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::taskbox::TaskBox;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

//pub struct Project {
//    link: ComponentLink<Self>,
//    id: Uuid,
//    title: String,
//    tasks: Option<Vec<Task>>,
//    ft: Option<FetchTask>,
//    fetch_status: FetchStatus,
//    error: Option<String>,
//    show_completed: bool,
//}

#[derive(Debug, Clone, PartialEq)]
enum FetchStatus {
    Loading,
    Completed,
    Failed,
}

//#[derive(Debug)]
//pub enum Msg {
//    Add,
//    Added(Task),
//    Update(Task),
//    Updated(Task),
//    Delete,
//    Deleted,
//    FetchTasksCompleted(ProjectTasks),
//    FetchTasksFailed,
//    SetError(String),
//    ToggleShowCompleted,
//}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectProps {
    pub id: Uuid,
}

fn fetch_tasks() {
    let fetch_status = FetchStatus::Loading;

    //match TASK_SERVICE_URL {
    //    Some(url) => {
    //        let req = Request::get(&format!("{}/project-tasks/{}", url, self.id))
    //            .body(Nothing)
    //            .unwrap();

    //        let callback = self.link.callback(
    //            |res: Response<Json<Result<ProjectTasks, anyhow::Error>>>| {
    //                if let (meta, Json(Ok(body))) = res.into_parts() {
    //                    if meta.status.is_success() {
    //                        return Msg::FetchTasksCompleted(body);
    //                    }
    //                }
    //                Msg::FetchTasksFailed
    //            },
    //        );

    //        self.ft = FetchService::fetch(req, callback).ok();
    //    }
    //    None => log::error!("Unable to fetch tasks because the URL is not set"),
    //}
}

fn add_task() -> Result<(), JsValue> {
    // let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window avilable"))?;
    // let title = window
    //     .prompt_with_message("Enter task name")?
    //     .ok_or("No task name specified")?;
    // let estimate: Option<u8> =
    //     match window.prompt_with_message("Enter estimate (or leave blank)")? {
    //         Some(est) => {
    //             if est.is_empty() {
    //                 None
    //             } else {
    //                 Some(
    //                     est.parse::<u8>()
    //                         .map_err(|err| JsValue::from(err.to_string()))?,
    //                 )
    //             }
    //         }
    //         None => None,
    //     };

    // let command = CreateTaskCommand {
    //     project_id: self.id,
    //     title,
    //     estimate,
    // };

    // let req = Request::post(&format!("{}/task/create", TASK_SERVICE_URL.unwrap()))
    //     .header("Content-Type", "application/json")
    //     .body(Json(&command))
    //     .map_err(|_| JsValue::from("Failed to build post request"))?;

    // let callback = self
    //     .link
    //     .callback(|res: Response<Json<Result<Task, anyhow::Error>>>| {
    //         if let (meta, Json(Ok(body))) = res.into_parts() {
    //             return match meta.status.is_success() {
    //                 true => Msg::Added(body),
    //                 false => Msg::SetError(format!("Add task failed with {:?}", meta.status)),
    //             };
    //         }
    //         Msg::SetError("An error occured when adding task".to_owned())
    //     });

    // self.ft = FetchService::fetch(req, callback).ok();

    Ok(())
}

fn update_task(task: Task) -> Result<(), anyhow::Error> {
    // log::info!("Updating task {}...", task.number);

    // let command = UpdateTaskCommand {
    //     project_id: self.id,
    //     updated_task: task.clone(),
    // };

    // let req = Request::put(&format!("{}/task/update", TASK_SERVICE_URL.unwrap()))
    //     .header("Content-Type", "application/json")
    //     .body(Json(&command))?;

    // let callback = self
    //     .link
    //     .callback(move |res: Response<Result<String, anyhow::Error>>| {
    //         match res.status().is_success() {
    //             true => Msg::Updated(task.clone()),
    //             false => Msg::SetError(format!("Update task failed due to {:?}", res.status())),
    //         }
    //     });

    // self.ft = FetchService::fetch(req, callback).ok();
    Ok(())
}

fn delete_project() -> Result<(), anyhow::Error> {
    let window = web_sys::window().ok_or_else(|| anyhow!("Window was None"))?;

    // match window.confirm() {
    //     Ok(conf) if conf => {
    //         log::info!("Deleting project {}", self.title);
    //         let req = Request::delete(&format!("{}/{}", PROJECT_SERVICE_URL.unwrap(), self.id))
    //             .body(Nothing)?;

    //         let callback = self
    //             .link
    //             .callback(|res: Response<Result<String, anyhow::Error>>| {
    //                 match res.status().is_success() {
    //                     true => Msg::Deleted,
    //                     false => Msg::SetError(format!(
    //                         "Delete project failed due to {:?}",
    //                         res.status()
    //                     )),
    //                 }
    //             });

    //         self.ft = FetchService::fetch(req, callback).ok();
    //     }
    //     Ok(_) => log::info!("Delete project aborted"),
    //     Err(e) => return Err(anyhow!(format!("{:?}", e))),
    // }
    Ok(())
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    let title = use_state(|| props.id.to_string());
    let tasks: UseStateHandle<Option<Vec<Task>>> = use_state(|| None);
    let show_completed = use_state(|| false);
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let handle_task_create = Callback::from(|_| {
        log::warn!("Should create a new task");
    });

    let handle_task_update = Callback::from(|_t: Task| {
        log::warn!("Should update the task but don't know how to yet");
    });

    let handle_project_delete = Callback::from(|_| {
        log::warn!("Should delete project");
    });

    let to_taskbox = |model: Task| {
        html! {
            <TaskBox onchange={&handle_task_update} data={model.clone()} />
        }
    };

    let task_list = match tasks.as_ref().map(|t| {
        // Display new tasks first
        let mut t = t.to_vec();
        t.sort_unstable_by_key(|t| Reverse(t.number));

        // Filter out completed when applicable
        t.into_iter().filter(|t| {
            if *show_completed {
                true
            } else {
                !matches!(t.status, Status::Done)
            }
        })
    }) {
        // Convert to HTML
        Some(tasks) => html! {
            <ul>
            {tasks.map(to_taskbox).collect::<Html>()}
            </ul>
        },
        None => html! {
            <p>{ "Loading tasks..." }</p>
        },
    };

    let error_box = match &*error {
        Some(e) => html! { <div class="error"> { e } </div> },
        None => html! { <> </> },
    };

    use_effect(move || {
        // TODO: fetch tasks, fetch title
        tasks.set(Some(vec![Task::new(1, "mocktask")]));
        || {}
    });

    html! {
        <>
        <h3>{ &format!("Taskboard for {}", *title) }</h3>
        {error_box}
        <button id="newtask-btn" onclick={handle_task_create}>{ "new"} </button>
        <div>
            <label for="show-completed">{ "Show completed" }</label>
            <input
                type="checkbox"
                id="show-completed"
                name="show completed tasks"
                checked={*show_completed}
                onchange={move |_| show_completed.set(*show_completed)}
            />
        </div>
        {task_list}
        <button class="bg-danger" onclick={handle_project_delete}>{ "delete project" } </button>
        </>
    }
}
