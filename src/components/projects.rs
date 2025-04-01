use leptos::prelude::*;
use crate::components::ProjectCard;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="py-12 md:py-16 scroll-mt-20">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Mis Proyectos"</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
                <ProjectCard
                    title="Meica Mod".to_string()
                    description="Mod de Minecraft inspirado en la Vtuber Meica05".to_string()
                    image="/public/meica_mod.png".to_string()
                    demo="https://www.curseforge.com/minecraft/mc-mods/meica-mod".to_string()
                    source_code="https://github.com/CrawKatt/meica_mod".to_string()
                />
                <ProjectCard
                    title="Leafy".to_string()
                    description="Bot de Discord hecho en Rust".to_string()
                    image="/public/leafy.png".to_string()
                    demo="#".to_string()
                    source_code="https://github.com/CrawKatt/leafy".to_string()
                />
                <ProjectCard
                    title="Leafy Dashboard".to_string()
                    description="Una Dashboard Web hecha en Leptos, Actix y SurrealDB para configurar a Leafy"
                        .to_string()
                    image="/public/leafy.png".to_string()
                    demo="#".to_string()
                    source_code="https://github.com/CrawKatt/leafy_dashboard".to_string()
                />
            </div>
        </section>
    }
}