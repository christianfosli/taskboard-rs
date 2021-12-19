use taskboard_core_lib::{Status, Task};
use wasm_bindgen::JsValue;
use yew::prelude::*;

//pub struct TaskBox {
//    error: Option<String>,
//}
//
//pub enum Msg {
//    ChangeTitle,
//    StatusChanged(Status),
//    ChangeRem,
//    SetError(Option<String>),
//}

#[derive(Clone, PartialEq, Properties)]
pub struct TaskBoxProps {
    pub onchange: Callback<Task>,
    pub data: Task,
}

#[function_component(TaskBox)]
pub fn taskbox(props: &TaskBoxProps) -> Html {
    let rem_work = match props.data.remaining_work {
        Some(hours) => format!("rem: {} hrs", hours),
        None => String::from("rem: ?"),
    };

    let error = use_state(|| None);

    let window = web_sys::window().expect("No window available");

    let handle_status_change = |status: Status| {
        let remaining_work = match status {
            Status::Done => Some(0),
            Status::Doing if props.data.status == Status::Done => None,
            _ => props.data.remaining_work,
        };

        props.onchange.emit(Task {
            status,
            remaining_work,
            ..props.data.clone()
        })
    };

    let handle_title_change = |_| {
        let title = window.prompt_with_message("Enter task name").ok().flatten();
        match title {
            Some(title) => props.onchange.emit(Task {
                title,
                ..props.data.clone()
            }),
            None => log::warn!(
                "Not changing title for task {}. Operation failed or was cancelled.",
                props.data.number
            ),
        }
    };

    let handle_rem_change = |_| {
        let new_rem = window
            .prompt_with_message("Enter remaining work")
            .and_then(|rem: Option<String>| rem.ok_or_else(|| JsValue::from("No value provided")))
            .and_then(|rem: String| {
                rem.parse::<u8>()
                    .map_err(|err| JsValue::from(err.to_string()))
            });

        match new_rem {
            Ok(rem) => props.onchange.emit(Task {
                remaining_work: Some(rem),
                ..props.data.clone()
            }),
            Err(e) => *error = e.as_string(),
        }
    };

    let action = match props.data.status {
        Status::Todo => html! {
            <button onclick={|_| {handle_status_change(Status::Doing)}}>{ "Do -->" }</button>
        },
        Status::Doing => html! {
            <>
            <button onclick={|_| {handle_status_change(Status::Todo)}}>{ "<-- Not doing" }</button>
            <button onclick={|_| {handle_status_change(Status::Done)}}>{ "Done -->" }</button>
            </>
        },
        Status::Done => html! {
            <button onclick={|_| {handle_status_change(Status::Doing)}}>{ "<-- Not done" }</button>
        },
    };

    let data = props.data;

    html! {

        <li class={format!("taskbox {:?}", data.status)}>
            <h3>{ data.title } </h3>
            <p class="status">{ format!("status: {:?}", data.status) }</p>
            <p>{rem_work}  </p>
            <div>
                <button onclick={handle_title_change}>{ "Edit title" }</button>
                <button disabled={data.status==Status::Done} onclick={handle_rem_change}>{ "Update rem" }</button>
            </div>
            <div>
                {action}
            </div>
            <p class="error">{
                match *error {
                    Some(e) => &e,
                    None => ""
                }
            }</p>
        </li>
    }
}
