use crate::app::recipes::list::Filters;
use crate::app::Route;
use std::collections::HashSet;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchPaneProps {
    pub labels: HashSet<ladle::models::LabelIndex>,
    pub change_pattern: Callback<String>,
    pub selected_labels: HashSet<String>,
}

#[derive(PartialEq, Clone, Default)]
struct SearchPaneState {
    label_tray_shown: bool,
}

#[function_component(SearchPane)]
pub fn search_pane(props: &SearchPaneProps) -> Html {
    let state = use_state(|| SearchPaneState::default());
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let parameters = location.query::<Filters>().unwrap_or(Filters::default());

    let cloned_state = state.clone();
    let toggle_tray = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.label_tray_shown = !data.label_tray_shown;
        cloned_state.set(data);
    });
    let mut filters_avail: Vec<ladle::models::LabelIndex> = props.labels.iter().cloned().collect();
    filters_avail.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
    let filters_avail = filters_avail
        .iter()
        .cloned()
        .map(|l| {
            let element_props = props.clone();
            let nc = navigator.clone();
            let lc = l.clone();
            let selected = parameters.labels.clone();
            if element_props.selected_labels.contains(&l.name) {
                html! {
                        <li
                            key={l.id.as_str()}
                            class="label filter remove"
                            onclick={Callback::from(move |_| {
                    let mut new_labels = selected.clone();
                    if let Some(pos) = new_labels.iter().position(|x| *x == lc.name) {
                        new_labels.remove(pos);
                    }

                    let _ = nc.push_with_query(
                        &Route::ListRecipes,
                        &Filters {
                            labels: new_labels,
                            restrictions: String::from(""),
                            name: String::from(""),
                        },
                    );
                })}
                        >{
                            l.name.clone()
                        }</li>
                    }
            } else {
                html! {
                        <li
                            key={l.id.as_str()}
                            class="label filter add"
                            onclick={Callback::from(move |_| {
                    let mut new_labels = selected.clone();
                    new_labels.push(lc.name.clone());

                    let _ = nc.push_with_query(
                        &Route::ListRecipes,
                        &Filters {
                            labels: new_labels,
                            restrictions: String::from(""),
                            name: String::from(""),
                        },
                    );
                })}
                        >{
                            l.name.clone()
                        }</li>
                    }
            }
        })
        .collect::<Html>();

    let props_cloned = props.clone();
    let on_pattern_change = Callback::from(move |e: InputEvent| {
        let pattern = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();

        props_cloned.change_pattern.emit(pattern);
    });

    html! {
        <div class="search-pane">
            <div class="search-header">
                <input
                    type="search"
                    class="search-bar"
                    placeholder="Search recipes ..."
                    oninput={on_pattern_change} />
                <button class="label-tray-toggle" onclick={toggle_tray}>{"labels"}</button>
                <div class="restrictions">
                    <span>
                        <input type="checkbox" name="vegetarian" value="0" />
                        <label for="vegetarian">{"Vegetarian"}</label>
                    </span>
                    <span>
                        <input type="checkbox" name="vegan" value="0" />
                        <label for="vegan">{"Vegan"}</label>
                    </span>
                    <span>
                        <input type="checkbox" name="dairy-free" value="0" />
                        <label for="dairy-free">{"Dairy-Free"}</label>
                    </span>
                    <span>
                        <input type="checkbox" name="gluten-free" value="0" />
                        <label for="gluten-free">{"Gluten-Free"}</label>
                    </span>
                </div>
            </div>
            <ul class={"available-labels hidden"}>
                {filters_avail}
            </ul>
        </div>
    }
}
