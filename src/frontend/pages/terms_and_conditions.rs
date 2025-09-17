use crate::frontend::components::{Footer, Header};
use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn TermsAndConditions() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <Header />
            <main class="container py-12 px-6 mx-auto max-w-3xl rounded-lg shadow-lg sm:px-8">
                <h1 class="mb-6 text-3xl font-bold text-center">{move_tr!("terms-title")}</h1>

                <p class="mb-6 text-lg leading-relaxed">{move_tr!("terms-intro")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section1-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section1-content")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section2-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section2-intro")}</p>
                <ul class="space-y-2 list-disc list-inside">
                    <li class="text-red-500">{move_tr!("terms-section2-item1")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item3")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item4")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item5")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-item6")}</li>
                </ul>
                <p class="mt-2 italic">{move_tr!("terms-section2-note")}</p>
                <h3 class="mt-8 mb-4 text-2xl font-semibold">
                    {move_tr!("terms-section2-1-title")}
                </h3>
                <p class="mb-4">{move_tr!("terms-section2-1-intro")}</p>
                <ul class="space-y-2 list-disc list-inside">
                    <li class="text-red-500">{move_tr!("terms-section2-1-item1")}</li>
                    <li class="text-red-500">{move_tr!("terms-section2-1-item2")}</li>
                </ul>
                <p class="mt-2 italic">{move_tr!("terms-section2-note")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section3-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section3-content")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section4-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section4-content")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section5-title")}</h3>
                <p class="mb-4">{move_tr!("terms-section5-content")}</p>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section6-title")}</h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li>{move_tr!("terms-section6-item1")}</li>
                    <li>{move_tr!("terms-section6-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section6-item3")}</li>
                </ul>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section7-title")}</h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li>{move_tr!("terms-section7-item1")}</li>
                    <li>{move_tr!("terms-section7-item2")}</li>
                </ul>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section8-title")}</h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li>{move_tr!("terms-section8-item1")}</li>
                </ul>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">{move_tr!("terms-section9-title")}</h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li>{move_tr!("terms-section9-item1")}</li>
                    <li>{move_tr!("terms-section9-item2")}</li>
                </ul>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">
                    {move_tr!("terms-section10-title")}
                </h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li>{move_tr!("terms-section10-item1")}</li>
                    <li>{move_tr!("terms-section10-item2")}</li>
                </ul>

                <h3 class="mt-8 mb-4 text-2xl font-semibold">
                    {move_tr!("terms-section11-title")}
                </h3>
                <ul class="space-y-2 list-disc list-inside">
                    <li class="text-red-500">{move_tr!("terms-section11-item1")}</li>
                    <li class="text-red-500">{move_tr!("terms-section11-item2")}</li>
                    <li class="text-red-500">{move_tr!("terms-section11-item3")}</li>
                </ul>

            </main>
            <Footer />
        </div>
    }
}