use crate::components::taskbox::TaskBox;
use taskboard_core_lib::{ProjectTasks, Task};
use yew::{
    format::Json,
    format::Nothing,
    prelude::*,
    services::fetch::FetchTask,
    services::fetch::Request,
    services::fetch::Response,
    services::{ConsoleService, FetchService},
};

const API_URL: Option<&'static str> = option_env!("API_URL");

pub struct Model {
    link: ComponentLink<Self>,
    project_id: String,
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
    Add(String),
    Update(Task),
    Delete(Task),
    FetchCompleted(ProjectTasks),
    FetchFailed,
}

impl Model {
    fn fetch_tasks(&mut self) {
        self.fetch_status = FetchStatus::Loading;

        match API_URL {
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
            None => ConsoleService::error("Unable to fetch tasks because API_URL is not set"),
        }
    }

    fn add_task(&mut self, title: &str) {
        ConsoleService::log(&format!("Should be adding task {}", title))
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            project_id: String::from("tmp-id"), // TODO - get from query param
            project_title: String::from("tmp"),
            tasks: None,
            ft: None,
            fetch_status: FetchStatus::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add(title) => self.add_task(&title),
            Msg::Update(task) => ConsoleService::log(&format!("Should be updating {}", task.title)),
            Msg::Delete(task) => ConsoleService::log(&format!("Should be deleting {}", task.title)),
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
            {task_list}
            </main>
        }
    }

    fn destroy(&mut self) {}
}
