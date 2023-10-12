use crate::app::ingredients::list_item::IngredientListItem;
use crate::app::{set_title, AppContext};
use unidecode::unidecode;
use yew::prelude::*;

#[function_component(IngredientList)]
pub fn list() -> Html {
    set_title("Ingredients - spoon");

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let mut items: Vec<_> = context.ingredient_cache.iter().collect();
    items.sort_by(|lhs, rhs| unidecode(&lhs.name).cmp(&unidecode(&rhs.name)));

    let items = items
        .iter()
        .map(|i| html! {<IngredientListItem data={(*i).clone()}/>})
        .collect::<Html>();

    return html! {<ul class={"ingredient-list"}>{items}</ul>};
}
