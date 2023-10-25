use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DependencyEditItemProps {
    pub dependency: ladle::models::Dependency,
    pub update_dependency: Callback<(String, bool)>,
    pub delete_dependency: Callback<()>,
}

#[function_component(DependencyEditItem)]
pub fn dependency_edit_item(props: &DependencyEditItemProps) -> Html {
    let props_cloned = props.clone();
    let on_quantity_edit = Callback::from(move |e: Event| {
        let quantity = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        props_cloned
            .update_dependency
            .emit((quantity, props_cloned.dependency.optional))
    });

    let props_cloned = props.clone();
    let on_optional_edit = Callback::from(move |_| {
        props_cloned.update_dependency.emit((
            props_cloned.dependency.quantity.clone(),
            !props_cloned.dependency.optional,
        ))
    });

    let props_cloned = props.clone();
    let delete_dependency = Callback::from(move |_| {
        props_cloned.delete_dependency.emit(());
    });

    html! {
        <tr key={props.dependency.recipe.id.as_str()}>
            <td>{props.dependency.recipe.name.as_str()}</td>
            <td><input
                type="text"
                value={props.dependency.quantity.clone()}
                onchange={on_quantity_edit}
            /></td>
            <td><input
                type="checkbox"
                checked={props.dependency.optional}
                onclick={on_optional_edit}
            /></td>
            <td><button onclick={delete_dependency}>{"Delete"}</button></td>
        </tr>
    }
}
