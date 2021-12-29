use anyhow::anyhow;
use taskboard_core_lib::{commands::CreateProjectCommand, Project};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[function_component(CreateProject)]
pub fn create_project() -> Html {
    let name = use_state(|| String::from(""));
    let created: UseStateHandle<Option<Project>> = use_state(|| None);

    let handle_input = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let handle_submit = {
        let name = name.clone();
        let created = created.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let name = name.clone();
            let created = created.clone();
            spawn_local(async move {
                let res = create_the_project(name.as_ref()).await;
                match res {
                    Ok(res) => created.set(Some(res)),
                    Err(e) => {
                        log::error!("{:?}", e);
                        created.set(None);
                    }
                }
            });
        })
    };

    let created_html = match &*created {
        Some(p) => html! {
            <Link<AppRoute> to={AppRoute::Project { id: p.id }}>{ &format!("Project {} created successfully", p.name) }</Link<AppRoute>>
        },
        None => html! {},
    };

    html! {
        <>
        <h3>{ "Create a new project" }</h3>
        <form onsubmit={handle_submit}>
            <label for="create-project-name-field">{ "Project name" }</label>
            <input required={true} type="text" id="create-project-name-field" name="name" value={(*name).clone()} onchange={handle_input} />
            <input type="submit" />
        </form>
        {created_html}
        </>
    }
}

async fn create_the_project(name: &str) -> Result<Project, anyhow::Error> {
    let client = reqwest::Client::new();
    let command = CreateProjectCommand { name: name.into() };

    let created = client
        .post(PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL missing"))?)
        .json(&command)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(created)
}
