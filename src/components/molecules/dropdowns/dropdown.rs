use log::log;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum SelectOptionValue {
    Int(i32),
    String(String),
}

#[derive(PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: SelectOptionValue,
}

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    #[prop_or(Callback::noop())]
    pub on_change: Callback<Event>,
    pub options: Vec<SelectOption>,
    pub selected: i32,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    // let on_change = props.on_change.clone();
    let options = props
        .options
        .iter()
        .map(|option| {
            let value = match &option.value {
                SelectOptionValue::Int(v) => v.to_string(),
                SelectOptionValue::String(s) => s.clone().to_string(),
            };
            log::info!("Value is {}", value == props.selected.to_string());
            html! {
                <option value={value.to_string()} selected={props.selected.to_string() == value}>
                    {&option.label}
                </option>
            }
        })
        .collect::<Html>();

    html! {
        <select>
            {options}
        </select>
    }
}
