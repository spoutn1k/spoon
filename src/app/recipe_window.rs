use crate::app::status_bar::Message;
use futures::future::join_all;
use pulldown_cmark::{html::push_html, Options, Parser};
use yew::prelude::*;
use yew::{html, AttrValue, Html};

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub url: String,
    pub recipe_id: Option<String>,
    pub status: Callback<Message>,
    pub set_edition: Callback<bool>,
}

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
                <li key={requirement.ingredient.id.clone()}>
                    <span class="requirement-ingredient">{requirement.ingredient.name.clone()}</span>
                    <span class="requirement-quantity">{requirement.quantity.clone()}</span>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <li key={data.id.as_str()}>
            <p>{format!("Pour {}:", data.name)}</p>
            <ul>{requirements}</ul>
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

fn render_recipe(data: &Vec<ladle::models::Recipe>, edit: &Callback<bool>) -> Html {
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

    let edit = edit.clone();
    let on_click_edit = Callback::from(move |_| edit.emit(true));

    html! {
        <div class="recipe-display">
            <div class="recipe-name">{main_recipe.name.as_str()}</div>
            <div class="recipe-author">{main_recipe.author.as_str()}</div>
            <ul>{requirements}</ul>
            <div class="recipe-directions">{directions}</div>
            <ul>{tags}</ul>
            <div class="options">
                <button onclick={on_click_edit}>{"Edit"}</button>
            </div>
        </div>
    }
}

fn calc_missing<'a>(list: &'a Vec<ladle::models::Recipe>) -> Vec<&'a str> {
    list.iter()
        .flat_map(|r| r.dependencies.iter().map(|d| d.id.as_str()))
        .filter(|id| {
            let fetched = list.iter().map(|r| r.id.as_str()).collect::<Vec<&str>>();
            !fetched.contains(id)
        })
        .collect::<Vec<&str>>()
}

#[function_component(RecipeWindow)]
pub fn recipes_window(props: &RecipeWindowProps) -> Html {
    let recipe = use_state(|| vec![]);

    let recipe_cloned = recipe.clone();
    let props_copy = props.clone();
    use_effect_with_deps(
        move |_| {
            let recipe_cloned = recipe_cloned.clone();
            if let Some(id) = props_copy.recipe_id.clone() {
                let recipe_init = recipe_cloned.clone();
                let props_cloned = props_copy.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut recipes: Vec<ladle::models::Recipe> = vec![];

                    match ladle::recipe_get(props_cloned.url.as_str(), id.as_str()).await {
                        Ok(recipe) => recipes.push(recipe),
                        Err(message) => props_cloned
                            .status
                            .emit(Message::Error(message.to_string())),
                    }

                    loop {
                        let missing = calc_missing(&recipes);

                        if missing.len() == 0 {
                            break;
                        }

                        let fetches = missing
                            .iter()
                            .map(|id| ladle::recipe_get(props_cloned.url.as_str(), id));

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
            }
        },
        props.clone(),
    );

    match (*recipe).len() {
        0 => html! {<span>{"No data"}</span>},
        _ => render_recipe(&recipe, &props.set_edition),
    }
}
