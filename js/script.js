document.addEventListener("DOMContentLoaded", function () {
  const sections = document.querySelectorAll(".section");
  const navLinks = document.querySelectorAll("nav a");

  function showSection(sectionId) {
    sections.forEach((section) => {
      if (sectionId === "all") {
        // Mostrar todas las secciones
        section.classList.remove("hidden");
      } else if (section.id === sectionId) {
        section.classList.remove("hidden");
      } else {
        section.classList.add("hidden");
      }
    });
  }

  // Click Handler del menú
  navLinks.forEach((link) => {
    link.addEventListener("click", function (event) {
      event.preventDefault();
      const sectionId = this.getAttribute("data-section");
      showSection(sectionId);

      navLinks.forEach((link) => link.classList.remove("active"));
      this.classList.add("active");
    });
  });

  // Mostrar todas las secciones al cargar la página
  showSection("all");
});
