use crate::app::recipes::recipe_edit::EditionContext;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DependencyAddItemProps {
    pub refresh: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct DependencyAddItemState {
    dependency_id_buffer: Option<String>,
    choices: Vec<ladle::models::RecipeIndex>,
}

#[function_component(DependencyAddItem)]
pub fn dependency_add_item(props: &DependencyAddItemProps) -> Html {
    let state = use_state(|| DependencyAddItemState {
        dependency_id_buffer: None,
        choices: vec![],
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let refresh_selection = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::recipe_index(context_cloned.settings.server_url.as_str(), "").await {
                Ok(mut recipes) => {
                    recipes.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.choices = recipes;
                    state_cloned.set(data);
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }
        })
    });

    use_effect_with_deps(move |_| refresh_selection.emit(()), props.clone());

    let state_cloned = state.clone();
    let on_dependency_select = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.dependency_id_buffer = Some(target.unchecked_into::<HtmlInputElement>().value());
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let recipe_id = edition_context.recipe_id;
    let create_dependency = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::dependency_create(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
                data.dependency_id_buffer.clone().unwrap().as_str(),
                "",
                false,
            )
            .await
            {
                Ok(_) => {
                    props_cloned.refresh.emit(());
                    data.dependency_id_buffer = None;
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }

            state_cloned.set(data);
        });
    });

    let options = state
        .choices
        .iter()
        .map(|r| html! {<option value={r.id.clone()}>{r.name.clone()}</option>})
        .collect::<Html>();

    html! {
        <tr key={"dependency_add"}>
            <td><select onchange={on_dependency_select}>
                <option hidden={true} disabled={true} selected={true}>{"Recipes"}</option>
                {options}
            </select></td>
            <td><button onclick={create_dependency}>{"Add"}</button></td>
        </tr>
    }
}
