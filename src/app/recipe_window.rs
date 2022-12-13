use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeWindowProps {
    pub url: String,
    pub recipe_id: Option<String>,
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
        Some(data) => html! {
            <>
                <div>{data.name}</div>
            </>
        },
        None => html! {<span>{"No data"}</span>},
    }
}
