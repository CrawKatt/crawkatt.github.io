document.addEventListener("DOMContentLoaded", function () {
  const sections = document.querySelectorAll(".section");
  const navButtons = document.querySelectorAll("nav button");
  const typewriterContainer = document.querySelector(".typewriter-container");
  const titleImageContainer = document.querySelector("#title-image-container");

  function showSection(sectionId) {
    sections.forEach((section) => {
      if (sectionId === "all" || section.id === sectionId) {
        section.classList.remove("hidden");
      } else {
        section.classList.add("hidden");
      }
    });

    // Ocultar o mostrar la typewriter-container con animación
    if (sectionId === "all") {
      typewriterContainer.classList.remove("hidden");
      titleImageContainer.classList.remove("hidden");
    } else {
      typewriterContainer.classList.add("hidden");
      titleImageContainer.classList.add("hidden");
    }
  }

  navButtons.forEach((button) => {
    button.addEventListener("click", function () {
      const sectionId = this.getAttribute("data-section");
      showSection(sectionId);

      navButtons.forEach((btn) => btn.classList.remove("active"));
      this.classList.add("active");
    });
  });

  showSection("all");
});
