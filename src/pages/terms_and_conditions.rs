use crate::components::{Footer, Header};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use crate::context::provide_theme_context;

#[component]
pub fn TermsAndConditions() -> impl IntoView {
    let theme_context = provide_theme_context();
    let is_dark = theme_context.is_dark;

    view! {
        <div 
            class="min-h-screen"
            class=("bg-gray-50 text-gray-900", move || !is_dark.get())
            class=("bg-gray-900 text-gray-200", move || is_dark.get())
        >
            <Header />
            <main 
                class="container mx-auto max-w-3xl py-12 px-6 sm:px-8 shadow-lg rounded-lg"
                class=("bg-white", move || !is_dark.get())
                class=("bg-gray-900", move || is_dark.get())
            >
                <h1 class="text-3xl font-bold text-center mb-6">{move_tr!("terms-title")}</h1>

                <p class="text-lg leading-relaxed mb-6">
                    {move_tr!("terms-intro")}
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section1-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section1-content")}</p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section2-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section2-intro")}</p>
                <ul class="list-disc list-inside space-y-2">
                    <li class="text-red-500">{move_tr!("terms-section2-item1")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item3")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item4")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item5")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item6")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item7")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item8")}</li>
                </ul>
                <p class="italic mt-2"
                    class=("text-gray-600", move || !is_dark.get())
                    class=("text-gray-400", move || is_dark.get())
                >
                    {move_tr!("terms-section2-note")}
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section3-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section3-content")}</p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section4-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section4-content")}</p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section5-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section5-content")}</p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section6-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>{move_tr!("terms-section6-item1")}</li>
                    <li>{move_tr!("terms-section6-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section6-item3")}</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section7-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>{move_tr!("terms-section7-item1")}</li>
                    <li>{move_tr!("terms-section7-item2")}</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section8-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>{move_tr!("terms-section8-item1")}</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section9-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>{move_tr!("terms-section9-item1")}</li>
                    <li>{move_tr!("terms-section9-item2")}</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section10-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>{move_tr!("terms-section10-item1")}</li>
                    <li>{move_tr!("terms-section10-item2")}</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">{move_tr!("terms-section11-title")}</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li class="text-red-500">{move_tr!("terms-section11-item1")}</li>
                    <li class="text-red-500">{move_tr!("terms-section11-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section11-item3")}</li>
                </ul>

            </main>
            <Footer />
        </div>
    }
}