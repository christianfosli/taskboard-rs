use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ErrorProps {
    pub err: Option<String>,
    pub set_err: Callback<Option<String>>,
}

#[function_component(ErrorBox)]
pub fn error_comp(props: &ErrorProps) -> Html {
    if props.err.is_some() {
        // Unset the error after the user has likely finished reading it
        let set_err = props.set_err.clone();
        use_effect(move || {
            let handle = Timeout::new(10_000, move || set_err.emit(None));
            || {
                handle.cancel();
            }
        });
    }

    if let Some(error) = &props.err {
        html! {
        <div class="error">
            <h4>{ "An error occured" }</h4>
            <p> { error } </p>
        </div>
        }
    } else {
        html! {}
    }
}
