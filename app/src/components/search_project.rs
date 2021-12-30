use wasm_bindgen_futures::spawn_local;

use anyhow::anyhow;
use reqwest::Url;
use taskboard_core_lib::Project;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

#[derive(Clone, PartialEq, Properties)]
pub struct SearchProps {
    pub set_err: Callback<Option<String>>,
}

#[function_component(SearchProject)]
pub fn search_project(props: &SearchProps) -> Html {
    let query = use_state(|| String::from(""));
    let matches: UseStateHandle<Option<Vec<Project>>> = use_state(|| None);

    let handle_input = {
        let query = query.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            query.set(input.value());
        })
    };

    let handle_submit = {
        let query = query.clone();
        let matches = matches.clone();
        let set_err = props.set_err.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let query = query.clone();
            let matches = matches.clone();
            let set_err = set_err.clone();
            spawn_local(async move {
                let res = search(query.as_ref()).await;
                match res {
                    Ok(res) => matches.set(Some(res)),
                    Err(e) => {
                        log::error!("Error searching for projects: {}", e);
                        set_err.emit(Some(format!("Error searching for projects: {}", e)));
                    }
                };
            });
        })
    };

    let to_li = |p: &Project| {
        html! {
            <li>
                <Link<AppRoute> to={AppRoute::Project { id: p.id }}>
                  { &p.name }
                </Link<AppRoute>>
            </li>
        }
    };

    let matches_html = match matches.as_ref() {
        None => html! {},
        Some(m) if m.is_empty() => html! {<p>{ "No matches" }</p>},
        Some(m) => {
            let matches = m.iter().map(|p| to_li(p)).collect::<Html>();

            html! {
                <ul>
                {matches}
                </ul>
            }
        }
    };

    html! {
        <>
        <h3>{ "Search for an existing project" }</h3>
        <form onsubmit={handle_submit}>
            <label for="search-project-field">{ "Project name" }</label>
            <input type="text" id="search-project-field" name="query" value={(*query).clone()} onchange={handle_input} required={true}/>
            <input type="submit" value="Search"/>
        </form>
        {matches_html}
        </>
    }
}

async fn search(query: &str) -> Result<Vec<Project>, anyhow::Error> {
    let search_url = Url::parse(&format!(
        "{}/search/",
        PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL not set"))?
    ))?
    .join(query)?;

    let results = reqwest::get(search_url)
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(results)
}
