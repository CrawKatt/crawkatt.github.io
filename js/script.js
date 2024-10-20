document.addEventListener("DOMContentLoaded", function () {
    const sections = document.querySelectorAll(".section");
    const navButtons = document.querySelectorAll(".sidebar button, .navbar button");
    const typewriterContainer = document.querySelector(".typewriter-container");
    const titleImageContainer = document.querySelector("#title-image-container");
    const menuToggle = document.querySelector(".menu-toggle");
    const sidebar = document.querySelector("nav.sidebar");
    const closeBtn = document.querySelector(".close-btn");
    const body = document.body;

    // Función para detectar si es móvil
    function isMobile() {
        return window.innerWidth <= 768;
    }

    // Función para mostrar u ocultar la sidebar
    function toggleSidebar() {
        sidebar.classList.toggle("open");
        body.classList.toggle("sidebar-open");
        menuToggle.classList.toggle("hidden");
    }

    // Evento para abrir/cerrar la sidebar al hacer clic en el botón de menú
    menuToggle.addEventListener("click", function () {
        toggleSidebar();
    });

    // Evento para cerrar la sidebar al hacer clic en el botón de cierre
    closeBtn.addEventListener("click", function () {
        toggleSidebar();
    });

    // Evento para los botones del menú
    navButtons.forEach((button) => {
        button.addEventListener("click", function () {
            const sectionId = this.getAttribute("data-section");
            if (isMobile()) {
                // En móviles, cerrar la sidebar y hacer scroll suave
                toggleSidebar();
                const targetSection = document.getElementById(sectionId);
                if (targetSection) {
                    targetSection.scrollIntoView({ behavior: "smooth" });
                }
            } else {
                // En escritorio, mostrar la sección correspondiente
                showSection(sectionId);
                navButtons.forEach((btn) => btn.classList.remove("active"));
                this.classList.add("active");
                // Desplazar suavemente a la sección seleccionada
                const targetSection = document.getElementById(sectionId);
                if (targetSection) {
                    window.scrollTo({
                        top: targetSection.offsetTop - 60, // Ajustar por la altura del navbar
                        behavior: "smooth",
                    });
                }
            }
        });
    });

    function showSection(sectionId) {
        sections.forEach((section) => {
            if (sectionId === "all" || section.id === sectionId) {
                section.classList.remove("hidden");
            } else {
                section.classList.add("hidden");
            }
        });

        // Ocultar o mostrar la typewriter-container y la imagen del título
        if (sectionId === "all") {
            typewriterContainer.classList.remove("hidden");
            titleImageContainer.classList.remove("hidden");
        } else {
            typewriterContainer.classList.add("hidden");
            titleImageContainer.classList.add("hidden");
        }
    }

    // Mostrar todas las secciones al cargar la página en escritorio
    if (!isMobile()) {
        showSection("all");
    }

    // Escuchar cambios de tamaño de la ventana para actualizar el comportamiento
    window.addEventListener("resize", () => {
        if (!isMobile()) {
            // En escritorio, aseguramos que la sidebar esté cerrada
            sidebar.classList.remove("open");
            body.classList.remove("sidebar-open");
            // Mostrar todas las secciones o filtrar según el botón activo
            const activeButton = document.querySelector(".navbar button.active");
            const sectionId = activeButton
                ? activeButton.getAttribute("data-section")
                : "all";
            showSection(sectionId);
        } else {
            // En móviles, cerramos la sidebar si está abierta
            sidebar.classList.remove("open");
            body.classList.remove("sidebar-open");
        }
    });
});
