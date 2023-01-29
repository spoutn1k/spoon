use crate::app::status_bar::Message;
use crate::app::AppContext;
use crate::app::Route;
use futures::future::join_all;
use pulldown_cmark::{html::push_html, Options, Parser};
use yew::prelude::*;
use yew::{html, AttrValue, Html};
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
            <h3 class="dependency-header">{format!("Pour {}:", data.name)}</h3>
            <ul class="requirement-list">{requirements}</ul>
        </li>
    }
}

fn render_directions(data: &ladle::models::Recipe) -> Html {
    let parse_html = parse_text(&data.directions);
    let parsed = Html::from_html_unchecked(AttrValue::from(parse_html));

    html! {
        {parsed}
    }
}

fn render_recipe(data: &Vec<ladle::models::Recipe>) -> Html {
    let main_recipe = data.first().unwrap();
    let requirements = data.iter().rev().map(render_requirements).collect::<Html>();
    let directions = data.iter().rev().map(render_directions).collect::<Html>();

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
            <ul class="recipe-ingredients">{requirements}</ul>
            <h2 class="recipe-directions-label">{"Préparation"}</h2>
            <div class="recipe-directions">{directions}</div>
            </>
    }
}

fn calc_missing(list: &Vec<ladle::models::Recipe>) -> Vec<&str> {
    let fetched = list.iter().map(|r| r.id.as_str()).collect::<Vec<&str>>();
    list.iter()
        .flat_map(|r| r.dependencies.iter().map(|d| d.recipe.id.as_str()))
        .filter(|id| !fetched.contains(id))
        .collect()
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub recipe_id: Option<String>,
}

#[function_component(RecipeWindow)]
pub fn recipe_window(props: &RecipeWindowProps) -> Html {
    let recipe_set = use_state(|| vec![]);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let recipe_set_cloned = recipe_set.clone();
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    use_effect_with_deps(
        move |_| {
            let recipe_set_cloned = recipe_set_cloned.clone();
            let id = props_cloned.recipe_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut recipes: Vec<ladle::models::Recipe> = vec![];

                if let Some(id) = id {
                    match ladle::recipe_get(context_cloned.server.as_str(), id.as_str()).await {
                        Ok(recipe) => recipes.push(recipe),
                        Err(message) => context
                            .status
                            .emit(Message::Error(message.to_string(), chrono::Utc::now())),
                    }

                    loop {
                        let missing = calc_missing(&recipes);

                        if missing.len() == 0 {
                            break;
                        }

                        let fetches = missing
                            .iter()
                            .map(|id| ladle::recipe_get(context_cloned.server.as_str(), id));

                        join_all(fetches)
                            .await
                            .iter()
                            .for_each(|recipe_opt| match recipe_opt {
                                Ok(recipe) => recipes.push(recipe.clone()),
                                Err(_) => (),
                            });
                    }
                }

                recipe_set_cloned.set(recipes)
            });
        },
        props.recipe_id.clone(),
    );

    let class;
    let recipe_html;
    let options;

    if recipe_set.len() == 0 || props.recipe_id.is_none() {
        class = "recipe-display empty";
        recipe_html = html! {
                <span>{"No data"}</span>
        };
        options = html! {};
    } else {
        class = "recipe-display filled";
        recipe_html = render_recipe(&recipe_set);
        options = html! {<div class="options">
            <Link<Route>
                classes={classes!("recipe-edit")}
                to={Route::EditRecipe{id: props.recipe_id.clone().unwrap()}}>
                {"Edit"}
            </Link<Route>>
            <Link<Route>
                classes={classes!("recipe-deselect")}
                to={Route::ListRecipes}>
                {"Close"}
            </Link<Route>>
        </div>};
    };

    html! {
    <div {class}>
        {recipe_html}
        {options}
    </div>}
}