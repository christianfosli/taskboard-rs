#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

// models (should probably be moved into core)

struct Project {
    link: ComponentLink<Self>,
    id: String,
    title: String,
}

struct Task {
    link: ComponentLink<Self>,
    title: String,
    description: String,
    status: Status,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Todo,
    Doing,
    Done,
}

// controller

enum Msg {
    AddTodo,
    RemoveTodo,
}

// view

impl Component for Project {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: String::from("tmp-id"),
            title: String::from("tmp"),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <h1>{ "Task Board" }</h1>
                <ul>
                    <Task:/>
                    <Task:/>
                </ul>
            </main>
        }
    }
}

impl Component for Task {
    type Message = Status;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            title: String::from("My todo"),
            description: String::from(""),
            status: Status::Todo,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let initialStatus = self.status;
        match msg {
            Status::Todo => self.status = Status::Todo,
            Status::Doing => self.status = Status::Doing,
            Status::Done => self.status = Status::Done,
        }
        self.status != initialStatus
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="todo">
                <h3>{ &self.title }</h3>
                <p class="status">{ format!("status: {:?}", self.status) }</p>
                <button onclick=self.link.callback(|_| Status::Todo)>{ "Todo" }</button>
                <button onclick=self.link.callback(|_| Status::Doing)>{ "Do" }</button>
                <button onclick=self.link.callback(|_| Status::Done)>{ "Done" }</button>
                <p>{ &self.description }</p>
            </li>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::<Project>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
