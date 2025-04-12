use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative py-12 md:py-20 min-h-[500px] flex items-center justify-center overflow-hidden">
            <video
                autoplay
                muted
                loop
                class="absolute inset-0 w-full h-full object-cover blur-lg z-0"
            >
                <source src="/public/background-video.webm" type="video/webm" />
                "Your browser does not support HTML5 video."
            </video>
            <div class="absolute inset-0 bg-black/50 z-10"></div>
            <div class="relative z-20 flex flex-col items-center text-center">
                <div class="relative w-24 h-24 md:w-32 md:h-32 mb-6 md:mb-8 rounded-full overflow-hidden border-4 border-primary">
                    <img
                        src="/public/profile.png"
                        alt=move_tr!("hero-profile-alt")
                        class="object-cover w-full h-full"
                    />
                </div>
                <h1 class="text-3xl md:text-4xl lg:text-6xl font-bold tracking-tight mb-4 text-white">
                    "CrawKatt"
                </h1>
                <p class="text-lg md:text-xl lg:text-2xl mb-6 max-w-2xl text-white/80">
                    {move_tr!("hero-subtitle")}
                </p>
                <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
                    <a
                        href="#contact"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-white hover:bg-primary/90 transition-colors"
                    >
                        {move_tr!("hero-contact")}
                    </a>
                    <a
                        href="#projects"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 bg-white/20 text-white hover:bg-white/30 backdrop-blur-sm border border-white/30 transition-all"
                    >
                        {move_tr!("hero-projects")}
                    </a>
                </div>
            </div>
        </section>
    }
}