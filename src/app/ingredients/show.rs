use crate::app::{AppContext, Message, Route};
use ladle::models::{Ingredient, RecipeIndex};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientViewProps {
    pub ingredient_id: Option<String>,
}

struct IngredientViewState {
    ingredient: Option<Ingredient>,
}

fn render_ingredient(data: &Ingredient) -> Html {
    let recipes = data
        .used_in
        .iter()
        .map(|r: &RecipeIndex| {
            html! {
                <li>
                    <Link<Route> to={Route::ShowRecipe{id:r.id.clone()}}>
                        {r.name.as_str()}
                    </Link<Route>>
                </li>
            }
        })
        .collect::<Html>();

    let trail = match data.used_in.len() {
        0 => ".",
        1 => ":",
        _ => "s:",
    };

    html! {
        <div>
            <h1>{data.name.as_str()}</h1>
            <h3>{format!("Used in {} recipe{}", data.used_in.len(), trail)}</h3>
            <ul>
                {recipes}
            </ul>
        </div>
    }
}

#[function_component(IngredientView)]
pub fn view(props: &IngredientViewProps) -> Html {
    let state = use_state(|| IngredientViewState { ingredient: None });
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
                    match ladle::ingredient_get(&context.settings.server_url, &ingredient_id).await
                    {
                        Ok(ingredient) => {
                            state_cloned.set(IngredientViewState {
                                ingredient: Some(ingredient),
                            });
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

    match &state.ingredient {
        None => html! {},
        Some(data) => render_ingredient(data),
    }
}
