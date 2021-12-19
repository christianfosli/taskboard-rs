use yew::prelude::*;

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Up,
    Down,
}

//fn ping_task_svc() -> Result<(), anyhow::Error> {
//    let req = Request::get(&format!(
//        "{}/readyz",
//        TASK_SERVICE_URL.ok_or_else(|| anyhow!("missing task service url"))?
//    ))
//    .body(Nothing)?;
//
//    let callback = self
//        .link
//        .callback(|res: Response<Result<String, anyhow::Error>>| {
//            if let (meta, Ok(body)) = res.into_parts() {
//                match meta.status.is_success() {
//                    true => Msg::PongTaskSvc(Status::Up, body),
//                    false => Msg::PongTaskSvc(Status::Down, body),
//                }
//            } else {
//                log::warn!("Failed to contact task service");
//                Msg::PongTaskSvc(Status::Down, String::from("not reachable"))
//            }
//        });
//
//    self.ping_task_svc = FetchService::fetch(req, callback).ok();
//
//    Ok(())
//}
//
//fn ping_project_svc(&mut self) -> Result<(), anyhow::Error> {
//    let req = Request::get(&format!(
//        "{}/readyz",
//        PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("missing project service url"))?
//    ))
//    .body(Nothing)?;
//
//    let callback = self
//        .link
//        .callback(|res: Response<Result<String, anyhow::Error>>| {
//            if let (meta, Ok(body)) = res.into_parts() {
//                match meta.status.is_success() {
//                    true => Msg::PongProjectSvc(Status::Up, body),
//                    false => Msg::PongProjectSvc(Status::Down, body),
//                }
//            } else {
//                log::warn!("Failed to contact project service");
//                Msg::PongProjectSvc(Status::Down, String::from("not reachable"))
//            }
//        });
//
//    self.ping_project_svc = FetchService::fetch(req, callback).ok();
//
//    Ok(())
//}

#[function_component(Health)]
pub fn health() -> Html {
    let overall_status = use_state(|| "Loading...");
    let task_svc = use_state(|| "loading...");
    let project_svc = use_state(|| "loading...");

    // TODO: Fetch status and set fields. use_effect + reqwest???

    //    let overall_status = match (&self.task_svc_status, &self.project_svc_status) {
    //        (None, _) | (_, None) => "Loading...",
    //        (Some((Status::Down, _)), _) | (_, Some((Status::Down, _))) => "Degraded",
    //        (Some((Status::Up, _)), Some((Status::Up, _))) => "OK",
    //    };

    // let task_service = match &self.task_svc_status {
    //     Some((_, msg)) => msg,
    //     None => "loading...",
    // };

    // let project_service = match &self.project_svc_status {
    //     Some((_, msg)) => msg,
    //     None => "loading...",
    // };

    html! {
        <>
        <h3>{ *overall_status }</h3>
        <p>{ "App OK" }</p>
        <p>{ "Task service " }{ *task_svc }</p>
        <p>{ "Project service " }{ *project_svc }</p>
        <br/>
        <p>
        { "More details are available " }
        <a href="https://metrics.taskboard.cloud">{ "here" }</a>
        { "."}
        </p>
        </>
    }
}
