use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TagAddItemProps {
    pub add_tag: Callback<String, ()>,
}

#[derive(PartialEq, Clone, Default)]
struct TagAddItemState {
    label_name_buffer: String,
}

#[function_component(TagAddItem)]
pub fn tag_add_item(props: &TagAddItemProps) -> Html {
    let state = use_state(TagAddItemState::default);

    let state_cloned = state.clone();
    let on_label_edit = Callback::from(move |e: Event| {
        let mut data = state_cloned.deref().clone();
        let target: EventTarget = e.target().expect("");
        data.label_name_buffer = target.unchecked_into::<HtmlInputElement>().value();
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let props_cloned = props.clone();
    let create_tag = Callback::from(move |_| {
        props_cloned
            .add_tag
            .emit(state_cloned.label_name_buffer.clone());
        state_cloned.set(TagAddItemState {
            label_name_buffer: String::default(),
        });
    });

    html! {
        <li key={"tag_add"}>
            <input
                type="text"
                value={(*state).label_name_buffer.clone()}
                onchange={on_label_edit}
            />
            <button onclick={create_tag}>{"Add"}</button>
        </li>
    }
}
