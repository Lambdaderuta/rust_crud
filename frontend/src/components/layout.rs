use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col h-dvh">
            <header class="h-14 bg-gradient-to-r from-sky-500 to-indigo-500"></header>
            <main class="grow bg-sky-200">
                {children()}
            </main>
            <footer class=""></footer>
        </div>
    }
}
