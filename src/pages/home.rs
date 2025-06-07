use crate::components::{Header, Hero, About, Projects, Skills, Contact, Footer, Separator};
use crate::context::provide_theme_context;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let _ = provide_theme_context();

    view! {
        <Header />
        <div class="relative">
            <Hero />
            <div class="h-86 z-20 relative overflow-hidden">
                <Separator />
            </div>
            <main class="relative z-30 min-h-screen w-full bg-background">
                <div class="container mx-auto py-8 px-4 sm:px-6">
                    <About />
                    <Projects />
                    <Skills />
                    <Contact />
                </div>
            </main>
        </div>
        <Footer />
    }
}