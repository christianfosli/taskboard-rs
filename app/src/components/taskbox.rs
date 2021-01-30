use taskboard_core_lib::{Status, Task};
use wasm_bindgen::JsValue;
use yew::{prelude::*, web_sys};

pub struct TaskBox {
    link: ComponentLink<Self>,
    onchange: Callback<Task>,
    data: Task,
    error: Option<String>,
}

pub enum Msg {
    ChangeTitle,
    StatusChanged(Status),
    ChangeRem,
    SetError(Option<String>),
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
            onchange: props.onchange,
            data: props.data,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeTitle => {
                let window = web_sys::window().expect("No window available");
                let title = window.prompt_with_message("Enter task name").ok().flatten();
                match title {
                    Some(title) => self.onchange.emit(Task {
                        title,
                        ..self.data.clone()
                    }),
                    None => log::warn!(
                        "Not changing title for task {}. Operation failed or was cancelled.",
                        self.data.number
                    ),
                }
            }
            Msg::StatusChanged(status) => {
                let remaining_work = match status {
                    Status::Done => Some(0),
                    Status::Doing if self.data.status == Status::Done => None,
                    _ => self.data.remaining_work,
                };

                self.onchange.emit(Task {
                    status,
                    remaining_work,
                    ..self.data.clone()
                })
            }
            Msg::ChangeRem => {
                let window = web_sys::window().expect("No window available");

                let new_rem = window
                    .prompt_with_message("Enter remaining work")
                    .and_then(|rem: Option<String>| rem.ok_or(JsValue::from("No value provided")))
                    .and_then(|rem: String| {
                        rem.parse::<u8>()
                            .map_err(|err| JsValue::from(err.to_string()))
                    });

                match new_rem {
                    Ok(rem) => self.onchange.emit(Task {
                        remaining_work: Some(rem),
                        ..self.data.clone()
                    }),
                    Err(e) => self.link.send_message(Msg::SetError(e.as_string())),
                }
            }
            Msg::SetError(error) => {
                match &error {
                    Some(e) => log::error!("{}", e),
                    None => (),
                }
                self.error = error;
                return true;
            }
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
            None => String::from("rem: ?"),
        };

        let action = match self.data.status {
            Status::Todo => html! {
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Doing))>{ "Do -->" }</button>
            },
            Status::Doing => html! {
                <>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Todo))>{ "<-- Not doing" }</button>
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Done))>{ "Done -->" }</button>
                </>
            },
            Status::Done => html! {
                <button onclick=self.link.callback(|_| Msg::StatusChanged(Status::Doing))>{ "<-- Not done" }</button>
            },
        };

        html! {
            <li class="todo">
                <h3>{ &self.data.title } </h3>
                <p class="status">{ format!("status: {:?}", self.data.status) }</p>
                <p>{rem_work}  </p>
                <div>
                    <button onclick=self.link.callback(|_| Msg::ChangeTitle)>{ "Edit title" }</button>
                    <button disabled=self.data.status==Status::Done onclick=self.link.callback(|_| Msg::ChangeRem)>{ "Update rem" }</button>
                </div>
                <div>
                    {action}
                </div>
                <p class="error">{
                    match &self.error {
                        Some(e) => e,
                        None => ""
                    }
                }</p>
            </li>
        }
    }
}
