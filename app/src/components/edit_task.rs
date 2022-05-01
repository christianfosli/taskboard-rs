use taskboard_core_lib::Task;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditTaskProps {
    pub on_submit: Callback<Task>,
    pub on_err: Callback<Option<String>>,
    pub data: Option<Task>,
}

#[function_component(EditTask)]
pub fn edit_task(props: &EditTaskProps) -> Html {
    let title = use_state(|| {
        props
            .data
            .clone()
            .map(|d| d.title)
            .unwrap_or_else(|| "".to_owned())
    });
    let rem = use_state(|| props.data.clone()?.remaining_work);

    let handle_submit = {
        let on_submit = props.on_submit.clone();
        let title = title.clone();
        let rem = rem.clone();
        let original_data = props.data.clone().unwrap_or_else(|| Task::new(0, "Dummy"));

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            on_submit.emit(Task {
                title: title.to_string(),
                remaining_work: *rem,
                ..original_data
            });
        })
    };

    let dialogue_title = format!(
        "Edit task {number:?} {title}",
        number = props.data.as_ref().map(|d| d.number).unwrap_or_else(|| 0),
        title = *title
    );

    html! {
        <dialogue open={props.data.is_some()}>
            <h3>{ dialogue_title }</h3>
            <form onsubmit={&handle_submit} method="dialogue">
                <label for="edit-task-title">{ "Title" }</label>
                <input required={true} type="text" id="edit-task-title" name="title" value={(*title).clone()}/>
                <label for="edit-task-rem">{ "Remaining work" }</label>
                <input required={false} type="number" id="edit-task-rem" name="remaining_work" value={format!("{:?}", *rem)}/>
            </form>
        </dialogue>
    }
}
