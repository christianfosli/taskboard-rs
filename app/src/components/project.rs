use std::cmp::Reverse;

use anyhow::anyhow;
use gloo_dialogs::{confirm, prompt};
use taskboard_core_lib::commands::{CreateTaskCommand, UpdateTaskCommand};
use taskboard_core_lib::{uuid::Uuid, ProjectTasks, Status, Task};
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
    let tasks: UseStateHandle<Vec<Task>> = use_state(Vec::new);
    let fetch_status = use_state(|| FetchStatus::Loading);
    let show_completed = use_state(|| false);
    let is_deleted = use_state(|| false);

    let handle_task_create = {
        let project_id = props.id;
        let tasks = tasks.clone();

        move |_| {
            let project_id = project_id;
            let tasks = tasks.clone();
            spawn_local(async move {
                match add_task(&project_id).await {
                    Ok(task) => {
                        log::info!("Task {} created", task.number);
                        let mut t = (*tasks).clone();
                        t.push(task);
                        tasks.set(t);
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                }
            });
        }
    };

    let handle_task_update = {
        let project_id = props.id;
        let tasks = tasks.clone();

        Callback::from(move |updated: Task| {
            let tasks = tasks.clone();
            spawn_local(async move {
                match update_task(&project_id, &updated).await {
                    Ok(updated) => {
                        log::info!("Task {} updated", updated.number);
                        let updated_tasks: Vec<Task> = (*tasks)
                            .iter()
                            .map(|t| {
                                if t.number == updated.number {
                                    updated.clone()
                                } else {
                                    t.clone()
                                }
                            })
                            .collect();
                        tasks.set(updated_tasks);
                    }
                    Err(e) => log::error!("{:?}", e),
                };
            });
        })
    };

    let handle_project_delete = {
        let id = props.id;
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
                log::debug!("Delete project aborted");
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
                <TaskBox onchange={&handle_task_update} data={t} />
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
        use_effect_with_deps(
            move |project_id| {
                let project_id = *project_id;
                let title = title.clone();
                let fetch_status = fetch_status.clone();
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
    .error_for_status()?
    .json()
    .await?;

    Ok(tasks)
}

async fn delete_project(id: &Uuid) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    client
        .delete(format!(
            "{}/{}",
            PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL missing"))?,
            id
        ))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

async fn add_task(project_id: &Uuid) -> Result<Task, anyhow::Error> {
    let title = prompt("Enter task name", None).ok_or_else(|| anyhow!("No task name specified"))?;

    let estimate = match prompt("Enter estimate", None) {
        Some(est) if est.is_empty() => None,
        Some(est) => Some(est.parse::<u8>()?),
        None => None,
    };

    let command = CreateTaskCommand {
        project_id: *project_id,
        title,
        estimate,
    };

    let client = reqwest::Client::new();

    let added = client
        .post(format!(
            "{}/task/create",
            TASK_SERVICE_URL.ok_or_else(|| anyhow!("TASK_SERVICE_URL missing"))?,
        ))
        .json(&command)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(added)
}

async fn update_task(project_id: &Uuid, task: &Task) -> Result<Task, anyhow::Error> {
    let command = UpdateTaskCommand {
        project_id: *project_id,
        updated_task: task.clone(),
    };

    let client = reqwest::Client::new();

    client
        .put(format!(
            "{}/task/update",
            TASK_SERVICE_URL.ok_or_else(|| anyhow!("TASK_SERVICE_URL missing"))?
        ))
        .json(&command)
        .send()
        .await?
        .error_for_status()?;

    Ok(task.clone())
}
