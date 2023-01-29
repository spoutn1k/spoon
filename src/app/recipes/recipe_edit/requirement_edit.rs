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
    let update_requirement = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_update(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
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
    let delete_requirement = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_delete(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
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
        <li key={props.requirement.ingredient.id.as_str()}>
            <span>{props.requirement.ingredient.name.as_str()}</span>
            <input
                type="text"
                value={state.quantity_buffer.clone()}
                onchange={on_quantity_edit}
            />
            <button onclick={update_requirement}>{"Update"}</button>
            <button onclick={delete_requirement}>{"Delete"}</button>
        </li>
    }
}
