use leptos::prelude::*;

#[component]
pub fn ProjectCard(
    title: String,
    description: String,
    image: String,
    demo: String,
    source_code: String
) -> impl IntoView {
    view! {
        <div class="overflow-hidden rounded-lg border bg-card text-card-foreground shadow">
            <div class="relative aspect-square">
                <img
                    src=image
                    alt=title.clone()
                    class="object-contain w-full h-full p-4"
                    loading="lazy"
                />
            </div>
            <div class="p-6">
                <h3 class="text-lg font-semibold">{title}</h3>
                <p class="mb-4">{description}</p>
                <div class="flex gap-2">
                    <a
                        href=demo
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-3 border border-input bg-background"
                    >
                        "Demo"
                    </a>
                    <a
                        href=source_code
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-3 bg-primary text-primary-foreground"
                    >
                        "Source Code"
                    </a>
                </div>
            </div>
        </div>
    }
}

