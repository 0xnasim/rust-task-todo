use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub active: bool,
    pub children: Children,
    // pub on_close: Callback<bool>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html!(
        <div class={ if props.active {"fixed z-50 inset-0 overflow-y-auto block ease-out duration-300"} else {"fixed
            z-50 inset-0 overflow-y-auto hidden ease-in duration-200"} }>
            <div class="flex justify-center min-h-screen items-center">
                <div class="fixed inset-0 bg-black bg-opacity-75 transition-opacity" aria-hidden="true" />
                <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">
                </span>
                <div class="relative modal-card rounded-2xl w-2/6">
                    <div class="bg-white rounded-2xl shadow-2xl w-full px-3">
                        <div class="flex justify-between px-5 py-4 mb-3">
                            <h1 class="text-3xl font-bold text-center">{"Add Task"}</h1>
                        </div>
                        <div class="px-5 pb-10">
                            {props.children.clone()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
