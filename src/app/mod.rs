use std::ops::Deref;
mod recipe_list;
mod recipe_window;
use recipe_list::RecipeList;
use recipe_window::RecipeWindow;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
struct AppState {
    server: String,
    selected_recipe_id: Option<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state_eq(|| AppState {
        server: String::from("http://localhost:8000"),
        selected_recipe_id: None,
    });

    let cloned_state = state.clone();
    let on_server_change = Callback::from(move |e: Event| {
        let mut data = cloned_state.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.server = target.unchecked_into::<HtmlInputElement>().value();
        log::info!("New server: {}", &data.server);
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let on_recipe_select = Callback::from(move |recipe_id: String| {
        let mut data = cloned_state.deref().clone();
        data.selected_recipe_id = Some(recipe_id);
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let recipe_create = Callback::from(move |recipe_name: String| {
        let cloned_state = cloned_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            ladle::recipe_create(cloned_state.server.as_str(), recipe_name.as_str()).await;
        });
    });

    html! {
        <main>
            <input type="text"
                onchange={on_server_change}
                value={state.server.clone()}
            />
            <RecipeList
                url={state.server.clone()}
                on_click={on_recipe_select}
                create_recipe={recipe_create}
            />
            <RecipeWindow
                url={state.server.clone()}
                recipe_id={state.selected_recipe_id.clone()}
            />
        </main>
    }
}
