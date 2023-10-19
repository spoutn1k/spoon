use crate::app::AppContext;
use ladle::models::IngredientIndex;
use std::rc::Rc;
use unidecode::unidecode;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RequirementAddItemProps {
    pub create_requirement: Callback<(ladle::models::IngredientIndex, String, bool), ()>,
}

enum RequirementAddItemAction {
    SetIngredient(ladle::models::IngredientIndex),
    SetQuantity(String),
    ToggleOptional,
    Reset,
}

#[derive(PartialEq, Clone, Default)]
struct RequirementAddItemState {
    selected_ingredient: Option<ladle::models::IngredientIndex>,
    quantity_buffer: String,
    optional: bool,
}

impl Reducible for RequirementAddItemState {
    type Action = RequirementAddItemAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();

        match action {
            RequirementAddItemAction::SetIngredient(ing) => {
                new_state.selected_ingredient = Some(ing.clone())
            }
            RequirementAddItemAction::SetQuantity(qt) => new_state.quantity_buffer = qt,
            RequirementAddItemAction::ToggleOptional => new_state.optional = !new_state.optional,
            RequirementAddItemAction::Reset => new_state = RequirementAddItemState::default(),
        }

        new_state.into()
    }
}

#[function_component(RequirementAddItem)]
pub fn requirement_add_item(props: &RequirementAddItemProps) -> Html {
    let state = use_reducer(RequirementAddItemState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_ingredient_select = Callback::from(move |e: Event| {
        let selected_ingredient_id = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        if let Some(ingredient) = context_cloned
            .ingredient_cache
            .iter()
            .find(|index| index.id == selected_ingredient_id)
        {
            state_cloned.dispatch(RequirementAddItemAction::SetIngredient(ingredient.clone()));
        }
    });

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let value = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        state_cloned.dispatch(RequirementAddItemAction::SetQuantity(value));
    });

    let state_cloned = state.clone();
    let on_optional_clicked = Callback::from(move |_| {
        state_cloned.dispatch(RequirementAddItemAction::ToggleOptional);
    });

    let props_cloned = props.clone();
    let state_cloned = state.clone();
    let create_requirement = Callback::from(move |_| {
        if let Some(ingredient) = &state_cloned.selected_ingredient {
            props_cloned.create_requirement.emit((
                ingredient.clone(),
                state_cloned.quantity_buffer.clone(),
                state_cloned.optional,
            ));

            state_cloned.dispatch(RequirementAddItemAction::Reset);
        }
    });

    let mut options: Vec<IngredientIndex> = context.ingredient_cache.iter().cloned().collect();
    options.sort_by(|lhs, rhs| unidecode(&lhs.name).cmp(&unidecode(&rhs.name)));
    let options = options
        .iter()
        .map(|opt| {
            html! {
                <option
                    value={opt.id.clone()}>
                    {opt.name.clone()}
                </option>
            }
        })
        .collect::<Html>();

    let selected_index = match &state.selected_ingredient.is_some() {
        true => None,
        false => Some(0.to_string()),
    };

    let state_cloned = state.clone();
    html! {
        <tr key={"requirement_add"}>
            <td>
                <select
                    onchange={on_ingredient_select}>
                    <option
                        hidden={true}
                        disabled={true}
                        selected={state_cloned.selected_ingredient.is_none()}
                        selectedIndex={selected_index}>
                        {"Ingredient"}
                    </option>
                    {options}
                </select>
            </td>
            <td>
                <input
                    type="text"
                    value={state.quantity_buffer.clone()}
                    onchange={on_quantity_edit}
                />
            </td>
            <td>
                <input
                    type="checkbox"
                    checked={state.optional}
                    onclick={on_optional_clicked}
                />
            </td>
            <td>
                <button
                    disabled={state_cloned.selected_ingredient.is_none()}
                    onclick={create_requirement}>
                    {"Add"}
                </button>
            </td>
        </tr>
    }
}
