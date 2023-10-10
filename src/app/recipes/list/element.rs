use crate::app::Route;
use ladle::models::RecipeIndex;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeElementProps {
    pub item: RecipeIndex,
}

#[function_component(RecipeElement)]
pub fn recipe_element(
    RecipeElementProps {
        item: RecipeIndex { id, name },
    }: &RecipeElementProps,
) -> Html {
    html! {
        <li class={"recipe-item"} key={id.as_str()}>
            <Link<Route> to={Route::ShowRecipe {id:id.clone()}}>
                {name}
            </Link<Route>>
            <span class={"knife-id"}>{id.as_str()}</span>
        </li>
    }
}
