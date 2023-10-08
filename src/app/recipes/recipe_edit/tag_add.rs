use crate::app::recipes::recipe_edit::EditionContext;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TagAddItemProps {
    pub refresh: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct TagAddItemState {
    label_name_buffer: String,
}

#[function_component(TagAddItem)]
pub fn tag_add_item(props: &TagAddItemProps) -> Html {
    let state = use_state(|| TagAddItemState {
        label_name_buffer: String::default(),
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());

    let state_cloned = state.clone();
    let on_label_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.label_name_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let recipe_id = edition_context.recipe_id.clone();
    let create_tag = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::recipe_tag(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
                data.label_name_buffer.as_str(),
            )
            .await
            {
                Ok(_) => {
                    props_cloned.refresh.emit(());
                    data.label_name_buffer = String::default();
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }

            state_cloned.set(data);
        });
    });

    html! {
        <li key={"tag_add"}>
            <input
                type="text"
                value={(*state).label_name_buffer.clone()}
                onchange={on_label_edit}
            />
            <button onclick={create_tag}>{"Add"}</button>
        </li>
    }
}
