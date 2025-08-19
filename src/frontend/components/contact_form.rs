use crate::server::send_contact;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::ev;
use leptos_fluent::move_tr;

#[component]
pub fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (accepted_terms, set_accepted_terms) = signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if !accepted_terms.get() {
            web_sys::window()
                .unwrap()
                .alert_with_message("Debe aceptar los términos y condiciones ⚠️")
                .unwrap();
            return;
        }

        let name_val = name.get();
        let email_val = email.get();
        let message_val = message.get();

        if name_val.is_empty() || email_val.is_empty() || message_val.is_empty() {
            web_sys::window()
                .unwrap()
                .alert_with_message("Todos los campos son obligatorios ⚠️")
                .unwrap();
            return;
        }

        spawn_local(async move {
            match send_contact(name_val, email_val, message_val).await {
                Ok(_) => web_sys::window()
                    .unwrap()
                    .alert_with_message("Formulario enviado con éxito. ¡Gracias por contactarme!")
                    .unwrap(),
                Err(why) => web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!("Error enviando el formulario: {why}"))
                    .unwrap(),
            }
        });
    };

    view! {
        <div class="rounded-lg border bg-card text-card-foreground shadow">
            <div class="p-6">
                <h3 class="text-lg font-semibold mb-1">{move_tr!("contact-form-title")}</h3>
                <p class="text-sm text-muted-foreground mb-4">
                    {move_tr!("contact-form-description")}
                </p>
                <form on:submit=handle_submit class="grid gap-4">
                    <div class="grid gap-2">
                        <label for="name" class="text-sm font-medium">
                            {move_tr!("contact-form-name-label")}
                        </label>
                        <input
                            id="name"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder=move_tr!("contact-form-name-placeholder")
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            prop:value=name
                        />
                    </div>
                    <div class="grid gap-2">
                        <label for="email" class="text-sm font-medium">
                            {move_tr!("contact-form-email-label")}
                        </label>
                        <input
                            id="email"
                            type="email"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder=move_tr!("contact-form-email-placeholder")
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            prop:value=email
                        />
                    </div>
                    <div class="grid gap-2">
                        <label for="message" class="text-sm font-medium">
                            {move_tr!("contact-form-message-label")}
                        </label>
                        <textarea
                            id="message"
                            class="flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            placeholder=move_tr!("contact-form-message-placeholder")
                            on:input=move |ev| set_message.set(event_target_value(&ev))
                            prop:value=message
                        />
                    </div>
                    <div class="flex items-start gap-2">
                        <input
                            id="terms"
                            type="checkbox"
                            class="w-5 h-5 rounded border border-gray-300 checked:bg-primary checked:border-transparent focus:ring-2 focus:ring-primary"
                            on:change=move |ev| set_accepted_terms.set(event_target_checked(&ev))
                            prop:checked=accepted_terms
                        />
                        <label for="terms" class="text-sm text-gray-600">
                            {move_tr!("contact-form-terms-agree")}
                            " "
                            <a href="/terms-and-conditions" class="text-primary hover:underline">
                                {move_tr!("contact-form-terms-link")}
                            </a>
                        </label>
                    </div>
                    <button
                        type="submit"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 w-full bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || !accepted_terms.get()
                    >
                        {move_tr!("contact-form-submit")}
                    </button>
                </form>
            </div>
        </div>
    }
}