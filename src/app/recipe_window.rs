use crate::app::status_bar::Message;
use crate::app::AppContext;
use futures::future::join_all;
use pulldown_cmark::{html::push_html, Options, Parser};
use yew::prelude::*;
use yew::{html, AttrValue, Html};

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
                <li key={label.id.clone()}>
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
    list.iter()
        .flat_map(|r| r.dependencies.iter().map(|d| d.id.as_str()))
        .filter(|id| {
            let fetched = list.iter().map(|r| r.id.as_str()).collect::<Vec<&str>>();
            !fetched.contains(id)
        })
        .collect()
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub set_edition: Callback<bool>,
    pub deselect: Callback<()>,
}

#[function_component(RecipeWindow)]
pub fn recipes_window(props: &RecipeWindowProps) -> Html {
    let recipe_set = use_state(|| vec![]);

    let context = use_context::<AppContext>().unwrap_or(AppContext {
        server: "".to_string(),
        recipe_id: None,
        status: Callback::from(|_| {}),
    });

    let recipe_set_cloned = recipe_set.clone();
    use_effect_with_deps(
        move |_| {
            let recipe_set_cloned = recipe_set_cloned.clone();
            if let Some(id) = context.recipe_id.clone() {
                let recipe_init = recipe_set_cloned.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut recipes: Vec<ladle::models::Recipe> = vec![];

                    match ladle::recipe_get(context.server.as_str(), id.as_str()).await {
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
                            .map(|id| ladle::recipe_get(context.server.as_str(), id));

                        join_all(fetches)
                            .await
                            .iter()
                            .for_each(|recipe_opt| match recipe_opt {
                                Ok(recipe) => recipes.push(recipe.clone()),
                                Err(_) => (),
                            });
                    }

                    recipe_init.set(recipes)
                });
            } else {
                recipe_set_cloned.set(vec![]);
            }
        },
        props.clone(),
    );

    let empty = (*recipe_set).len() == 0;

    let class;
    let recipe_html;
    let options;

    let props_cloned = props.clone();
    if empty {
        class = "recipe-display empty";
        recipe_html = html! {
                <span>{"No data"}</span>
        };
        options = html! {};
    } else {
        class = "recipe-display filled";
        recipe_html = render_recipe(&recipe_set);
        options = html! {<div class="options">
            <button
                class="recipe-edit"
                onclick={move |_| props_cloned.set_edition.emit(true)}
            >{"Edit"}</button>
            <button
                class="recipe-deselect"
                onclick={move |_| props_cloned.deselect.emit(())}
            >{"Close"}</button>
        </div>};
    };

    html! {
    <div {class}>
        {recipe_html}
        {options}
    </div>}
}
