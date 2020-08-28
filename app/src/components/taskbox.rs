use taskboard_core_lib::{Status, Task};
use yew::prelude::*;

pub struct TaskBox {
    link: ComponentLink<Self>,
    onchange: Callback<Task>,
    data: Task,
}

pub enum Msg {
    StatusChanged(Status),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Task>,
    pub data: Task,
}

impl Component for TaskBox {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: props.data,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StatusChanged(status) => self.onchange.emit(Task {
                status,
                ..self.data.clone()
            }),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.data = props.data;
        self.onchange = props.onchange;
        true
    }

    fn view(&self) -> Html {
        let rem_work = match self.data.remaining_work {
            Some(hours) => format!("rem: {} hrs", hours),
            None => String::from(""),
        };
        html! {
            <li class="todo">
                <h3>{ &self.data.title }</h3>
                <p class="status">{ format!("status: {:?}", self.data.status) }</p>
                <p>{rem_work}</p>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Todo))>{ "Todo" }</button>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Doing))>{ "Do" }</button>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Done))>{ "Done" }</button>
            </li>
        }
    }
}
