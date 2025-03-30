use crate::components_old::{footer::Footer, header::Header};
use leptos::prelude::*;
use leptos_meta::*;
use crate::components_old::about::AboutSection;
use crate::components_old::contact::ContactSection;
use crate::components_old::hero::HeroSection;
use crate::components_old::projects::ProjectsSection;
use crate::components_old::skills::SkillsSection;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="CrawKatt | Minecraft Modder"/>
        <Header/>
        <main class="min-h-screen bg-background">
            <HeroSection/>
            <AboutSection/>
            <ProjectsSection/>
            <SkillsSection/>
            <ContactSection/>
        </main>
        <Footer/>
    }
}