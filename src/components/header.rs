use crate::components::button::Button;
use yew::{function_component, html, Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header class="header sticky top-0 bg-white shadow-md flex items-center justify-between px-8 py-02">
            <nav class="nav font-semibold text-lg">
                <ul class="flex items-center">
                    <li
                        class="p-4 border-b-2 border-green-500 border-opacity-0 hover:border-opacity-100 hover:text-green-500 duration-200 cursor-pointer active">
                        <h1 class="text-2xl font-semibold">{"Task List"}</h1>
                    </li>
                </ul>
            </nav>
            <div><Button name="New Task" on_click={props.on_click.clone()} /></div>
        </header>
    }
}
