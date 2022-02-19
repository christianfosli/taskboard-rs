use anyhow::anyhow;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[derive(Debug, Clone)]
pub enum Status {
    Up(String),
    Down,
    Unkown,
}

#[function_component(Health)]
pub fn health() -> Html {
    let task_svc = use_state(|| Status::Unkown);
    let project_svc = use_state(|| Status::Unkown);

    let overall_status = match ((*task_svc).clone(), (*project_svc).clone()) {
        (Status::Up(_), Status::Up(_)) => "OK".to_string(),
        (Status::Down, _) | (_, Status::Down) => "Degraded".to_string(),
        _ => "Loading...".to_string(),
    };

    {
        let task_svc = task_svc.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match ping_task_svc().await {
                        Ok(res) => task_svc.set(Status::Up(res)),
                        Err(e) => {
                            log::error!("{:?}", e);
                            task_svc.set(Status::Down);
                        }
                    }
                });
                || {}
            },
            (),
        );
    }

    {
        let project_svc = project_svc.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match ping_project_svc().await {
                        Ok(res) => project_svc.set(Status::Up(res)),
                        Err(e) => {
                            log::error!("{:?}", e);
                            project_svc.set(Status::Down);
                        }
                    }
                });
                || {}
            },
            (),
        );
    }

    html! {
        <>
        <h3>{ overall_status }</h3>
        <p>{ "App OK" }</p>
        <p>{ "Task service " }{ format!("{:?}",*task_svc) }</p>
        <p>{ "Project service " }{ format!("{:?}", *project_svc) }</p>
        <br/>
        <p>
        { "More metrics on " }
        <a href="https://metrics.taskboard.cloud">{ "metrics.taskboard.cloud" }</a>
        { ". Logs on https://logs.taskboard.cloud" }
        <a href="https://logs.taskboard.cloud">{ "logs.taskboard.cloud" }</a>
        { "." }
        </p>
        </>
    }
}

async fn ping_task_svc() -> Result<String, anyhow::Error> {
    let res = reqwest::get(format!(
        "{}/readyz",
        TASK_SERVICE_URL.ok_or_else(|| anyhow!("TASK_SERVICE_URL missing"))?
    ))
    .await?
    .error_for_status()?
    .text()
    .await?;

    Ok(res)
}

async fn ping_project_svc() -> Result<String, anyhow::Error> {
    let res = reqwest::get(format!(
        "{}/readyz",
        PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL missing"))?
    ))
    .await?
    .error_for_status()?
    .text()
    .await?;

    Ok(res)
}
