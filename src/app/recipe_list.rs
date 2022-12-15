use std::ops::Deref;
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

#[derive(Clone)]
struct RecipeCreateState {
    clicked: bool,
    recipe_name: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeCreateProps {
    pub create_recipe: Callback<String>,
}

#[function_component(RecipeCreateButton)]
pub fn recipe_create_button(RecipeCreateProps { create_recipe }: &RecipeCreateProps) -> Html {
    let state = use_state(|| RecipeCreateState {
        clicked: false,
        recipe_name: String::from(""),
    });

    let cloned_state = state.clone();
    let label_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.clicked = true;
        cloned_state.set(data)
    });

    let cloned_state = state.clone();
    let name_changed = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Error accessing pattern input");
        let name = target.unchecked_into::<HtmlInputElement>().value();
        let mut data = cloned_state.deref().clone();
        data.recipe_name = name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let create_handle = create_recipe.clone();
    let name_submit = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        create_handle.emit(data.recipe_name);
        data.clicked = false;
        data.recipe_name = String::default();
        cloned_state.set(data);
    });

    match (*state).clicked {
        false => html! {
            <li key="new" onclick={label_clicked}>
                {"Add recipe"}
            </li>
        },
        true => html! {
            <li key="new">
                <input type="text" value={state.recipe_name.clone()} onchange={name_changed} />
                <input type="submit" onclick={name_submit}/>
            </li>
        },
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListProps {
    pub url: String,
    pub on_click: Callback<String>,
    pub create_recipe: Callback<String>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &RecipeListProps) -> Html {
    let pattern_handle = use_state(|| String::from(""));
    let pattern: String = (*pattern_handle).clone();

    let on_pattern_change = Callback::from(move |e: InputEvent| {
        let target: EventTarget = e.target().expect("Error accessing pattern input");
        let pattern = target.unchecked_into::<HtmlInputElement>().value();
        pattern_handle.set(pattern);
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
            <RecipeCreateButton create_recipe={props.create_recipe.clone()}/>
        </ul>
        </>
    }
}
