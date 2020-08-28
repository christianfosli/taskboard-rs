use crate::components::taskbox::{Status, Task, TaskBox};
use yew::{
    prelude::*,
    services::{ConsoleService, FetchService},
};

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
}

impl Model {
    fn fetch_tasks(&mut self) {
        ConsoleService::log("fetching tasks...");
        self.loading = true;
        // todo: fetch data
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
