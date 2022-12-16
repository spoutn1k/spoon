use crate::app::status_bar::Message;
use std::ops::Deref;
use yew::prelude::*;

fn render_recipe(data: &ladle::models::Recipe, edit: Callback<bool>, delete: Callback<()>) -> Html {
    let on_click_edit = Callback::from(move |_| edit.emit(false));
    let on_click_delete = Callback::from(move |_| delete.emit(()));

    html! {
        <div class="recipe-display edit">
            <div class="recipe-name">{data.name.as_str()}</div>
            <div class="options">
                <button onclick={on_click_edit}>{"Done"}</button>
                <button onclick={on_click_delete}>{"Delete"}</button>
            </div>
        </div>
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

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeEditWindowState {
    pub recipe: Option<ladle::models::Recipe>,
    pub name_buffer: String,
}

#[function_component(RecipeEditWindow)]
pub fn recipe_edit_window(props: &RecipeEditWindowProps) -> Html {
    let state = use_state(|| RecipeEditWindowState {
        recipe: None,
        name_buffer: String::default(),
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
                        Ok(recipe) => data.recipe = Some(recipe),
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

    match (*state).recipe.clone() {
        Some(data) => render_recipe(&data, props.set_edition.clone(), props.on_delete.clone()),
        None => html! {<span>{"No data"}</span>},
    }
}
