use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use log::warn;

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientEditProps {
    pub ingredient_id: Option<String>,
}

enum IngredientEditActions {
    UpdateIngredient(ladle::models::Ingredient),
    UpdateName(String),
    ToggleDairy,
    ToggleMeat,
    ToggleGluten,
    ToggleAnimalProduct,
    Reset,
}

#[derive(Clone, Default, Debug)]
struct IngredientEditState {
    original_ingredient: Option<ladle::models::Ingredient>,
    new_ingredient: ladle::models::Ingredient,
}

impl Reducible for IngredientEditState {
    type Action = IngredientEditActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();

        match action {
            IngredientEditActions::UpdateIngredient(ingredient) => {
                new_state.original_ingredient = Some(ingredient.clone());
                new_state.new_ingredient = ingredient.clone();
            }
            IngredientEditActions::UpdateName(new_name) => new_state.new_ingredient.name = new_name,
            IngredientEditActions::ToggleDairy => {
                new_state.new_ingredient.classifications.dairy =
                    !new_state.new_ingredient.classifications.dairy
            }
            IngredientEditActions::ToggleGluten => {
                new_state.new_ingredient.classifications.gluten =
                    !new_state.new_ingredient.classifications.gluten
            }
            IngredientEditActions::ToggleMeat => {
                new_state.new_ingredient.classifications.meat =
                    !new_state.new_ingredient.classifications.meat
            }
            IngredientEditActions::ToggleAnimalProduct => {
                new_state.new_ingredient.classifications.animal_product =
                    !new_state.new_ingredient.classifications.animal_product
            }
            IngredientEditActions::Reset => {
                new_state.new_ingredient = match &new_state.original_ingredient {
                    Some(data) => data.clone(),
                    None => ladle::models::Ingredient::default(),
                }
            }
        }

        new_state.into()
    }
}

#[function_component(IngredientEdit)]
pub fn ingredient_edit_window(props: &IngredientEditProps) -> Html {
    let state = use_reducer(IngredientEditState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let ingredient_id = props.ingredient_id.clone();
    use_effect_with_deps(
        move |_| {
            let state_cloned = state_cloned.clone();
            let context_cloned = context_cloned.clone();
            let ingredient_id = ingredient_id.clone();
            if let Some(ingredient_id) = ingredient_id {
                wasm_bindgen_futures::spawn_local(async move {
                    match ladle::ingredient_get(&context_cloned.settings.server_url, &ingredient_id)
                        .await
                    {
                        Ok(ingredient) => {
                            state_cloned
                                .dispatch(IngredientEditActions::UpdateIngredient(ingredient));
                        }
                        Err(message) => context_cloned
                            .status
                            .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                    }
                });
            }
        },
        props.ingredient_id.clone(),
    );

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_update_clicked = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if state_cloned.original_ingredient.is_some() {
                let new = &state_cloned.new_ingredient;
                match ladle::ingredient_update(
                    context_cloned.settings.server_url.as_str(),
                    &new.id,
                    Some(&new.name),
                    Some(new.classifications.dairy),
                    Some(new.classifications.meat),
                    Some(new.classifications.gluten),
                    Some(new.classifications.animal_product),
                )
                .await
                {
                    Ok(_) => context_cloned.status.emit(Message::Success(
                        String::from("Ingredient updated"),
                        chrono::Utc::now(),
                    )),
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            }
        });
    });

    let state_cloned = state.clone();
    let on_name_edit = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("");
        let name = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.dispatch(IngredientEditActions::UpdateName(name));
    });

    let state_cloned = state.clone();
    let on_dairy_clicked = Callback::from(move |_| {
        state_cloned.dispatch(IngredientEditActions::ToggleDairy);
    });

    let state_cloned = state.clone();
    let on_gluten_clicked = Callback::from(move |_| {
        state_cloned.dispatch(IngredientEditActions::ToggleGluten);
    });

    let state_cloned = state.clone();
    let on_meat_clicked = Callback::from(move |_| {
        state_cloned.dispatch(IngredientEditActions::ToggleMeat);
    });

    let state_cloned = state.clone();
    let on_animal_product_clicked = Callback::from(move |_| {
        state_cloned.dispatch(IngredientEditActions::ToggleAnimalProduct);
    });

    let state_cloned = state.clone();
    let on_reset_clicked = Callback::from(move |_| {
        state_cloned.dispatch(IngredientEditActions::Reset);
    });

    let state_cloned = state.clone();
    html! {
    <div>
        <input type="text"
            class="ingredient-name edit"
            onchange={on_name_edit}
            value={state_cloned.new_ingredient.name.clone()}
        />
        <div>
            <table>
                <tr>
                    <td>
                        <input type="checkbox"
                            name="gluten"
                            onclick={on_gluten_clicked}
                            checked={state_cloned.new_ingredient.classifications.gluten} />
                    </td>
                    <td>
                        {"contains gluten"}
                    </td>
                </tr>
                <tr>
                    <td>
                        <input type="checkbox"
                            name="meat"
                            onclick={on_meat_clicked}
                            checked={state_cloned.new_ingredient.classifications.meat} />
                    </td>
                    <td>
                        {"contains meat"}
                    </td>
                </tr>
                <tr>
                    <td>
                        <input type="checkbox"
                            name="dairy"
                            onclick={on_dairy_clicked}
                            checked={state_cloned.new_ingredient.classifications.dairy} />
                    </td>
                    <td>
                        {"contains dairy"}
                    </td>
                </tr>
                <tr>
                    <td>
                        <input type="checkbox"
                            name="animal_product"
                            onclick={on_animal_product_clicked}
                            checked={state_cloned.new_ingredient.classifications.animal_product} />
                    </td>
                    <td>
                        {"contains animal products"}
                    </td>
                </tr>
            </table>
            <button onclick={on_reset_clicked}>{"Reset"}</button>
            <button onclick={on_update_clicked}>{"Update"}</button>
        </div>
    </div>
    }
}
