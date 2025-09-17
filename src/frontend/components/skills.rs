use leptos::prelude::*;
use crate::frontend::components::SkillCard;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills" class="py-12 md:py-16 scroll-mt-20">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Skills"</h2>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 md:gap-4">
                <SkillCard name="Rust".to_string() />
                <SkillCard name="Leptos".to_string() />
                <SkillCard name="Forge".to_string() />
                <SkillCard name="Fabric".to_string() />
                <SkillCard name="SurrealDB".to_string() />
                <SkillCard name="TailwindCSS".to_string() />
                <SkillCard name="Java".to_string() />
                <SkillCard name="C#".to_string() />
            </div>
        </section>
    }
}