use crate::app::set_title;
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
    pub current: AppSettings,
    pub update_settings: Callback<AppSettings>,
}

#[derive(PartialEq, Clone, Default, Debug)]
struct SettingsState {
    server_field_contents: String,
}

#[function_component(Settings)]
pub fn settings(props: &SettingsProps) -> Html {
    let state = use_state(|| SettingsState {
        server_field_contents: props.current.server_url.clone(),
    });

    set_title("Settings - spoon");

    let state_cloned = state.clone();
    let on_server_edit = Callback::from(move |e: InputEvent| {
        let new_url = e
            .target()
            .expect("Intercepted event with no target")
            .unchecked_into::<HtmlInputElement>()
            .value();

        state_cloned.set(SettingsState {
            server_field_contents: new_url.clone(),
        });
    });

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let on_server_submit = Callback::from(move |_: MouseEvent| {
        props_cloned.update_settings.emit(AppSettings {
            server_url: state_cloned.server_field_contents.clone(),
        })
    });

    html! {
        <div class="settings">
            <table class="items">
                <tr>
                    <td>
                        <label for="server">{"Knife server url:"}</label>
                    </td>
                    <td>
                        <input type="text"
                            name="server"
                            oninput={on_server_edit}
                            value={state.server_field_contents.clone()}
                        />
                    </td>
                    <td>
                        <button
                            name="server_submit"
                            onclick={on_server_submit}>
                            {"Submit"}
                        </button>
                    </td>
                </tr>
            </table>
        </div>
    }
}
