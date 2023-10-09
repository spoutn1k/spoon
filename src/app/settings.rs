use crate::app::AppContext;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AppSettings {
    pub server_url: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SettingsProps {
    pub update_settings: Callback<AppSettings>,
}

#[derive(PartialEq, Clone, Default)]
struct SettingsState {
    server_field_contents: String,
}

#[function_component(Settings)]
pub fn settings(props: &SettingsProps) -> Html {
    let state = use_state(|| SettingsState::default());
    let context = use_context::<AppContext>().unwrap_or_default();

    let state_cloned = state.clone();
    let on_server_edit = Callback::from(move |e: Event| {
        let new_url = e
            .target()
            .expect("Intercepted event with no target")
            .unchecked_into::<HtmlInputElement>()
            .value();

        state_cloned.set(SettingsState {
            server_field_contents: new_url,
        });
    });

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let on_server_submit = Callback::from(move |_e: MouseEvent| {
        props_cloned.update_settings.emit(AppSettings {
            server_url: state_cloned.server_field_contents.clone(),
        })
    });

    html! {
        <div class="settings">
            <label for="server">{"Knife server url:"}</label>
            <input type="text"
                name="server"
                onchange={on_server_edit}
                value={context.settings.server_url.clone()}
            />
            <button name="server_submit" onclick={on_server_submit}>{"Submit"}</button>
        </div>
    }
}
