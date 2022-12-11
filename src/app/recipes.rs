use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipesListProps {
    pub url: String,
    #[prop_or("".to_string())]
    pub pattern: String,
}

#[function_component(RecipesList)]
pub fn recipes_list(props: &RecipesListProps) -> Html {
    let recipes = use_state(|| vec![]);
    {
        let recipes = recipes.clone();
        let props = props.clone();
        use_effect_with_deps(
            move |_| {
                let recipes = recipes.clone();
                let props = props.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_recipes: Vec<ladle::models::Recipe> =
                        ladle::recipe_index(props.url.as_str(), props.pattern.as_str())
                            .await
                            .unwrap();
                    recipes.set(fetched_recipes);
                });
                || ()
            },
            (),
        );
    }

    let items = recipes
        .iter()
        .map(|recipe| {
            html! {
                <li key={recipe.id.as_str()}>{format!("{}", recipe.name)}</li>
            }
        })
        .collect::<Html>();

    html! {
        <ul>
            {items}
        </ul>
    }
}
