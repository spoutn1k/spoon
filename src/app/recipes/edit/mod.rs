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
use futures::future::join_all;
use std::collections::BTreeSet;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeEditWindowProps {
    pub recipe_id: String,
    pub ingredient_cache_refresh: Callback<()>,
}

enum RecipeEditWindowActions {
    UpdateRecipe(ladle::models::Recipe),
    UpdateName(String),
    UpdateAuthor(String),
    UpdateDirections(String),

    AddRequirement(ladle::models::IngredientIndex, String, bool),
    UpdateRequirement(ladle::models::Requirement, String, bool),
    DeleteRequirement(ladle::models::Requirement),
    /*
    AddDependency(String, String, bool),
    UpdateDependency(String, String, bool),
    DeleteDependency(String),
    */
    AddTag(String),
    DeleteTag(String),
    //Reset,
}

#[derive(Clone, Default, Debug, PartialEq)]
struct RecipeEditWindowState {
    original_recipe: Option<ladle::models::Recipe>,
    new_recipe: ladle::models::Recipe,
}

impl Reducible for RecipeEditWindowState {
    type Action = RecipeEditWindowActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();

        match action {
            RecipeEditWindowActions::UpdateRecipe(recipe) => {
                new_state.original_recipe = Some(recipe.clone());
                new_state.new_recipe = recipe.clone();
            }
            RecipeEditWindowActions::UpdateName(name) => {
                new_state.new_recipe.name = name.clone();
            }
            RecipeEditWindowActions::UpdateAuthor(author) => {
                new_state.new_recipe.author = author.clone();
            }
            RecipeEditWindowActions::UpdateDirections(directions) => {
                new_state.new_recipe.directions = directions.clone();
            }
            RecipeEditWindowActions::AddRequirement(ingredient, quantity, optional) => {
                let new_requirement = ladle::models::Requirement {
                    ingredient: ingredient.clone(),
                    quantity,
                    optional,
                };

                if self
                    .new_recipe
                    .requirements
                    .iter()
                    .find(|r| &r.ingredient == &ingredient)
                    .is_none()
                {
                    new_state.new_recipe.requirements.insert(new_requirement);
                }
            }
            RecipeEditWindowActions::UpdateRequirement(req, quantity, optional) => {
                let new_requirement = ladle::models::Requirement {
                    ingredient: req.ingredient.clone(),
                    quantity,
                    optional,
                };

                new_state.new_recipe.requirements.remove(&req);
                new_state.new_recipe.requirements.insert(new_requirement);
            }
            RecipeEditWindowActions::DeleteRequirement(req) => {
                new_state.new_recipe.requirements.remove(&req);
            }
            RecipeEditWindowActions::AddTag(label_name) => {
                new_state.new_recipe.tags.insert(ladle::models::LabelIndex {
                    id: String::default(),
                    name: label_name.clone(),
                });
            }
            RecipeEditWindowActions::DeleteTag(label_name) => {
                new_state
                    .new_recipe
                    .tags
                    .retain(|lidx| lidx.name != label_name);
            }
        }

        new_state.into()
    }
}

#[function_component(RecipeEditWindow)]
pub fn edit_window(props: &RecipeEditWindowProps) -> Html {
    let navigator = use_navigator().unwrap();

    let state = use_reducer(RecipeEditWindowState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let fetch_recipe: Callback<()> = Callback::from(move |_| {
        if state_cloned.original_recipe.is_none() {
            let state_cloned = state_cloned.clone();
            let props_cloned = props_cloned.clone();
            let context_cloned = context_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ladle::recipe_get(
                    context_cloned.settings.server_url.as_str(),
                    &props_cloned.recipe_id.clone(),
                )
                .await
                {
                    Ok(recipe) => {
                        state_cloned.dispatch(RecipeEditWindowActions::UpdateRecipe(recipe))
                    }
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

            if let Some(recipe) = &state_cloned.original_recipe {
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

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_update_clicked = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Some(original) = &state_cloned.original_recipe {
                let recipe = &state_cloned.new_recipe;

                let common_requirements = &original
                    .requirements
                    .intersection(&recipe.requirements)
                    .cloned()
                    .collect();

                let missing_requirements = original
                    .requirements
                    .difference(common_requirements)
                    .collect::<BTreeSet<&ladle::models::Requirement>>();

                let requests = missing_requirements.iter().map(|requirement| {
                    ladle::requirement_delete(
                        &context_cloned.settings.server_url,
                        &recipe.id,
                        &requirement.ingredient.id,
                    )
                });

                join_all(requests)
                    .await
                    .iter()
                    .map(|response| match response {
                        Ok(_) => (),
                        Err(message) => log::error!("{}", message),
                    })
                    .for_each(drop);

                let new_requirements = recipe
                    .requirements
                    .difference(common_requirements)
                    .collect::<BTreeSet<&ladle::models::Requirement>>();

                let requests = new_requirements.iter().map(|requirement| {
                    ladle::requirement_create(
                        &context_cloned.settings.server_url,
                        &recipe.id,
                        &requirement.ingredient.id,
                        &requirement.quantity,
                        requirement.optional,
                    )
                });

                join_all(requests)
                    .await
                    .iter()
                    .map(|response| match response {
                        Ok(_) => (),
                        Err(message) => log::error!("{}", message),
                    })
                    .for_each(drop);

                let deleted_tags: Vec<_> = original.tags.difference(&recipe.tags).collect();

                let requests = deleted_tags.iter().map(|label| {
                    ladle::recipe_untag(&context_cloned.settings.server_url, &recipe.id, &label.id)
                });

                join_all(requests)
                    .await
                    .iter()
                    .map(|response| match response {
                        Ok(_) => (),
                        Err(message) => log::error!("{}", message),
                    })
                    .for_each(drop);

                let added_tags: Vec<_> = recipe.tags.difference(&original.tags).collect();

                let requests = added_tags.iter().map(|label| {
                    ladle::recipe_tag(&context_cloned.settings.server_url, &recipe.id, &label.name)
                });

                join_all(requests)
                    .await
                    .iter()
                    .map(|response| match response {
                        Ok(_) => (),
                        Err(message) => log::error!("{}", message),
                    })
                    .for_each(drop);

                match ladle::recipe_update(
                    &context_cloned.settings.server_url,
                    &recipe.id,
                    Some(&recipe.name),
                    Some(&recipe.author),
                    Some(&recipe.directions),
                    None,
                )
                .await
                {
                    Ok(_) => {}
                    Err(message) => context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                }
            }
        });
    });

    let nc = navigator.clone();
    let state_cloned = state.clone();
    let update = on_update_clicked.clone();
    let on_exit_clicked = Callback::from(move |_| {
        if let Some(recipe) = &state_cloned.original_recipe {
            if recipe.requirements != state_cloned.new_recipe.requirements
                || recipe.name != state_cloned.new_recipe.name
                || recipe.author != state_cloned.new_recipe.author
                || recipe.directions != state_cloned.new_recipe.directions
                || recipe.dependencies != state_cloned.new_recipe.dependencies
                || recipe.tags != state_cloned.new_recipe.tags
            {
                match web_sys::window()
                    .unwrap()
                    .confirm_with_message("Save before exiting ?")
                {
                    Ok(true) => update.emit(yew::MouseEvent::new("").unwrap()),
                    _ => (),
                }
            }
        }

        nc.back();
    });

    let state_cloned = state.clone();
    let on_name_edit = Callback::from(move |e: Event| {
        let name = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        state_cloned.dispatch(RecipeEditWindowActions::UpdateName(name));
    });

    let state_cloned = state.clone();
    let on_author_edit = Callback::from(move |e: Event| {
        let author = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        state_cloned.dispatch(RecipeEditWindowActions::UpdateAuthor(author));
    });

    let state_cloned = state.clone();
    let on_directions_edit = Callback::from(move |e: Event| {
        let directions = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        state_cloned.dispatch(RecipeEditWindowActions::UpdateDirections(directions));
    });

    let state_cloned = state.clone();
    let ingredients_in_use = Callback::from(move |()| -> Vec<String> {
        state_cloned
            .new_recipe
            .requirements
            .iter()
            .map(|r| r.ingredient.id.clone())
            .collect()
    });

    let fetch_recipe_cloned = fetch_recipe.clone();
    use_effect_with_deps(move |_| fetch_recipe_cloned.emit(()), props.clone());

    let state_cloned = state.clone();
    let recipe = &state_cloned.new_recipe;
    let dependencies = recipe
        .dependencies
        .iter()
        .map(|d| {
            html! {
                <DependencyEditItem
                    dependency={d.clone()}
                    refresh={fetch_recipe.clone()}
                />
            }
        })
        .collect::<Html>();

    let requirements = recipe
        .requirements
        .iter()
        .map(|r| {
            let state_cloned = state.clone();
            let rc = r.clone();
            let update = Callback::from(move |(quantity, optional): (String, bool)| {
                state_cloned.dispatch(RecipeEditWindowActions::UpdateRequirement(
                    rc.clone(),
                    quantity,
                    optional,
                ));
            });

            let state_cloned = state.clone();
            let rc = r.clone();
            let delete = Callback::from(move |_| {
                state_cloned.dispatch(RecipeEditWindowActions::DeleteRequirement(rc.clone()));
            });
            html! {
                <RequirementEditItem
                    requirement={r.clone()}
                    update_requirement={update}
                    delete_requirement={delete}
                />
            }
        })
        .collect::<Html>();

    let state_cloned = state.clone();
    let create_requirement = Callback::from(
        move |(ingredient, quantity, optional): (ladle::models::IngredientIndex, String, bool)| {
            state_cloned.dispatch(RecipeEditWindowActions::AddRequirement(
                ingredient, quantity, optional,
            ));
        },
    );

    let state_cloned = state.clone();
    let delete_tag = Callback::from(move |label: String| {
        state_cloned.dispatch(RecipeEditWindowActions::DeleteTag(label));
    });

    let state_cloned = state.clone();
    let add_tag = Callback::from(move |label: String| {
        state_cloned.dispatch(RecipeEditWindowActions::AddTag(label));
    });

    let tags = recipe
        .tags
        .iter()
        .map(|t| {
            let delete_tag_cloned = delete_tag.clone();
            html! {
                <TagEditItem
                    label={t.clone()}
                    delete_tag={delete_tag_cloned}
                />
            }
        })
        .collect::<Html>();

    let nc = navigator.clone();
    let state_cloned = state.clone();
    html! {
        <div class="recipe-display edit">
            <div>
                <input type="text"
                    class="recipe-name edit"
                    onchange={on_name_edit}
                    value={state_cloned.new_recipe.name.clone()}
                />
            </div>
            <div>
                <input type="text"
                    class="recipe-author edit"
                    onchange={on_author_edit}
                    value={state_cloned.new_recipe.author.clone()}
                />
            </div>
            <table>
                {dependencies}
                <DependencyAddItem
                    refresh={fetch_recipe.clone()}
                />
            </table>
            <table>
                {requirements}
                <RequirementAddItem
                    create_requirement={create_requirement}
                    ingredient_blacklist={ingredients_in_use}
                    ingredient_cache_refresh={props.ingredient_cache_refresh.clone()}
                />
            </table>
            <textarea
                class="recipe-directions edit"
                onchange={on_directions_edit}
                value={state_cloned.new_recipe.directions.clone()}
            />
            <ul>
                {tags}
                <TagAddItem
                    add_tag={add_tag}
                />
            </ul>
            <div class="options">
                <button onclick={on_update_clicked}>{"Update"}</button>
                <button onclick={on_delete_clicked}>{"Delete"}</button>
                <button
                    class={classes!("recipe-deselect")}
                    onclick={on_exit_clicked}>
                    {"Close"}
                </button>
            </div>
        </div>
    }
}
