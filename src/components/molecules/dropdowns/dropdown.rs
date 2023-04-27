use wasm_bindgen::JsCast;
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
    #[prop_or_default]
    pub options_reversed: bool,
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
        let options_reversed = props.options_reversed.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>();
            let select_index = i16::try_from(select.selected_index()).unwrap();
            let select_length = i16::try_from(select.length()).unwrap();

            let value = i16::try_from(select_index).unwrap();

            let value_to_emit = if options_reversed {
                select_length - select_index
            } else {
                value + 1
            };

            props_on_change.emit(value_to_emit);
        })
    };

    // TODO
    // use_effect_with_deps(
    //     move |_| {
    //         let select = select_ref
    //             .cast::<HtmlSelectElement>()
    //             .expect("select_ref not set");

    //         let listener =  Callback::from(move |event: Event| {
    //             let select = event
    //                 .target()
    //                 .unwrap()
    //                 .unchecked_into::<HtmlSelectElement>();
    //         });

    //         select.add_event_listener_with_callback("onchange", listener);
    //     },
    //     props.selected,
    // );

    html! {
        <select class="p-4 rounded-lg bg-slate-200" autocomplete="off" ref={select_ref} onchange={handle_on_change}>
            {options}
        </select>
    }
}
