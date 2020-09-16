use crate::components::taskbox::TaskBox;
use taskboard_core_lib::{ProjectTasks, Status, Task};
use yew::{
    format::Json,
    format::Nothing,
    prelude::*,
    services::fetch::Request,
    services::fetch::Response,
    services::{ConsoleService, FetchService},
};

const API_URL: Option<&'static str> = option_env!("API_URL");

pub struct Model {
    link: ComponentLink<Self>,
    project_id: String,
    project_title: String,
    tasks: Vec<Task>,
    loading: bool,
}

pub enum Msg {
    Add,
    Update(Task),
    Delete(Task),
    FetchCompleted(ProjectTasks),
    FetchFailed,
}

impl Model {
    fn fetch_tasks(&mut self) {
        ConsoleService::log("fetching tasks...");
        self.loading = true;
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

                let task = FetchService::fetch(req, callback);
            }
            None => ConsoleService::error("Unable to fetch tasks because API_URL is not set"),
        }
        self.loading = false;
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
            tasks: Vec::new(),
            loading: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => ConsoleService::log("should be adding task"),
            Msg::Update(task) => ConsoleService::log(&format!("Should be updating {}", task.title)),
            Msg::Delete(task) => ConsoleService::log(&format!("Should be deleting {}", task.title)),
            Msg::FetchCompleted(tasks) => ConsoleService::log(&format!("Fetched: {:?}", tasks)),
            Msg::FetchFailed => ConsoleService::error("Failed to fetch )-:"),
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::log("App changed");
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        ConsoleService::log("App rendered");
        self.fetch_tasks();
    }

    fn view(&self) -> Html {
        let dummy_task = Task {
            title: String::from("Dummy"),
            status: Status::Todo,
            remaining_work: Some(5),
        };
        html! {
            <main>
                <h1>{ &format!("Project {} - Board", self.project_title) }</h1>
                <ul>
                    <TaskBox onchange=self.link.callback(|task| Msg::Update(task)) data=&dummy_task/>
                </ul>
            </main>
        }
    }

    fn destroy(&mut self) {}
}
