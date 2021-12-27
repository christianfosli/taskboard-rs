use taskboard_core_lib::Project;
use yew::prelude::*;

const PROJECT_SERVICE_URL: Option<&'static str> = option_env!("PROJECT_SERVICE_URL");

//pub enum Msg {
//    SetSearch(String),
//    PerformSearch,
//    SearchCompleted(Vec<Project>),
//    SearchFailed(String),
//}

fn do_search() -> Result<(), anyhow::Error> {
    // TODO: Use reqwest
    // let search_url = Url::parse(&format!(
    //     "{}/search/",
    //     PROJECT_SERVICE_URL.ok_or_else(|| anyhow!("PROJECT_SERVICE_URL not set"))?
    // ))?
    // .join(&self.search_query)?;

    // Request::get(search_url.as_str()).body(Nothing).map(|req| {
    //     let callback =
    //         self.link
    //             .callback(|res: Response<Json<Result<Vec<Project>, anyhow::Error>>>| {
    //                 if let (meta, Json(Ok(body))) = res.into_parts() {
    //                     if meta.status.is_success() {
    //                         return Msg::SearchCompleted(body);
    //                     }
    //                 }
    //                 Msg::SearchFailed(String::from("An error occured"))
    //             });

    //     self.ft = FetchService::fetch(req, callback).ok();
    // })?;
    Ok(())
}

#[function_component(SearchProject)]
pub fn search_project() -> Html {
    let query = use_state(|| "");
    let matches: UseStateHandle<Option<Vec<Project>>> = use_state(|| None);
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let handle_input = |_| {};
    // let handle_input = self.link.callback(|e: InputData| Msg::SetSearch(e.value));

    let handle_submit = |_| {};
    // let handle_submit = self.link.callback(|e: FocusEvent| {
    //     e.prevent_default();
    //     Msg::PerformSearch
    // });

    let to_li = |project: &Project| {
        html! {
            <li><a href={format!("/{}", project.id)}>{ &project.name }</a></li>
        }
    };

    let matches_html = match &*matches.clone() {
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

    let error = match &*error {
        Some(e) => html! {
            <div class="error">
            { e }
            </div>
        },
        None => html! {},
    };

    html! {
        <>
        <h3>{ "Search for an existing project" }</h3>
        <form onsubmit={handle_submit}>
            <label for="search-project-field">{ "Project name" }</label>
            <input type="text" id="search-project-field" name="query" value={*query} oninput={handle_input} required={true}/>
            <input type="submit" value="Search"/>
        </form>
        {error}
        {matches_html}
        </>
    }
}
