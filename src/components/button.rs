use yew::{function_component, html, Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub name: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button type="button" onclick={props.on_click.clone()} class="bg-fuchsia-800 py-2 px-10 flex justify-center item-center font-semibold rounded text-white">
            {&props.name}
        </button>
    }
}
