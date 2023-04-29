use yew::prelude::*;

use crate::components::{
    atoms::buttons::button::ButtonVariant, atoms::icon::Icon,
    molecules::icon_button::icon_button::IconButton,
};

#[function_component(SettingsDropdown)]
pub fn settings_dropdown() -> Html {
    html! {
        <IconButton
                class="absolute top-2 right-2"
                icon={Icon::Settings}
                variant={ButtonVariant::Secondary}
            />
    }
}
