use leptos::prelude::ServerFnError;
use leptos::server;
#[cfg(feature = "ssr")]
use lettre::{Message, SmtpTransport, Transport};
#[cfg(feature = "ssr")]
use lettre::message::SinglePart;
#[cfg(feature = "ssr")]
use lettre::transport::smtp::authentication::Credentials;

#[server(SendContact, "/api")]
pub async fn send_contact(name: String, email: String, message: String) -> Result<(), ServerFnError<String>> {
    let mailer_email = dotenvy::var("EMAIL").map_err(|_| ServerFnError::ServerError("EMAIL not set".to_string()))?;
    let password = dotenvy::var("APP_PASSWORD").map_err(|_| ServerFnError::ServerError("APP_PASSWORD not set".to_string()))?;
    let to_email = dotenvy::var("TO_EMAIL").map_err(|_| ServerFnError::ServerError("TO_EMAIL not set".to_string()))?;

    let email_msg = Message::builder()
        .from(format!("Portfolio <{mailer_email}>").parse().unwrap())
        .to(format!("Admin <{to_email}>").parse().unwrap())
        .subject(format!("Contact from: {name}"))
        .singlepart(SinglePart::plain(format!("De: {name}\nEmail: {email}\n\nMensaje:\n{message}"))).unwrap();

    let creds = Credentials::new(mailer_email, password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email_msg).map_err(|why| ServerFnError::ServerError(format!("Error enviando correo: {why}")))?;

    Ok(())
}