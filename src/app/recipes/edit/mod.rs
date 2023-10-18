mod context;
mod dependency_add;
mod dependency_edit;
mod requirement_add;
mod requirement_edit;
mod tag_add;
mod tag_edit;

use dependency_add::DependencyAddItem;
use dependency_edit::DependencyEditItem;
use requirement_add::RequirementAddItem;
use requirement_edit::RequirementEditItem;
use tag_add::TagAddItem;
use tag_edit::TagEditItem;

use context::EditionContext;

use crate::app::{status_bar::Message, AppContext, Route};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeEditWindowProps {
    pub recipe_id: String,
}

#[derive(Clone)]
pub struct RecipeEditWindowState {
    pub recipe: Option<ladle::models::Recipe>,
    pub name_buffer: String,
    pub author_buffer: String,
    pub directions_buffer: String,
}

#[function_component(RecipeEditWindow)]
pub fn edit_window(props: &RecipeEditWindowProps) -> Html {
    let navigator = use_navigator().unwrap();

    let state = use_state(|| RecipeEditWindowState {
        recipe: None,
        name_buffer: String::default(),
        author_buffer: String::default(),
        directions_buffer: String::default(),
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edit_context = use_state(|| EditionContext {
        recipe_id: props.recipe_id.clone(),
    });

    let state_cloned = state.clone();
    let on_name_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.name_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_author_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.author_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let on_directions_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.directions_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let refresh_recipe: Callback<()> = Callback::from(move |_| {
        let recipe_id = props_cloned.recipe_id.clone();

        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();

            match ladle::recipe_get(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
            )
            .await
            {
                Ok(recipe) => {
                    data.name_buffer = recipe.name.clone();
                    data.author_buffer = recipe.author.clone();
                    data.directions_buffer = recipe.directions.clone();
                    data.recipe = Some(recipe);
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }

            state_cloned.set(data)
        });
    });

    let context_cloned = context.clone();
    let state_cloned = state.clone();
    let refresh_recipe_cloned = refresh_recipe.clone();
    let update_name = Callback::from(move |_| {
        if state_cloned.recipe.is_some() {
            let context_cloned = context_cloned.clone();
            let state_cloned = state_cloned.clone();
            let refresh_recipe_cloned = refresh_recipe_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(
                    context_cloned.settings.server_url.as_str(),
                    id.as_str(),
                    Some(state_cloned.name_buffer.as_str()),
                    None,
                    None,
                    None,
                )
                .await
                {
                    Ok(_) => refresh_recipe_cloned.emit(()),
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            });
        }
    });

    let context_cloned = context.clone();
    let state_cloned = state.clone();
    let refresh_recipe_cloned = refresh_recipe.clone();
    let update_author = Callback::from(move |_| {
        if state_cloned.recipe.is_some() {
            let context_cloned = context_cloned.clone();
            let state_cloned = state_cloned.clone();
            let refresh_recipe_cloned = refresh_recipe_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(
                    context_cloned.settings.server_url.as_str(),
                    id.as_str(),
                    None,
                    Some(state_cloned.author_buffer.as_str()),
                    None,
                    None,
                )
                .await
                {
                    Ok(_) => refresh_recipe_cloned.emit(()),
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            });
        }
    });

    let context_cloned = context.clone();
    let state_cloned = state.clone();
    let refresh_recipe_cloned = refresh_recipe.clone();
    let update_directions = Callback::from(move |_| {
        if state_cloned.recipe.is_some() {
            let context_cloned = context_cloned.clone();
            let state_cloned = state_cloned.clone();
            let refresh_recipe_cloned = refresh_recipe_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(
                    context_cloned.settings.server_url.as_str(),
                    id.as_str(),
                    None,
                    None,
                    Some(state_cloned.directions_buffer.as_str()),
                    None,
                )
                .await
                {
                    Ok(_) => refresh_recipe_cloned.emit(()),
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            });
        }
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let nc = navigator.clone();
    let on_delete_clicked = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let nc = nc.clone();

        let confirm = match web_sys::window()
            .unwrap()
            .confirm_with_message(&format!("Delete recipe ?"))
        {
            Ok(true) => true,
            _ => false,
        };

        wasm_bindgen_futures::spawn_local(async move {
            if !confirm {
                return;
            }

            if let Some(recipe) = &state_cloned.recipe {
                match ladle::recipe_delete(context_cloned.settings.server_url.as_str(), &recipe.id)
                    .await
                {
                    Ok(_) => {
                        nc.push(&Route::ListRecipes);
                    }
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            }
        });
    });

    let refresh_recipe_cloned = refresh_recipe.clone();
    use_effect_with_deps(move |_| refresh_recipe_cloned.emit(()), props.clone());

    let state_cloned = state.clone();
    if let Some(recipe) = &(*state_cloned).recipe {
        let dependencies = recipe
            .dependencies
            .iter()
            .map(|d| {
                html! {
                    <DependencyEditItem
                        dependency={d.clone()}
                        refresh={refresh_recipe.clone()}
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
                        requirement={r.clone()}
                        refresh={refresh_recipe.clone()}
                    />
                }
            })
            .collect::<Html>();

        let tags = recipe
            .tags
            .iter()
            .map(|t| {
                html! {
                    <TagEditItem
                        label={t.clone()}
                        refresh={refresh_recipe.clone()}
                    />
                }
            })
            .collect::<Html>();

        let nc = navigator.clone();

        html! {
            <div class="recipe-display edit">
                <ContextProvider<EditionContext> context={(*edit_context).clone()}>
                    <div>
                        <input type="text"
                            class="recipe-name edit"
                            onchange={on_name_edit}
                            value={state_cloned.name_buffer.clone()}
                        />
                        <button onclick={update_name}>{"Update"}</button>
                    </div>
                    <div>
                        <input type="text"
                            class="recipe-author edit"
                            onchange={on_author_edit}
                            value={state_cloned.author_buffer.clone()}
                        />
                        <button onclick={update_author}>{"Update"}</button>
                    </div>
                    <table>
                        {dependencies}
                        <DependencyAddItem
                            refresh={refresh_recipe.clone()}
                        />
                    </table>
                    <table>
                        {requirements}
                        <RequirementAddItem
                            refresh={refresh_recipe.clone()}
                        />
                    </table>
                    <textarea
                        class="recipe-directions edit"
                        onchange={on_directions_edit}
                        value={state_cloned.directions_buffer.clone()}
                    />
                    <button onclick={update_directions}>{"Update directions"}</button>
                    <ul>
                        {tags}
                        <TagAddItem
                            refresh={refresh_recipe.clone()}
                        />
                    </ul>
                    <div class="options">
                        <button onclick={on_delete_clicked}>{"Delete"}</button>
                        <button
                            class={classes!("recipe-deselect")}
                            onclick={Callback::from(move |_| {
                                nc.back();
                            })}>
                            {"Close"}
                        </button>
                    </div>
                </ContextProvider<EditionContext>>
            </div>
        }
    } else {
        html! {}
    }
}
