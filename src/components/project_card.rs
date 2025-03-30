use leptos::prelude::*;

#[component]
pub fn ProjectCard(
    title: String,
    description: String,
    image: String,
) -> impl IntoView {
    view! {
        <div class="overflow-hidden rounded-lg border bg-card text-card-foreground shadow">
            <div class="relative h-48">
                <img 
                    src=image
                    alt=title.clone()
                    class="object-cover w-full h-full"
                />
            </div>
            <div class="p-6">
                <h3 class="text-lg font-semibold">{title}</h3>
                <p class="text-sm text-muted-foreground mt-2 mb-4">{description}</p>
                <p class="mb-4">
                    "This project demonstrates the power of Rust on the web, using Leptos for reactivity
                    and TailwindCSS for styling."
                </p>
                <div class="flex gap-2">
                    <a 
                        href="#" 
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-3 border border-input bg-background"
                    >
                        "Demo"
                    </a>
                    <a 
                        href="#" 
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-3 bg-primary text-primary-foreground"
                    >
                        "Source Code"
                    </a>
                </div>
            </div>
        </div>
    }
}

