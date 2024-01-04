use leptos::*;

// mod table;

use crate::components::table::Table;
use crate::components::controls::Controls;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center flex items-center justify-center h-1/2">
            <div class="flex flex-col gap-3 max-h-80">
                <Table  />
                <Controls />
            </div>
        </div>
    }
}
