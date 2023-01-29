use crate::app::Route;
use ladle::models::IngredientIndex;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientListItemProps {
    pub data: IngredientIndex,
}

#[function_component(IngredientListItem)]
pub fn item(props: &IngredientListItemProps) -> Html {
    html! {
        <li>
            <Link<Route> to={Route::ShowIngredient{id:props.data.id.clone()}}>
                {props.data.name.as_str()}
            </Link<Route>>
        </li>
    }
}
