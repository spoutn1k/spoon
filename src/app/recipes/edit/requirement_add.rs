use crate::app::{status_bar::Message, AppContext};
use ladle::models::IngredientIndex;
use std::rc::Rc;
use unidecode::unidecode;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RequirementAddItemProps {
    pub create_requirement: Callback<(ladle::models::IngredientIndex, String, bool), ()>,
    pub ingredient_blacklist: Callback<(), Vec<String>>,
    pub ingredient_cache_refresh: Callback<()>,
}

#[derive(PartialEq, Clone, Default)]
enum RequirementAddItemMode {
    #[default]
    Collapsed,
    Open,
}

enum RequirementAddItemAction {
    SetIngredient(ladle::models::IngredientIndex),
    SetQuantity(String),
    ToggleOptional,
    Close,
    Open,
}

#[derive(PartialEq, Clone, Default)]
struct RequirementAddItemState {
    mode: RequirementAddItemMode,
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
            RequirementAddItemAction::Close => new_state = RequirementAddItemState::default(),
            RequirementAddItemAction::Open => new_state.mode = RequirementAddItemMode::Open,
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
    let props_cloned = props.clone();
    let create_ingredient = Callback::from(move |name: String| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::ingredient_create(
                context_cloned.settings.server_url.as_str(),
                name.as_str(),
                false,
                false,
                false,
                false,
            )
            .await
            {
                Ok(ingredient) => {
                    state_cloned
                        .dispatch(RequirementAddItemAction::SetIngredient(ingredient.clone()));
                    props_cloned.ingredient_cache_refresh.emit(());
                }
                Err(error) => context_cloned
                    .status
                    .emit(Message::Error(error.to_string(), chrono::Utc::now())),
            }
        });
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_ingredient_select = Callback::from(move |e: Event| {
        let selected_ingredient_id = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        if selected_ingredient_id == "new" {
            match web_sys::window()
                .unwrap()
                .prompt_with_message("Ingredient name:")
            {
                Ok(Some(name)) => create_ingredient.emit(name),
                Ok(None) => (),
                Err(error) => context_cloned.status.emit(Message::Error(
                    error.as_string().unwrap_or(String::default()),
                    chrono::Utc::now(),
                )),
            }
        }

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

            state_cloned.dispatch(RequirementAddItemAction::Close);
        }
    });

    let blacklist = props.ingredient_blacklist.emit(());
    let mut options: Vec<IngredientIndex> = context
        .ingredient_cache
        .iter()
        .filter(|idx| !blacklist.contains(&idx.id))
        .cloned()
        .collect();
    options.sort_by(|lhs, rhs| unidecode(&lhs.name).cmp(&unidecode(&rhs.name)));
    let option_html = options
        .iter()
        .map(|opt| {
            let selected = match &state.selected_ingredient {
                Some(idx) => &idx == &opt,
                None => false,
            };

            html! {
                <option
                    selected={selected}
                    value={opt.id.clone()}>
                    {opt.name.clone()}
                </option>
            }
        })
        .collect::<Html>();

    let state_cloned = state.clone();
    let on_add_clicked = Callback::from(move |_| {
        state_cloned.dispatch(RequirementAddItemAction::Open);
    });

    let state_cloned = state.clone();
    html! {
        if state_cloned.mode == RequirementAddItemMode::Collapsed {
            <button
                onclick={on_add_clicked}>
                {"Add ingredient"}
            </button>
        } else {
            <tr key={"requirement_add"}>
                <td>
                    <select
                        autocomplete="off"
                        onchange={on_ingredient_select}>
                        <option
                            hidden=true
                            disabled=true
                            selected={state_cloned.selected_ingredient.is_none()}>
                            {"Ingredient"}
                        </option>
                        <option
                            value="new">
                            {"Nouvel ingredient"}
                        </option>
                        {option_html}
                    </select>
                </td>
                <td>
                    <input
                        type="text"
                        placeholder="Quantity"
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
}
