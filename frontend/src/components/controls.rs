use ::leptos::*;

#[component]
pub fn Controls() -> impl IntoView {
    view! {
        <div class="flex w-80">
        <button class="py-2 px-1 w6 text-center w-32 h-12 bg-cyan-800 rounded-md hover:bg-cyan-600">
            Добавить
        </button>
        </div>
    }
}
