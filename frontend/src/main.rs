use leptos::*;

mod app;
mod components;
mod pages;

use app::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}
