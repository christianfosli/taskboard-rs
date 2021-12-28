use std::cmp::Reverse;

use anyhow::anyhow;
use gloo_dialogs::confirm;
use taskboard_core_lib::{uuid::Uuid, ProjectTasks, Status, Task};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::taskbox::TaskBox;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[derive(Debug, Clone, PartialEq)]
enum FetchStatus {
    Loading,
    Completed,
    Failed,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectProps {
    pub id: Uuid,
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    let title = use_state(|| props.id.to_string());
    let tasks: UseStateHandle<Vec<Task>> = use_state(|| vec![]);
    let fetch_status = use_state(|| FetchStatus::Loading);
    let show_completed = use_state(|| false);
    let is_deleted = use_state(|| false);

    let handle_task_create = Callback::from(|_| {
        log::warn!("Should create a new task");
    });

    let handle_task_update = Callback::from(|_t: Task| {
        log::warn!("Should update the task but don't know how to yet");
    });

    let handle_project_delete = {
        let id = props.id.clone();
        let is_deleted = is_deleted.clone();
        move |_| {
            let is_deleted = is_deleted.clone();
            if confirm("Are you sure you want to delete this project?") {
                spawn_local(async move {
                    match delete_project(&id).await {
                        Ok(_) => {
                            log::info!("Deleted project {:?}", &id);
                            is_deleted.set(true);
                        }
                        Err(e) => log::error!("{:?}", e),
                    }
                });
            } else {
                log::debug!("Delete project aborted")
            }
        }
    };

    let status_text = {
        if *fetch_status == FetchStatus::Loading {
            html! { <p>{ "Loading project tasks..." }</p> }
        } else if *fetch_status == FetchStatus::Failed {
            html! { <p class="error">{ "Project tasks could not be loaded 😭" }</p> }
        } else {
            html! {}
        }
    };

    let task_list = {
        let to_taskbox = |t: Task| {
            html! {
                <TaskBox onchange={&handle_task_update} data={t.clone()} />
            }
        };

        // Display new tasks first
        let mut tasks = (*tasks).clone();
        tasks.sort_unstable_by_key(|t| Reverse(t.number));

        tasks
            .into_iter()
            // Filter completed when applicable
            .filter(|t| {
                if *show_completed {
                    true
                } else {
                    !matches!(t.status, Status::Done)
                }
            })
            // Into html
            .map(to_taskbox)
            .collect::<Html>()
    };

    {
        let title = title.clone();
        let fetch_status = fetch_status.clone();
        use_effect_with_deps(
            move |project_id| {
                let project_id = (*project_id).clone();
                let title = title.clone();
                let fetch_status = fetch_status.clone();
                let tasks = tasks.clone();
                spawn_local(async move {
                    let res = fetch_tasks(&project_id).await;
                    match res {
                        Ok(res) => {
                            title.set(res.project_name);
                            tasks.set(res.tasks);
                            fetch_status.set(FetchStatus::Completed);
                        }
                        Err(e) => {
                            log::error!("{:?}", e);
                            fetch_status.set(FetchStatus::Failed);
                        }
                    }
                });
                || {}
            },
            props.id,
        );
    }

    if *is_deleted {
        log::info!("Redirecting home since project has been deleted");
        return html! {
            <Redirect<AppRoute> to={AppRoute::Home}/>
        };
    }

    html! {
        <>
        <h3>{ &format!("Taskboard for {}", *title) }</h3>
        <button id="newtask-btn" onclick={handle_task_create}>{ "new"} </button>
        <div>
            <label for="show-completed">{ "Show completed" }</label>
            <input
                type="checkbox"
                id="show-completed"
                name="show completed tasks"
                checked={*show_completed}
                onchange={move |_| show_completed.set(!*show_completed)}
            />
        </div>
        {status_text}
        {task_list}
        <button class="bg-danger" onclick={handle_project_delete}>{ "delete project" } </button>
        </>
    }
}

async fn fetch_tasks(project_id: &Uuid) -> Result<ProjectTasks, anyhow::Error> {
    let tasks = reqwest::get(format!(
        "{}/project-tasks/{}",
        TASK_SERVICE_URL.ok_or_else(|| anyhow!("TASK_SERVICE_URL not set"))?,
        project_id
    ))
    .await?
    .json()
    .await?;

    Ok(tasks)
}

async fn delete_project(id: &Uuid) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    let res = client
        .delete(format!(
            "{}/{}",
            PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL missing"))?,
            id
        ))
        .send()
        .await?;

    res.error_for_status()?;
    Ok(())
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
    Ok(())
}
