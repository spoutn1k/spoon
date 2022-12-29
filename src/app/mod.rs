use std::ops::Deref;

mod recipe_edit;
mod recipe_list;
mod recipe_window;
mod search_pane;
mod status_bar;

use recipe_edit::RecipeEditWindow;
use recipe_list::RecipeList;
use recipe_window::RecipeWindow;
use status_bar::{Message, StatusBar};

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
struct AppState {
    server: String,
    update: u32,
    selected_recipe_id: Option<String>,
    last_error: Message,
    settings_open: bool,
    edition: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state_eq(|| AppState {
        server: String::from("http://localhost:8000"),
        update: 0,
        selected_recipe_id: None,
        last_error: Message::None,
        settings_open: false,
        edition: false,
    });

    let state_cloned = state.clone();
    let on_server_change = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.server = target.unchecked_into::<HtmlInputElement>().value();
        data.update = data.update + 1;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_recipe_select = Callback::from(move |recipe_id: String| {
        let mut data = state_cloned.deref().clone();
        data.selected_recipe_id = Some(recipe_id);
        data.edition = false;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let recipe_deselect = Callback::from(move |_| {
        let mut data = state_cloned.deref().clone();
        data.selected_recipe_id = None;
        data.edition = false;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let display_status = Callback::from(move |status: Message| {
        let mut data = state_cloned.deref().clone();
        data.last_error = status;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let set_edit = Callback::from(move |edition: bool| {
        let mut data = state_cloned.deref().clone();
        data.edition = edition;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let set_settings_mode = Callback::from(move |mode: bool| {
        let mut data = state_cloned.deref().clone();
        data.settings_open = mode;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_delete: Callback<()> = Callback::from(move |_| {
        let mut data = state_cloned.deref().clone();

        data.selected_recipe_id = None;
        data.edition = false;
        data.update = data.update + 1;

        state_cloned.set(data)
    });

    let window = match state.edition {
        true => html! {
            <RecipeEditWindow
                url={state.server.clone()}
                recipe_id={state.selected_recipe_id.clone()}
                status={display_status.clone()}
                set_edition={set_edit}
                on_delete={on_delete}
            />
        },
        false => html! {
            <RecipeWindow
                url={state.server.clone()}
                recipe_id={state.selected_recipe_id.clone()}
                status={display_status.clone()}
                set_edition={set_edit}
                deselect={recipe_deselect}
            />
        },
    };

    let open_settings = set_settings_mode.clone();

    html! {
        <main>
            <StatusBar current={state.last_error.clone()} />
            <div class={format!("settings {}", if state.settings_open {"open"} else {"close"})}>
                <label for="server">{"Knife server url:"}</label>
                <input type="text"
                    name="server"
                    onchange={on_server_change}
                    value={state.server.clone()}
                />
                <button
                    class="settings-close"
                    onclick={move |_| open_settings.emit(false)}
                >
                    {"Close"}
                </button>
            </div>
            <div class="header">
                <div class="left">
                    <button onclick={move |_| set_settings_mode.clone().emit(true)}>
                        {"Settings"}
                    </button>
                </div>
                <div class="logo">
                    {format!("spoon v{}", env!("CARGO_PKG_VERSION"))}
                </div>
                <div class="right">
                </div>
            </div>
            <RecipeList
                url={state.server.clone()}
                update={state.update}
                on_click={on_recipe_select}
                status={display_status.clone()}
            />
            {window}
        </main>
    }
}
