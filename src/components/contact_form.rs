use leptos::{ev, logging};
use leptos::prelude::*;

#[component]
pub fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (accepted_terms, set_accepted_terms) = signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if accepted_terms.get() {
            logging::log!("Form submitted: {}, {}, {}", name.get(), email.get(), message.get());
        } else {
            logging::log!("Debe aceptar los términos y condiciones.");
        }
    };

    view! {
        <div class="rounded-lg border bg-card text-card-foreground shadow">
            <div class="p-6">
                <h3 class="text-lg font-semibold mb-1">"Envíe un Mensaje"</h3>
                <p class="text-sm text-muted-foreground mb-4">
                    "Llene el formulario a continuación y me pondré en contacto con usted lo antes posible."
                </p>
                <form on:submit=handle_submit class="grid gap-4">
                    <div class="grid gap-2">
                        <label for="name" class="text-sm font-medium">
                            "Nombre"
                        </label>
                        <input
                            id="name"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder="Su nombre"
                            on:input=move |ev| {
                                set_name.set(event_target_value(&ev));
                            }
                            prop:value=name
                        />
                    </div>
                    <div class="grid gap-2">
                        <label for="email" class="text-sm font-medium">
                            "Email"
                        </label>
                        <input
                            id="email"
                            type="email"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder="Su email"
                            on:input=move |ev| {
                                set_email.set(event_target_value(&ev));
                            }
                            prop:value=email
                        />
                    </div>
                    <div class="grid gap-2">
                        <label for="message" class="text-sm font-medium">
                            "Mensaje"
                        </label>
                        <textarea
                            id="message"
                            class="flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder="Su mensaje"
                            on:input=move |ev| {
                                set_message.set(event_target_value(&ev));
                            }
                            prop:value=message
                        />
                    </div>
                    <div class="flex items-start gap-2">
                        <input
                            id="terms"
                            type="checkbox"
                            class="w-5 h-5 rounded border border-gray-300 checked:bg-primary checked:border-transparent focus:ring-2 focus:ring-primary"
                            on:change=move |ev| {
                                set_accepted_terms.set(event_target_checked(&ev));
                            }
                            prop:checked=accepted_terms
                        />
                        <label for="terms" class="text-sm text-gray-600">
                            "Acepto los "
                            <a href="/terms-and-conditions" class="text-primary hover:underline">
                                "Términos y Condiciones"
                            </a>
                        </label>
                    </div>
                    <button
                        type="submit"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 w-full bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || !accepted_terms.get()
                    >
                        "Enviar Mensaje"
                    </button>
                </form>
            </div>
        </div>
    }
}

