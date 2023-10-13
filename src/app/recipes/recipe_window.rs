use crate::app::{set_title, status_bar::Message, AppContext, Route};
use futures::future::join_all;
use pulldown_cmark::{html::push_html, Options, Parser};
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

fn parse_text(value: &str) -> String {
    let options = Options::empty();

    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);

    parsed_text
}

fn render_requirements(data: &ladle::models::Recipe) -> Html {
    let requirements = data
        .requirements
        .iter()
        .map(|requirement| {
            html! {
                <li class="requirement" key={requirement.ingredient.id.clone()}>
                    <span class="requirement-ingredient">{requirement.ingredient.name.clone()}</span>
                    <span class="requirement-quantity">{requirement.quantity.clone()}</span>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <li class="dependency-requirement" key={data.id.as_str()}>
            <h3 class="dependency-subtitle">{data.name.clone()}</h3>
            <ul class="requirement-list">{requirements}</ul>
        </li>
    }
}

fn render_directions(data: &ladle::models::Recipe) -> Html {
    let parse_html = parse_text(&data.directions);
    let parsed = Html::from_html_unchecked(AttrValue::from(parse_html));

    html! {
        <>
            <h3 class="dependency-subtitle">{data.name.clone()}</h3>
            {parsed}
        </>
    }
}

fn render(data: &RecipeWindowState) -> Html {
    if data.main_recipe.is_none() {
        return html! {};
    }

    let main_recipe = data.main_recipe.clone().unwrap();
    let requirements = data
        .dependencies
        .iter()
        .map(|(_, data)| render_requirements(data))
        .collect::<Html>();
    let directions = data
        .dependencies
        .iter()
        .map(|(_, data)| render_directions(data))
        .collect::<Html>();

    let tags = main_recipe
        .tags
        .iter()
        .map(|label| {
            html! {
                <li class="label" key={label.id.clone()}>
                    {label.name.clone()}
                </li>
            }
        })
        .collect::<Html>();

    html! {
            <>
            <div class="recipe-header">
                <h1 class="recipe-name">{main_recipe.name.as_str()}</h1>
                <div class="recipe-author">{main_recipe.author.as_str()}</div>
            </div>
            <ul class="recipe-tags">{tags}</ul>
            <h2 class="recipe-ingredients-label">{"Ingrédients"}</h2>
            <ul class="recipe-ingredients">{render_requirements(&main_recipe)}{requirements}</ul>
            <h2 class="recipe-directions-label">{"Préparation"}</h2>
            <div class="recipe-directions">{render_directions(&main_recipe)}{directions}</div>
            </>
    }
}

fn calc_missing(data: &RecipeWindowState) -> Vec<String> {
    let mut fifo = match &data.main_recipe {
        Some(recipe) => vec![recipe.clone()],
        None => vec![],
    };
    let mut missing = vec![];

    while let Some(recipe) = fifo.pop() {
        let local_missing =
            recipe
                .dependencies
                .iter()
                .filter_map(|dependency: &ladle::models::Dependency| {
                    let target_id = &dependency.recipe.id;
                    match data.dependencies.get(&target_id.clone()) {
                        Some(recipe) => {
                            fifo.push(recipe.clone());
                            None
                        }
                        None => Some(target_id),
                    }
                });

        missing.extend(local_missing.cloned());
    }

    missing
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub recipe_id: Option<String>,
}

enum RecipeWindowActions {
    UpdateRecipe(ladle::models::Recipe),
    UpdateDependency(ladle::models::Recipe),
}

#[derive(Clone, Default, PartialEq)]
struct RecipeWindowState {
    main_recipe: Option<ladle::models::Recipe>,
    dependencies: HashMap<String, ladle::models::Recipe>,
}

impl Reducible for RecipeWindowState {
    type Action = RecipeWindowActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();
        match action {
            RecipeWindowActions::UpdateRecipe(main) => {
                new_state.main_recipe = Some(main.clone());
            }
            RecipeWindowActions::UpdateDependency(dependency) => {
                new_state
                    .dependencies
                    .insert(dependency.id.clone(), dependency);
            }
        };

        new_state.into()
    }
}

#[function_component(RecipeWindow)]
pub fn recipe_window(props: &RecipeWindowProps) -> Html {
    let navigator = use_navigator().unwrap();
    let state = use_reducer(RecipeWindowState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    use_effect_with_deps(
        move |_| {
            let state_cloned = state_cloned.clone();
            let id = props_cloned.recipe_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(id) = id {
                    match ladle::recipe_get(
                        context_cloned.settings.server_url.as_str(),
                        id.as_str(),
                    )
                    .await
                    {
                        Ok(recipe) => {
                            state_cloned.dispatch(RecipeWindowActions::UpdateRecipe(recipe.clone()))
                        }
                        Err(message) => context_cloned
                            .status
                            .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                    }
                }
            })
        },
        props.recipe_id.clone(),
    );

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    use_effect_with_deps(
        move |_| {
            let state_cloned = state_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let missing = calc_missing(&state_cloned);

                let fetches = missing
                    .iter()
                    .map(|id| ladle::recipe_get(context_cloned.settings.server_url.as_str(), id));

                join_all(fetches)
                    .await
                    .iter()
                    .for_each(|recipe_opt| match recipe_opt {
                        Ok(recipe) => state_cloned
                            .dispatch(RecipeWindowActions::UpdateDependency(recipe.clone())),
                        Err(_) => (),
                    });
            });
        },
        state.clone(),
    );

    let class;
    let recipe_html;
    let options;
    let nc = navigator.clone();

    if props.recipe_id.is_none() {
        class = "recipe-display empty";
        recipe_html = html! {
                <span>{"No data"}</span>
        };
        options = html! {};
    } else {
        let name = match &state.main_recipe {
            Some(data) => data.name.as_str(),
            None => "Loading",
        };
        set_title(&format!("{} - spoon", name));

        class = "recipe-display filled";
        recipe_html = render(&state);
        options = html! {<div class="options">
            <Link<Route>
                classes={classes!("recipe-edit")}
                to={Route::EditRecipe{id: props.recipe_id.clone().unwrap()}}>
                {"Edit"}
            </Link<Route>>
            <button
                class={classes!("recipe-deselect")}
                onclick={Callback::from(move |_| {
                    nc.back();
                })}>
                {"Close"}
            </button>
        </div>};
    };

    html! {
    <div {class}>
        {recipe_html}
        {options}
    </div>}
}
