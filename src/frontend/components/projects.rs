use leptos::prelude::*;
use leptos_fluent::move_tr;
use crate::frontend::components::ProjectCard;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="py-12 md:py-16 scroll-mt-20">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">
                {move_tr!("projects-title")}
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
                <ProjectCard
                    title=move_tr!("projects-meica-title")
                    description=move_tr!("projects-meica-description")
                    image="/public/meica_mod.png".to_string()
                    demo="https://www.curseforge.com/minecraft/mc-mods/meica-mod".to_string()
                    source_code="https://github.com/CrawKatt/meica_mod".to_string()
                />
                <ProjectCard
                    title=move_tr!("projects-leafy-title")
                    description=move_tr!("projects-leafy-description")
                    image="/public/leafy.png".to_string()
                    demo="#".to_string()
                    source_code="https://github.com/CrawKatt/leafy".to_string()
                />
                <ProjectCard
                    title=move_tr!("projects-leafy-dashboard-title")
                    description=move_tr!("projects-leafy-dashboard-description")
                    image="/public/leafy.png".to_string()
                    demo="#".to_string()
                    source_code="https://github.com/CrawKatt/leafy_dashboard".to_string()
                />
            </div>
        </section>
    }
}