use ::leptos::*;

#[component]
pub fn Table() -> impl IntoView {
    view! {
        <table class="w-full text-sm text-left rtl:text-right text-black">
        <thead class="text-xs bg-cyan-300 uppercase text-black">
            <tr>
                <th scope="col" class="px-6 py-3">
                    Product name
                </th>
                <th scope="col" class="px-6 py-3">
                    Color
                </th>
                <th scope="col" class="px-6 py-3">
                    Category
                </th>
                <th scope="col" class="px-6 py-3">
                    Price
                </th>
            </tr>
        </thead>
        <tbody>
        <tr class="border-b bg-emerald-400 border-gray-700">
                <th scope="row" class="px-6 py-4 font-medium whitespace-nowraptext-white">
                    Apple MacBook Pro 17
                </th>
                <td class="px-6 py-4">
                    Silver
                </td>
                <td class="px-6 py-4">
                    Laptop
                </td>
                <td class="px-6 py-4">
                    $2999
                </td>
            </tr>
        </tbody>
        </table>
    }
}
