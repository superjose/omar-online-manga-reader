use gloo::console::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum SelectOptionValue {
    Int(i16),
    // String(String),
}

#[derive(PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: SelectOptionValue,
}

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    #[prop_or(Callback::noop())]
    pub onchange: Callback<i16>,
    pub options: Vec<SelectOption>,
    pub selected: i16,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    // let on_change = props.on_change.clone();
    let select_ref = use_node_ref();
    let options = props
        .options
        .iter()
        .map(|option| {
            let value = match &option.value {
                SelectOptionValue::Int(v) => v.to_string(),
                // SelectOptionValue::String(s) => s.clone().to_string(),
            };
            log!(
                "Selected is {} {}",
                props.selected.clone().to_string(),
                value.clone()
            );
            html! {
                <option value={value.to_string()} selected={props.selected.to_string() == value}
                    >
                    {&option.label}
                </option>
            }
        })
        .collect::<Html>();
    let handle_on_change = {
        let props_on_change = props.onchange.clone();
        Callback::from(move |event: Event| {
            let index = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .selected_index();

            let value = i16::try_from(index).unwrap();

            log!("Value is {}", value.to_string());
            props_on_change.emit(value + 1);
        })
    };

    html! {
        <select autocomplete="off" ref={select_ref} onchange={handle_on_change}>
            {options}
        </select>
    }
}
