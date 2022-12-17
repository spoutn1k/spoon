use crate::app::status_bar::Message;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct RequirementEditItemProps {
    url: String,
    requirement: ladle::models::Requirement,
}

#[function_component(RequirementEditItem)]
fn requirement_edit_item(props: &RequirementEditItemProps) -> Html {
    let quantity_buffer = use_state(|| props.requirement.quantity.clone());

    /*let update_requirement = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::
        });
    });*/

    html! {
        <li key={props.requirement.ingredient.id.as_str()}>
            <span>{props.requirement.ingredient.name.as_str()}</span>
            <input
                type="text"
                value={(*quantity_buffer).clone()}
            />
            <button>{"Update"}</button>
            <button>{"Delete"}</button>
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
                        requirement={r.clone()}
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
