use std::ops::Deref;
mod recipe_list;
mod recipe_window;
mod status_bar;
use recipe_list::RecipeList;
use recipe_window::RecipeWindow;
use status_bar::{Message, StatusBar};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
struct AppState {
    server: String,
    selected_recipe_id: Option<String>,
    last_error: Message,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state_eq(|| AppState {
        server: String::from("http://localhost:8000"),
        selected_recipe_id: None,
        last_error: Message::None,
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
    let display_status = Callback::from(move |status: Message| {
        let mut data = cloned_state.deref().clone();
        data.last_error = status;
        cloned_state.set(data);
    });

    html! {
        <main>
            <StatusBar current={state.last_error.clone()} />
            <input type="text"
                onchange={on_server_change}
                value={state.server.clone()}
            />
            <RecipeList
                url={state.server.clone()}
                on_click={on_recipe_select}
                status={display_status.clone()}
            />
            <RecipeWindow
                url={state.server.clone()}
                recipe_id={state.selected_recipe_id.clone()}
                status={display_status}
            />
        </main>
    }
}
