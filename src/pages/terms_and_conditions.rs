use leptos::prelude::*;
use crate::components::{Footer, Header};

#[component]
pub fn TermsAndConditions() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background text-gray-900 dark:text-gray-200">
            <Header />
            <main class="container mx-auto max-w-3xl py-12 px-6 sm:px-8 bg-white dark:bg-gray-900 shadow-lg rounded-lg">
                <h1 class="text-3xl font-bold text-center mb-6">"Términos y condiciones"</h1>

                <p class="text-lg leading-relaxed mb-6">
                    "Bienvenido/a a mi portafolio web. Al interactuar con este sitio y/o considerar mis servicios como desarrollador de Rust y Minecraft Modder,
                    aceptas los siguientes términos y condiciones:"
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"1. Alcance de los Servicios"</h3>
                <p class="mb-4">
                    "Ofrezco servicios de desarrollo en Rust y creación de mods para Minecraft. Estos pueden incluir,
                    entre otros, diseño y programación de mods, optimización de código y consultoría técnica."
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"2. Restricciones sobre Proyectos Aceptados"</h3>
                <p class="mb-4">
                    "No acepto comisiones, solicitudes ni colaboraciones relacionadas con Blockchain, incluyendo, pero no limitado a:"
                </p>
                <ul class="list-disc list-inside space-y-2">
                    <li class="text-red-500">"Aplicaciones Web3"</li>
                    <li class="text-red-500">"Smart Contracts"</li>
                    <li class="text-red-500">"Tokens"</li>
                    <li class="text-red-500">"Criptomonedas (Crypto)"</li>
                    <li class="text-red-500">"NFTs"</li>
                    <li class="text-red-500">"Juegos Play-to-Earn (P2E)"</li>
                </ul>
                <p class="italic text-gray-600 mt-2">
                    "Cualquier solicitud que involucre estas tecnologías será rechazada de inmediato."
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"3. Derechos de Autor y Propiedad Intelectual"</h3>
                <p class="mb-4">
                    "A menos que se acuerde lo contrario, conservo los derechos de autor de los proyectos desarrollados.
                    El cliente recibirá los derechos de uso según lo estipulado en el acuerdo de trabajo."
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"4. Responsabilidad"</h3>
                <p class="mb-4">
                    "No me hago responsable por el mal uso del código o software entregado. Una vez finalizado el proyecto y entregado al cliente,
                    el uso y las consecuencias derivadas del mismo son responsabilidad exclusiva del receptor."
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"5. Modificaciones y Cancelaciones"</h3>
                <p class="mb-4">
                    "Me reservo el derecho de modificar estos términos y condiciones en cualquier momento.
                    Asimismo, me reservo el derecho de rechazar o cancelar proyectos en cualquier
                    fase si se violan estas condiciones o si surge alguna circunstancia que impida
                    la finalización del trabajo."
                </p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"6. Formas de Pago y Reembolsos"</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Acepto pagos a través de los métodos previamente acordados con el cliente."</li>
                    <li>"Se podría requerir un anticipo antes de iniciar el trabajo."</li>
                    <li class="text-red-500">"No se realizarán reembolsos una vez que el trabajo haya comenzado, salvo acuerdo previo."</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"7. Plazos y Entregas"</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Los tiempos de entrega serán acordados con el cliente y podrán ajustarse según la complejidad del proyecto."</li>
                    <li>"No me hago responsable de retrasos ocasionados por falta de información, respuestas tardías o cambios solicitados por el cliente."</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"8. Uso del Trabajo en el Portafolio"</h3>
                <p>"Me reservo el derecho de mostrar los proyectos desarrollados en mi portafolio y redes sociales, salvo que se acuerde lo contrario."</p>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"9. Soporte y Mantenimiento"</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Ofrezco soporte por un tiempo determinado tras la entrega del proyecto, el cual se definirá en el acuerdo con el cliente."</li>
                    <li>"Cualquier mantenimiento o actualización posterior podría implicar costos adicionales."</li>
                </ul>

                <h3 class="text-2xl font-semibold mt-8 mb-4">"10. Confidencialidad"</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Me comprometo a tratar con confidencialidad cualquier información proporcionada por el cliente."</li>
                    <li>"Si es necesario, podré firmar acuerdos de confidencialidad (NDA) bajo petición del cliente."</li>
                </ul>

            </main>
            <Footer />
        </div>
    }
}
