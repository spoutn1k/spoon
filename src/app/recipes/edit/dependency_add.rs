use crate::app::AppContext;
use ladle::models::RecipeIndex;
use std::rc::Rc;
use unidecode::unidecode;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DependencyAddItemProps {
    pub create_dependency: Callback<(ladle::models::RecipeIndex, String, bool), ()>,
    pub recipe_blacklist: Callback<(), Vec<String>>,
}

#[derive(PartialEq, Clone, Default)]
enum DependencyAddItemMode {
    #[default]
    Collapsed,
    Open,
}

enum DependencyAddItemAction {
    SetRecipe(ladle::models::RecipeIndex),
    SetQuantity(String),
    ToggleOptional,
    Close,
    Open,
}

#[derive(PartialEq, Clone, Default)]
struct DependencyAddItemState {
    mode: DependencyAddItemMode,
    selected_recipe: Option<ladle::models::RecipeIndex>,
    quantity_buffer: String,
    optional: bool,
}

impl Reducible for DependencyAddItemState {
    type Action = DependencyAddItemAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();

        match action {
            DependencyAddItemAction::SetRecipe(ing) => {
                new_state.selected_recipe = Some(ing.clone())
            }
            DependencyAddItemAction::SetQuantity(qt) => new_state.quantity_buffer = qt,
            DependencyAddItemAction::ToggleOptional => new_state.optional = !new_state.optional,
            DependencyAddItemAction::Close => new_state = DependencyAddItemState::default(),
            DependencyAddItemAction::Open => new_state.mode = DependencyAddItemMode::Open,
        }

        new_state.into()
    }
}

#[function_component(DependencyAddItem)]
pub fn dependency_add_item(props: &DependencyAddItemProps) -> Html {
    let state = use_reducer(DependencyAddItemState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_recipe_select = Callback::from(move |e: Event| {
        let selected_recipe_id = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        if let Some(recipe) = context_cloned
            .recipe_cache
            .iter()
            .find(|index| index.id == selected_recipe_id)
        {
            state_cloned.dispatch(DependencyAddItemAction::SetRecipe(recipe.clone()));
        }
    });

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let value = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        state_cloned.dispatch(DependencyAddItemAction::SetQuantity(value));
    });

    let state_cloned = state.clone();
    let on_optional_clicked = Callback::from(move |_| {
        state_cloned.dispatch(DependencyAddItemAction::ToggleOptional);
    });

    let props_cloned = props.clone();
    let state_cloned = state.clone();
    let create_dependency = Callback::from(move |_| {
        if let Some(recipe) = &state_cloned.selected_recipe {
            props_cloned.create_dependency.emit((
                recipe.clone(),
                state_cloned.quantity_buffer.clone(),
                state_cloned.optional,
            ));

            state_cloned.dispatch(DependencyAddItemAction::Close);
        }
    });

    let blacklist = props.recipe_blacklist.emit(());
    let mut options: Vec<RecipeIndex> = context
        .recipe_cache
        .iter()
        .filter(|idx| !blacklist.contains(&idx.id))
        .cloned()
        .collect();
    options.sort_by(|lhs, rhs| unidecode(&lhs.name).cmp(&unidecode(&rhs.name)));
    let option_html = options
        .iter()
        .map(|opt| {
            let selected = match &state.selected_recipe {
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
        state_cloned.dispatch(DependencyAddItemAction::Open);
    });

    let state_cloned = state.clone();
    html! {
        if state_cloned.mode == DependencyAddItemMode::Collapsed {
            <button
                onclick={on_add_clicked}>
                {"Ajouter une recette necessaire"}
            </button>
        } else {
            <tr key={"dependency_add"}>
                <td>
                    <select
                        autocomplete="off"
                        onchange={on_recipe_select}>
                        <option
                            hidden=true
                            disabled=true
                            selected={state_cloned.selected_recipe.is_none()}>
                            {"Recipe"}
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
                        disabled={state_cloned.selected_recipe.is_none()}
                        onclick={create_dependency}>
                        {"Add"}
                    </button>
                </td>
            </tr>
        }
    }
}
