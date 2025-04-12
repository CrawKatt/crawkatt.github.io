use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-12 md:py-16 scroll-mt-20 h-auto">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">
                {move_tr!("about-title")}
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12">
                <div class="order-2 md:order-1">
                    <div class="bg-black text-green-500 p-6 rounded-lg border-2 border-green-500 font-mono">
                        <div class="flex gap-2 mb-4">
                            <div class="w-3 h-3 rounded-full bg-red-500"></div>
                            <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                            <div class="w-3 h-3 rounded-full bg-green-500"></div>
                        </div>
                        <div class="flex flex-col gap-4 text-pretty justify-evenly text-xl">
                            <p>{move_tr!("about-intro")}</p>
                            <p>{move_tr!("about-childhood")}</p>
                            <p>{move_tr!("about-education")}</p>
                            <p>{move_tr!("about-focus")}</p>
                        </div>
                    </div>
                </div>
                <div class="relative rounded-lg overflow-visible order-1 md:order-2">
                    <img
                        src="/public/about.png"
                        alt=move_tr!("about-image-alt")
                        class="object-contain w-full p-4 overflow-visible"
                    />
                </div>
            </div>
        </section>
    }
}