use gloo_dialogs::prompt;
use taskboard_core_lib::{Status, Task};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TaskBoxProps {
    pub onchange: Callback<Task>,
    pub on_err: Callback<Option<String>>,
    pub data: Task,
}

#[function_component(TaskBox)]
pub fn taskbox(props: &TaskBoxProps) -> Html {
    let rem_work = match &props.data.remaining_work {
        Some(hours) => format!("rem: {} hrs", hours),
        None => String::from("rem: ?"),
    };

    let handle_status_change = {
        let onchange = props.onchange.clone();
        let data = props.data.clone();

        move |status: Status| {
            let remaining_work = match status {
                Status::Done => Some(0),
                Status::Doing if data.status == Status::Done => None,
                _ => data.remaining_work,
            };

            onchange.emit(Task {
                status,
                remaining_work,
                ..data
            });
        }
    };

    let handle_title_change = {
        let onchange = props.onchange.clone();
        let data = props.data.clone();

        move |_| {
            if let Some(title) = prompt("Enter task name", None) {
                onchange.emit(Task { title, ..data });
            } else {
                log::warn!("Not changing title. Operation canceled");
            }
        }
    };

    let handle_rem_change = {
        let onchange = props.onchange.clone();
        let on_err = props.on_err.clone();
        let data = props.data.clone();
        move |_| {
            if let Some(rem) = prompt("Enter remaining work", None) {
                let rem = rem.parse::<u8>();
                match rem {
                    Ok(rem) => onchange.emit(Task {
                        remaining_work: Some(rem),
                        ..data.clone()
                    }),
                    Err(e) => {
                        log::error!("Error changing rem: {}", e);
                        on_err.emit(Some(format!("Could not update rem due to: {}", e)));
                    }
                }
            }
        }
    };

    let action = {
        let status = props.data.status;
        let handle_status_change_2 = handle_status_change.clone();
        match status {
            Status::Todo => html! {
                <button onclick={move |_| handle_status_change.clone()(Status::Doing)}>{ "Do -->" }</button>
            },
            Status::Doing => html! {
                <>
                <button onclick={move |_| {handle_status_change.clone()(Status::Todo)}}>{ "<-- Not doing" }</button>
                <button onclick={move |_| {handle_status_change_2.clone()(Status::Done)}}>{ "Done -->" }</button>
                </>
            },
            Status::Done => html! {
                <button onclick={move |_| {handle_status_change.clone()(Status::Doing)}}>{ "<-- Not done" }</button>
            },
        }
    };

    html! {

        <li class={format!("taskbox {:?}", props.data.status)}>
            <h3>{ &props.data.title } </h3>
            <p class="status">{ format!("status: {:?}", props.data.status) }</p>
            <p>{rem_work}  </p>
            <div>
                <button onclick={handle_title_change}>{ "Edit title" }</button>
                <button disabled={props.data.status==Status::Done} onclick={handle_rem_change}>{ "Update rem" }</button>
            </div>
            <div>
                {action}
            </div>
        </li>
    }
}
