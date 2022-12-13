use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeElementProps {
    id: String,
    name: String,
    on_click: Callback<String>,
}

#[function_component(RecipeElement)]
pub fn recipe_element(RecipeElementProps { id, name, on_click }: &RecipeElementProps) -> Html {
    let on_recipe_select = {
        let on_click = on_click.clone();
        let id = id.clone();
        Callback::from(move |_| on_click.emit(id.clone()))
    };
    html! {
        <li key={String::from(id)} onclick={on_recipe_select}>{name}</li>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListProps {
    pub url: String,
    pub on_click: Callback<String>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &RecipeListProps) -> Html {
    let pattern_handle = use_state(|| String::from(""));
    let pattern: String = (*pattern_handle).clone();

    let on_pattern_change = Callback::from(move |e: InputEvent| {
        let target: EventTarget = e.target().expect("");
        pattern_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    let recipes = use_state(|| vec![]);
    {
        let recipes = recipes.clone();
        let url: String = props.url.clone();
        let pattern = pattern.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_recipes = ladle::recipe_index(url.as_str(), pattern.as_str()).await;

                    match fetched_recipes {
                        Some(list) => recipes.set(list),
                        None => recipes.set(vec![]),
                    }
                });
            },
            props.url.clone(),
        );
    }

    let items = recipes
        .iter()
        .filter(|recipe| {
            unidecode::unidecode(&recipe.name.to_lowercase()).contains(&pattern.to_lowercase())
        })
        .map(|recipe| {
            let recipe = recipe.clone();
            html! {
                <RecipeElement id={recipe.id} name={recipe.name}  on_click={props.on_click.clone()}/>
            }
        })
        .collect::<Html>();

    html! {
        <>
        <input type="text"
            oninput={on_pattern_change}
            value={pattern}
        />
        <ul>
            {items}
        </ul>
        </>
    }
}
