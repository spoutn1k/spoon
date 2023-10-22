use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TagEditItemProps {
    pub label: ladle::models::LabelIndex,
    pub delete_tag: Callback<String, ()>,
}

#[function_component(TagEditItem)]
pub fn tag_edit_item(props: &TagEditItemProps) -> Html {
    let props_cloned = props.clone();
    let on_tag_delete = Callback::from(move |_| {
        props_cloned
            .delete_tag
            .emit(props_cloned.label.name.clone())
    });

    html! {
        <li key={props.label.id.as_str()}>
            <span>{props.label.name.as_str()}</span>
            <button onclick={on_tag_delete}>{"Delete"}</button>
        </li>
    }
}
