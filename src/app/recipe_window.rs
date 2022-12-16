use pulldown_cmark::{html::push_html, Options, Parser};
use yew::prelude::*;
use yew::{html, AttrValue, Html};

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub url: String,
    pub recipe_id: Option<String>,
}

fn parse_text(value: &str) -> String {
    let options = Options::empty();

    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);

    parsed_text
}

fn render_recipe(data: &ladle::models::Recipe) -> Html {
    let data = data.clone();

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

    let dependencies = data
        .dependencies
        .iter()
        .map(|recipe| {
            html! {
                <li key={recipe.id.clone()}>
                    {recipe.name.clone()}
                </li>
            }
        })
        .collect::<Html>();

    let tags = data
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

    let parse_html = parse_text(&data.directions);
    let parsed = Html::from_html_unchecked(AttrValue::from(parse_html));

    html! {
        <div class="recipe-display">
            <div class="recipe-name">{data.name}</div>
            <div class="recipe-author">{data.author}</div>
            <ul>{dependencies}</ul>
            <ul>{requirements}</ul>
            <div class="recipe-directions">{parsed}</div>
            <ul>{tags}</ul>
        </div>
    }
}

#[function_component(RecipeWindow)]
pub fn recipes_window(props: &RecipeWindowProps) -> Html {
    let recipe = use_state(|| None);
    {
        let recipe = recipe.clone();
        let props_copy = props.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(id) = props_copy.recipe_id {
                    wasm_bindgen_futures::spawn_local(async move {
                        let fetched_recipe =
                            ladle::recipe_get(props_copy.url.as_str(), id.as_str()).await;

                        match fetched_recipe {
                            Some(data) => recipe.set(Some(data)),
                            None => recipe.set(None),
                        }
                    });
                }
            },
            props.clone(),
        );
    }

    match (*recipe).clone() {
        Some(data) => render_recipe(&data),
        None => html! {<span>{"No data"}</span>},
    }
}
