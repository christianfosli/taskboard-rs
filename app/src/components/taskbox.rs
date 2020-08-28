use yew::prelude::*;

pub struct TaskBox {
    link: ComponentLink<Self>,
    onChange: Callback<Task>,
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
            onChange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StatusChanged(status) => self.onChange.emit(Task {
                status,
                ..self.data.clone()
            }),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.data = props.data;
        self.onChange = props.onchange;
        true
    }

    fn view(&self) -> Html {
        let remWork = match self.data.remaining_work {
            Some(hours) => format!("rem: {} hrs", hours),
            None => String::from(""),
        };
        html! {
            <li class="todo">
                <h3>{ &self.data.title }</h3>
                <p class="status">{ format!("status: {:?}", self.data.status) }</p>
                <p>{remWork}</p>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Todo))>{ "Todo" }</button>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Doing))>{ "Do" }</button>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Done))>{ "Done" }</button>
            </li>
        }
    }
}
