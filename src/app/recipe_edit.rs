use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::collections::HashMap;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct RequirementAddItemProps {
    refresh: Callback<()>,
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

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

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

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let create_requirement = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::requirement_create_from_ingredient_name(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                data.ingredient_buffer.as_str(),
                data.quantity_buffer.as_str(),
            )
            .await
            {
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                Ok(_) => {
                    props_cloned.refresh.emit(());
                    data.ingredient_buffer = String::default();
                    data.quantity_buffer = String::default();
                }
            }

            state_cloned.set(data);
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
    requirement: ladle::models::Requirement,
    refresh: Callback<()>,
}

#[derive(PartialEq, Clone, Default, Debug)]
struct RequirementEditItemState {
    quantity_buffer: String,
}

#[function_component(RequirementEditItem)]
fn requirement_edit_item(props: &RequirementEditItemProps) -> Html {
    let state = use_state(|| RequirementEditItemState {
        quantity_buffer: props.requirement.quantity.clone(),
    });

    if props.requirement.quantity != state.quantity_buffer {
        state.set(RequirementEditItemState {
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
                state_cloned.quantity_buffer.as_str(),
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

#[derive(Properties, PartialEq, Clone)]
struct DependencyEditItemProps {
    dependency: ladle::models::RecipeIndex,
    refresh: Callback<()>,
}

#[function_component(DependencyEditItem)]
fn dependency_edit_item(props: &DependencyEditItemProps) -> Html {
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let on_dependency_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::recipe_unlink(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                props_cloned.dependency.id.as_str(),
            )
            .await
            {
                Ok(()) => props_cloned.refresh.emit(()),
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
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
struct DependencyAddItemProps {
    refresh: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct DependencyAddItemState {
    dependency_id_buffer: Option<String>,
    choices: Vec<ladle::models::RecipeIndex>,
}

#[function_component(DependencyAddItem)]
fn dependency_add_item(props: &TagAddItemProps) -> Html {
    let state = use_state(|| DependencyAddItemState {
        dependency_id_buffer: None,
        choices: vec![],
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let refresh_selection = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::recipe_index(context_cloned.server.as_str(), "").await {
                Ok(mut recipes) => {
                    recipes.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.choices = recipes;
                    state_cloned.set(data);
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }
        })
    });

    use_effect_with_deps(move |_| refresh_selection.emit(()), props.clone());

    let state_cloned = state.clone();
    let on_dependency_select = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.dependency_id_buffer = Some(target.unchecked_into::<HtmlInputElement>().value());
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let create_dependency = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::recipe_link(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                data.dependency_id_buffer.clone().unwrap().as_str(),
            )
            .await
            {
                Ok(_) => {
                    props_cloned.refresh.emit(());
                    data.dependency_id_buffer = None;
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }

            state_cloned.set(data);
        });
    });

    let options = state
        .choices
        .iter()
        .map(|r| html! {<option value={r.id.clone()}>{r.name.clone()}</option>})
        .collect::<Html>();

    html! {
        <li key={"dependency_add"}>
            <select onchange={on_dependency_select}>
            <option hidden={true} disabled={true} selected={true}>{"Recipes"}</option>
            {options}
            </select>
            <button onclick={create_dependency}>{"Add"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TagEditItemProps {
    label: ladle::models::LabelIndex,
    refresh: Callback<()>,
}

#[function_component(TagEditItem)]
fn tag_edit_item(props: &TagEditItemProps) -> Html {
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let on_tag_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::recipe_untag(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                props_cloned.label.id.as_str(),
            )
            .await
            {
                Ok(_) => props_cloned.refresh.emit(()),
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }
        });
    });

    html! {
        <li key={props.label.id.as_str()}>
            <span>{props.label.name.as_str()}</span>
            <button onclick={on_tag_delete}>{"Delete"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TagAddItemProps {
    refresh: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct TagAddItemState {
    label_name_buffer: String,
}

#[function_component(TagAddItem)]
fn tag_add_item(props: &TagAddItemProps) -> Html {
    let state = use_state(|| TagAddItemState {
        label_name_buffer: String::default(),
    });
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let on_label_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.label_name_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let create_tag = Callback::from(move |_| {
        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        let props_cloned = props_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();
            match ladle::recipe_tag(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                data.label_name_buffer.as_str(),
            )
            .await
            {
                Ok(_) => {
                    props_cloned.refresh.emit(());
                    data.label_name_buffer = String::default();
                }
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }

            state_cloned.set(data);
        });
    });

    html! {
        <li key={"tag_add"}>
            <input
                type="text"
                value={(*state).label_name_buffer.clone()}
                onchange={on_label_edit}
            />
            <button onclick={create_tag}>{"Add"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeEditWindowProps {
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

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

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
    let context_cloned = context.clone();
    let refresh_recipe: Callback<()> = Callback::from(move |_| {
        let recipe_id = context_cloned.recipe_id.clone().unwrap_or_else(|| {
            context_cloned.status.emit(Message::Info(
                String::from("Attempted to edit empty recipe"),
                chrono::Utc::now(),
            ));
            String::default()
        });

        let state_cloned = state_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = state_cloned.deref().clone();

            match ladle::recipe_get(context_cloned.server.as_str(), recipe_id.as_str()).await {
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
                let params = HashMap::from([("name", state_cloned.name_buffer.as_str())]);
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(context_cloned.server.as_str(), id.as_str(), params)
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
                let params = HashMap::from([("author", state_cloned.author_buffer.as_str())]);
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(context_cloned.server.as_str(), id.as_str(), params)
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
                let params =
                    HashMap::from([("directions", state_cloned.directions_buffer.as_str())]);
                let id = state_cloned.recipe.clone().unwrap().id.clone();
                match ladle::recipe_update(context_cloned.server.as_str(), id.as_str(), params)
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
    let on_delete: Callback<()> = Callback::from(move |_| {
        let context_cloned = context_cloned.clone();
        let state_cloned = state_cloned.clone();

        if let Some(_recipe) = state_cloned.recipe.clone() {
            wasm_bindgen_futures::spawn_local(async move {
                /*
                if let Err(message) =
                    ladle::recipe_delete(context.server.as_str(), &recipe.id).await
                {
                    props_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()))
                }

                props_cloned.on_delete.emit(());
                */

                context_cloned.status.emit(Message::Info(
                    String::from("Recipe deletion disabled"),
                    chrono::Utc::now(),
                ));
            });
        }
    });

    let refresh_recipe_cloned = refresh_recipe.clone();
    use_effect_with_deps(move |_| refresh_recipe_cloned.emit(()), props.clone());

    let props_cloned = props.clone();
    let on_click_edit = Callback::from(move |_| props_cloned.set_edition.emit(false));

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

        html! {
            <div class="recipe-display edit">
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
                <ul>
                    {dependencies}
                    <DependencyAddItem
                        refresh={refresh_recipe.clone()}
                    />
                </ul>
                <ul>
                    {requirements}
                    <RequirementAddItem
                        refresh={refresh_recipe.clone()}
                    />
                </ul>
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
                    <button onclick={on_click_edit}>{"Done"}</button>
                    <button onclick={move |_| {on_delete.emit(())}}>{"Delete"}</button>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
