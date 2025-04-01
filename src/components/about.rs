use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-12 md:py-16 scroll-mt-20">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Sobre Mí"</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 items-center">
                <div class="order-2 md:order-1">
                    <p class="text-base md:text-lg mb-4">
                        "¡Hola! Soy un desarrollador apasionado con experiencia en la creación de mods para Minecraft y programación en Rust."
                    </p>
                    <p class="text-base md:text-lg mb-4">
                        "Desde que era niño siempre me entusiasmé por la computación y la tecnología, crecí estudiando por mi cuenta siguiendo
                        guías, foros y tutoriales en YouTube. Cursé múltiples cursos de Udemy sobre programación y Modding para Minecraft."
                    </p>
                    <p class="text-base md:text-lg">
                        "Me dedico principalmente al Modding para Minecraft, BackEnd y Administración de Sistemas.
                        Soy un apasionado usuario de Arch Linux y me encanta aprender y desarrollar proyectos únicos para los usuarios."
                    </p>
                </div>
                <div class="relative h-60 md:h-80 rounded-lg overflow-hidden order-1 md:order-2">
                    <img
                        src="/public/about.png"
                        alt="About me"
                        class="object-cover w-full h-full"
                    />
                </div>
            </div>
        </section>
    }
}