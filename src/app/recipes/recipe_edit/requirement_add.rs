use crate::app::recipes::recipe_edit::EditionContext;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use ladle::models::IngredientIndex;
use std::ops::Deref;
use unidecode::unidecode;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RequirementAddItemProps {
    pub refresh: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct RequirementAddItemState {
    selected_ingredient_id: Option<String>,
    quantity_buffer: String,
}

#[function_component(RequirementAddItem)]
pub fn requirement_add_item(props: &RequirementAddItemProps) -> Html {
    let state = use_state(|| RequirementAddItemState {
        selected_ingredient_id: None,
        quantity_buffer: String::default(),
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());

    let state_cloned = state.clone();
    let on_ingredient_select = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.selected_ingredient_id = Some(target.unchecked_into::<HtmlInputElement>().value());
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.quantity_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let edition_context_cloned = edition_context.clone();
    let props_cloned = props.clone();
    let create_requirement = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let edition_context_cloned = edition_context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();

            if let Some(value) = data.selected_ingredient_id {
                match ladle::requirement_create(
                    context_cloned.settings.server_url.as_str(),
                    edition_context_cloned.recipe_id.as_str(),
                    value.as_str(),
                    data.quantity_buffer.as_str(),
                    false,
                )
                .await
                {
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                    Ok(_) => {
                        props_cloned.refresh.emit(());
                        data.selected_ingredient_id = None;
                        data.quantity_buffer = String::default();

                        state_cloned.set(data);
                    }
                }
            }
        });
    });

    let mut options: Vec<IngredientIndex> = context.ingredient_cache.iter().cloned().collect();
    options.sort_by(|lhs, rhs| unidecode(&lhs.name).cmp(&unidecode(&rhs.name)));
    let options = options
        .iter()
        .map(|opt| html! {<option value={opt.id.clone()}>{opt.name.clone()}</option>})
        .collect::<Html>();

    html! {
        <tr key={"requirement_add"}>
            <td><select onchange={on_ingredient_select}>
                <option hidden={true} disabled={true} selected={true}>{"Ingredients"}</option>
                {options}
            </select></td>
            <td><input
                type="text"
                value={(*state).quantity_buffer.clone()}
                onchange={on_quantity_edit}
            /></td>
            <td><button onclick={create_requirement}>{"Add"}</button></td>
        </tr>
    }
}
