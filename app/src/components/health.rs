use anyhow::anyhow;
use yew::{
    format::Nothing,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink,
};

const TASK_SERVICE_URL: Option<&'static str> = option_env!("TASK_SERVICE_URL");
const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

pub struct Health {
    link: ComponentLink<Self>,
    ping_task_svc: Option<FetchTask>,
    task_svc_status: Option<(Status, String)>,
    ping_project_svc: Option<FetchTask>,
    project_svc_status: Option<(Status, String)>,
}

pub enum Msg {
    PongTaskSvc(Status, String),
    PongProjectSvc(Status, String),
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Up,
    Down,
}

impl Health {
    fn ping_task_svc(&mut self) -> Result<(), anyhow::Error> {
        let req = Request::get(&format!(
            "{}/healthz",
            TASK_SERVICE_URL.ok_or_else(|| anyhow!("missing task service url"))?
        ))
        .body(Nothing)?;

        let callback = self
            .link
            .callback(|res: Response<Result<String, anyhow::Error>>| {
                if let (meta, Ok(body)) = res.into_parts() {
                    match meta.status.is_success() {
                        true => Msg::PongTaskSvc(Status::Up, body),
                        false => Msg::PongTaskSvc(Status::Down, body),
                    }
                } else {
                    log::warn!("Failed to contact task service");
                    Msg::PongTaskSvc(Status::Down, String::from("not reachable"))
                }
            });

        self.ping_task_svc = FetchService::fetch(req, callback).ok();

        Ok(())
    }

    fn ping_project_svc(&mut self) -> Result<(), anyhow::Error> {
        let req = Request::get(&format!(
            "{}/healthz",
            PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("missing task service url"))?
        ))
        .body(Nothing)?;

        let callback = self
            .link
            .callback(|res: Response<Result<String, anyhow::Error>>| {
                if let (meta, Ok(body)) = res.into_parts() {
                    match meta.status.is_success() {
                        true => Msg::PongProjectSvc(Status::Up, body),
                        false => Msg::PongProjectSvc(Status::Down, body),
                    }
                } else {
                    log::warn!("Failed to contact project service");
                    Msg::PongProjectSvc(Status::Down, String::from("not reachable"))
                }
            });

        self.ping_project_svc = FetchService::fetch(req, callback).ok();

        Ok(())
    }
}

impl Component for Health {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            ping_task_svc: None,
            task_svc_status: None,
            ping_project_svc: None,
            project_svc_status: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::PongTaskSvc(status, message) => self.task_svc_status = Some((status, message)),
            Msg::PongProjectSvc(status, message) => {
                self.project_svc_status = Some((status, message))
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.ping_task_svc()
                .unwrap_or_else(|e| log::error!("Error pinging task service: {}", e));

            self.ping_project_svc()
                .unwrap_or_else(|e| log::error!("Error pinging project service: {}", e));
        }
    }

    fn view(&self) -> yew::Html {
        let overall_status = match (&self.task_svc_status, &self.project_svc_status) {
            (None, _) | (_, None) => "Loading...",
            (Some((Status::Down, _)), _) | (_, Some((Status::Down, _))) => "Degraded",
            (Some((Status::Up, _)), Some((Status::Up, _))) => "OK",
        };

        let task_service = match &self.task_svc_status {
            Some((_, msg)) => msg,
            None => "loading...",
        };

        let project_service = match &self.project_svc_status {
            Some((_, msg)) => msg,
            None => "loading...",
        };

        html! {
            <>
            <h3>{ overall_status }</h3>
            <p>{ "App OK" }</p>
            <p>{ "Task service " }{ task_service }</p>
            <p>{ "Project service " }{ project_service }</p>
            <br/>
            <p>
            { "More details are available " }
            <a href="metrics.taskboard.cloud">{ "here" }</a>
            { "."}
            </p>
            </>
        }
    }
}
