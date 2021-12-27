use gloo_dialogs::prompt;
use taskboard_core_lib::{Status, Task};
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
    let rem_work = match &props.data.remaining_work {
        Some(hours) => format!("rem: {} hrs", hours),
        None => String::from("rem: ?"),
    };

    let onchange = props.onchange.clone();
    let data = props.data.clone();

    let handle_status_change = move |status: Status| {
        let remaining_work = match status {
            Status::Done => Some(0),
            Status::Doing if data.status == Status::Done => None,
            _ => data.remaining_work,
        };

        onchange.emit(Task {
            status,
            remaining_work,
            ..data
        })
    };

    let onchange = props.onchange.clone();
    let data = props.data.clone();

    let handle_title_change = move |_| {
        let title = prompt("Enter task name", None);
        match title {
            Some(title) => onchange.emit(Task {
                title: title.to_string(),
                ..data
            }),
            None => log::warn!(
                "Not changing title for task {}. Operation failed or was cancelled.",
                data.number
            ),
        }
    };

    let onchange = props.onchange.clone();
    let data = props.data.clone();

    let handle_rem_change = move |_| {
        if let Some(rem) = prompt("Enter remaining work", None) {
            let rem = rem.parse::<u8>();
            match rem {
                Ok(rem) => onchange.emit(Task {
                    remaining_work: Some(rem),
                    ..data.clone()
                }),
                Err(e) => log::error!("Error changing rem: {}", e),
            }
        }
    };

    let action = html! { <button>{ "TODO: Implement buttons" }</button> };
    // let action = match data.status {
    //     Status::Todo => html! {
    //         <button onclick={move |_| handle_status_change(Status::Doing)}>{ "Do -->" }</button>
    //     },
    //     Status::Doing => html! {
    //         <>
    //         <button onclick={move |_| {handle_status_change(Status::Todo)}}>{ "<-- Not doing" }</button>
    //         <button onclick={move |_| {handle_status_change(Status::Done)}}>{ "Done -->" }</button>
    //         </>
    //     },
    //     Status::Done => html! {
    //         <button onclick={move |_| {handle_status_change(Status::Doing)}}>{ "<-- Not done" }</button>
    //     },
    // };

    html! {

        <li class={format!("taskbox {:?}", props.data.status.clone())}>
            <h3>{ &props.data.title } </h3>
            <p class="status">{ format!("status: {:?}", props.data.status.clone()) }</p>
            <p>{rem_work}  </p>
            <div>
                <button onclick={handle_title_change}>{ "Edit title" }</button>
                <button disabled={props.data.status.clone()==Status::Done} onclick={handle_rem_change}>{ "Update rem" }</button>
            </div>
            <div>
                {action}
            </div>
        </li>
    }
}
