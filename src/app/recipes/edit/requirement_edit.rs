use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RequirementEditItemProps {
    pub requirement: ladle::models::Requirement,
    pub update_requirement: Callback<(String, bool), ()>,
    pub delete_requirement: Callback<()>,
}

#[derive(PartialEq, Clone, Default, Debug)]
struct RequirementEditItemState {}

#[function_component(RequirementEditItem)]
pub fn requirement_edit_item(props: &RequirementEditItemProps) -> Html {
    let props_cloned = props.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let quantity = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        props_cloned
            .update_requirement
            .emit((quantity, props_cloned.requirement.optional))
    });

    let props_cloned = props.clone();
    let on_optional_edit = Callback::from(move |_| {
        props_cloned.update_requirement.emit((
            props_cloned.requirement.quantity.clone(),
            !props_cloned.requirement.optional,
        ))
    });

    let props_cloned = props.clone();
    let delete_requirement = Callback::from(move |_| {
        props_cloned.delete_requirement.emit(());
    });

    html! {
        <tr key={props.requirement.ingredient.id.as_str()}>
            <td>{props.requirement.ingredient.name.as_str()}</td>
            <td><input
                type="text"
                value={props.requirement.quantity.clone()}
                onchange={on_quantity_edit}
            /></td>
            <td><input
                type="checkbox"
                checked={props.requirement.optional}
                onclick={on_optional_edit}
            /></td>
            <td><button onclick={delete_requirement}>{"Delete"}</button></td>
        </tr>
    }
}
