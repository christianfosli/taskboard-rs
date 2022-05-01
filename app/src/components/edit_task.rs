use taskboard_core_lib::Task;
use web_sys::{HtmlDialogElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditTaskProps {
    pub on_submit: Callback<Task>,
    pub on_err: Callback<Option<String>>,
    pub data: Option<Task>,
}

#[function_component(EditTask)]
pub fn edit_task(props: &EditTaskProps) -> Html {
    log::info!("Editing {:?}", props.data);

    let dialog_ref = use_node_ref();

    let title = use_state(|| {
        props
            .data
            .clone()
            .map(|d| d.title)
            .unwrap_or_else(|| "".to_owned())
    });

    let rem = use_state(|| props.data.clone()?.remaining_work);

    let handle_title_input = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let handle_rem_input = {
        let rem = rem.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value().parse::<u8>();
            match val {
                Ok(val) => rem.set(Some(val)),
                Err(e) => {
                    log::error!("Error setting rem: {e}");
                    rem.set(None);
                }
            };
        })
    };

    let handle_submit = {
        let on_submit = props.on_submit.clone();
        let title = title.clone();
        let rem = rem.clone();
        let original_data = props.data.clone().unwrap_or_else(|| Task::new(0, "Dummy"));

        Callback::from(move |_: FocusEvent| {
            on_submit.emit(Task {
                title: title.to_string(),
                remaining_work: *rem,
                ..original_data
            });
        })
    };

    {
        let data = props.data.clone();
        let dialog_ref = dialog_ref.clone();
        use_effect(move || {
            let dialog = dialog_ref.cast::<HtmlDialogElement>().unwrap();
            if data.is_some() && !dialog.open() {
                dialog.show_modal().unwrap();
            } else if data.is_none() && dialog.open() {
                dialog.close();
            }
            move || dialog.close()
        });
    }

    let dialogue_title = format!(
        "Edit task {number:?} {title}",
        number = props.data.as_ref().map(|d| d.number).unwrap_or_else(|| 0),
        title = *title
    );

    let rem_val = match *rem {
        Some(rem) => format!("{rem}"),
        None => "".to_owned(),
    };

    html! {
        <dialog ref={dialog_ref}>
            <h3>{ dialogue_title }</h3>
            <form onsubmit={&handle_submit} method="dialogue">
                <label for="edit-task-title">{ "Title" }</label>
                <input required={true} type="text" id="edit-task-title" name="title" value={(*title).clone()} onchange={handle_title_input}/>
                <label for="edit-task-rem">{ "Remaining work" }</label>
                <input required={false} type="number" id="edit-task-rem" name="remaining_work" value={rem_val} onchange={handle_rem_input}/>
                <input type="submit" />
            </form>
        </dialog>
    }
}
