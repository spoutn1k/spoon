use crate::app::recipes::recipe_edit::EditionContext;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RequirementEditItemProps {
    pub requirement: ladle::models::Requirement,
    pub refresh: Callback<()>,
}

#[derive(PartialEq, Clone, Default, Debug)]
struct RequirementEditItemState {
    ingredient_id: String,
    quantity_buffer: String,
}

#[function_component(RequirementEditItem)]
pub fn requirement_edit_item(props: &RequirementEditItemProps) -> Html {
    let state = use_state(|| RequirementEditItemState {
        ingredient_id: props.requirement.ingredient.id.clone(),
        quantity_buffer: props.requirement.quantity.clone(),
    });

    if props.requirement.ingredient.id != state.ingredient_id {
        state.set(RequirementEditItemState {
            ingredient_id: props.requirement.ingredient.id.clone(),
            quantity_buffer: props.requirement.quantity.clone(),
        });
    }

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let quantity = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        data.quantity_buffer = quantity;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let recipe_id = edition_context.recipe_id.clone();
    let update_requirement = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_update(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
                props_cloned.requirement.ingredient.id.as_str(),
                Some(state_cloned.quantity_buffer.as_str()),
                None,
            )
            .await
            {
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                Ok(_) => props_cloned.refresh.emit(()),
            }
        });
    });

    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let recipe_id = edition_context.recipe_id.clone();
    let delete_requirement = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_delete(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
                props_cloned.requirement.ingredient.id.as_str(),
            )
            .await
            {
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                Ok(_) => props_cloned.refresh.emit(()),
            }
        });
    });

    html! {
        <tr key={props.requirement.ingredient.id.as_str()}>
            <td>{props.requirement.ingredient.name.as_str()}</td>
            <td><input
                type="text"
                value={state.quantity_buffer.clone()}
                onchange={on_quantity_edit}
            /></td>
            <td><button onclick={update_requirement}>{"Update"}</button></td>
            <td><button onclick={delete_requirement}>{"Delete"}</button></td>
        </tr>
    }
}
