use std::ops::Deref;

use crate::app::status_bar::Message;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct RequirementAddItemProps {
    url: String,
    recipe_id: String,
    status: Callback<Message>,
}

#[derive(PartialEq, Clone)]
struct RequirementAddItemState {
    ingredient_buffer: String,
    quantity_buffer: String,
}

#[function_component(RequirementAddItem)]
fn requirement_add_item(props: &RequirementAddItemProps) -> Html {
    let state = use_state(|| RequirementAddItemState {
        ingredient_buffer: String::default(),
        quantity_buffer: String::default(),
    });

    let state_cloned = state.clone();
    let on_ingredient_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.ingredient_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.quantity_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let props_cloned = props.clone();
    let state_cloned = state.clone();
    let create_requirement = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let data = state_cloned.deref().clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_create_from_ingredient_name(
                props_cloned.url.as_str(),
                props_cloned.recipe_id.as_str(),
                data.ingredient_buffer.as_str(),
                data.quantity_buffer.as_str(),
            )
            .await
            {
                Err(message) => props_cloned
                    .status
                    .emit(Message::Error(message.to_string())),
                _ => (),
            }
        });
    });

    html! {
        <li key={"requirement_add"}>
            <input
                type="text"
                value={(*state).ingredient_buffer.clone()}
                onchange={on_ingredient_edit}
            />
            <input
                type="text"
                value={(*state).quantity_buffer.clone()}
                onchange={on_quantity_edit}
            />
            <button onclick={create_requirement}>{"Add"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct RequirementEditItemProps {
    url: String,
    recipe_id: String,
    requirement: ladle::models::Requirement,
    status: Callback<Message>,
}

#[function_component(RequirementEditItem)]
fn requirement_edit_item(props: &RequirementEditItemProps) -> Html {
    let state = use_state(|| props.requirement.quantity.clone());

    let state_cloned = state.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("");
        let input = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(input);
    });

    let props_cloned = props.clone();
    let state_cloned = state.clone();
    let update_requirement = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let state_cloned = state_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_update(
                props_cloned.url.as_str(),
                props_cloned.recipe_id.as_str(),
                props_cloned.requirement.ingredient.id.as_str(),
                (*state_cloned).as_str(),
            )
            .await
            {
                Err(message) => props_cloned
                    .status
                    .emit(Message::Error(message.to_string())),
                _ => (),
            }
        });
    });

    let props_cloned = props.clone();
    let state_cloned = state.clone();
    let delete_requirement = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let state_cloned = state_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::requirement_delete(
                props_cloned.url.as_str(),
                props_cloned.recipe_id.as_str(),
                props_cloned.requirement.ingredient.id.as_str(),
            )
            .await
            {
                Err(message) => props_cloned
                    .status
                    .emit(Message::Error(message.to_string())),
                _ => (),
            }
        });
    });

    html! {
        <li key={props.requirement.ingredient.id.as_str()}>
            <span>{props.requirement.ingredient.name.as_str()}</span>
            <input
                type="text"
                value={(*state).clone()}
                onchange={on_quantity_edit}
            />
            <button onclick={update_requirement}>{"Update"}</button>
            <button onclick={delete_requirement}>{"Delete"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DependencyEditItemProps {
    url: String,
    recipe_id: String,
    dependency: ladle::models::Dependency,
    status: Callback<Message>,
}

#[function_component(DependencyEditItem)]
fn dependency_edit_item(props: &DependencyEditItemProps) -> Html {
    let props_cloned = props.clone();
    let on_dependency_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(message) = ladle::recipe_unlink(
                props_cloned.url.as_str(),
                props_cloned.recipe_id.as_str(),
                props_cloned.dependency.id.as_str(),
            )
            .await
            {
                props_cloned
                    .status
                    .emit(Message::Error(message.to_string()));
            }
        });
    });

    html! {
        <li key={props.dependency.id.as_str()}>
            <span>{props.dependency.name.as_str()}</span>
            <button onclick={on_dependency_delete}>{"Delete"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeEditWindowProps {
    pub url: String,
    pub recipe_id: Option<String>,
    pub status: Callback<Message>,
    pub set_edition: Callback<bool>,
    pub on_delete: Callback<()>,
}

#[derive(Clone)]
pub struct RecipeEditWindowState {
    pub recipe: Option<ladle::models::Recipe>,
    pub name_buffer: String,
    pub author_buffer: String,
    pub directions_buffer: String,
}

#[function_component(RecipeEditWindow)]
pub fn recipe_edit_window(props: &RecipeEditWindowProps) -> Html {
    let state = use_state(|| RecipeEditWindowState {
        recipe: None,
        name_buffer: String::default(),
        author_buffer: String::default(),
        directions_buffer: String::default(),
    });

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    use_effect_with_deps(
        move |_| {
            let state_cloned = state_cloned.clone();
            if let Some(id) = props_cloned.recipe_id.clone() {
                let props_cloned = props_cloned.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut data = state_cloned.deref().clone();

                    match ladle::recipe_get(props_cloned.url.as_str(), id.as_str()).await {
                        Ok(recipe) => {
                            data.name_buffer = recipe.name.clone();
                            data.author_buffer = recipe.author.clone();
                            data.directions_buffer = recipe.directions.clone();
                            data.recipe = Some(recipe);
                        }
                        Err(message) => props_cloned
                            .status
                            .emit(Message::Error(message.to_string())),
                    }

                    state_cloned.set(data)
                });
            }
        },
        props.clone(),
    );

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    if let Some(recipe) = &(*state_cloned).recipe {
        let on_click_edit = Callback::from(move |_| props_cloned.set_edition.emit(false));
        let on_click_delete = Callback::from(move |_| props_cloned.on_delete.emit(()));

        let dependencies = recipe
            .dependencies
            .iter()
            .map(|d| {
                html! {
                    <DependencyEditItem
                        url={props_cloned.url.clone()}
                        recipe_id={recipe.id.clone()}
                        dependency={d.clone()}
                        status={props_cloned.status.clone()}
                    />
                }
            })
            .collect::<Html>();

        let requirements = recipe
            .requirements
            .iter()
            .map(|r| {
                html! {
                    <RequirementEditItem
                        url={props_cloned.url.clone()}
                        recipe_id={recipe.id.clone()}
                        requirement={r.clone()}
                        status={props_cloned.status.clone()}
                    />
                }
            })
            .collect::<Html>();

        html! {
            <div class="recipe-display edit">
                <div>
                    <input type="text"
                        class="recipe-name edit"
                        value={state_cloned.name_buffer.clone()}
                    />
                    <button>{"Update"}</button>
                </div>
                <div>
                    <input type="text"
                        class="recipe-author edit"
                        value={state_cloned.author_buffer.clone()}
                    />
                    <button>{"Update"}</button>
                </div>
                <ul>
                    {dependencies}
                </ul>
                <ul>
                    {requirements}
                    <RequirementAddItem
                        url={props_cloned.url.clone()}
                        recipe_id={recipe.id.clone()}
                        status={props_cloned.status.clone()}
                    />
                </ul>
                <textarea
                    class="recipe-directions edit"
                    value={state_cloned.directions_buffer.clone()}
                />
                <div class="options">
                    <button onclick={on_click_edit}>{"Done"}</button>
                    <button onclick={on_click_delete}>{"Delete"}</button>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
