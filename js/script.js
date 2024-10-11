document.addEventListener("DOMContentLoaded", function () {
  const sections = document.querySelectorAll(".section");
  const navLinks = document.querySelectorAll("nav button");

  function showSection(sectionId) {
    sections.forEach((section) => {
      if (sectionId === "all" || section.id === sectionId) {
        section.classList.remove("hidden", "hide");
      } else {
        section.classList.add("hidden");
        setTimeout(() => {
          section.classList.add("hide");
        }, 550);
      }
    });
  }

  // Click Handler del menú
  navLinks.forEach((link) => {
    link.addEventListener("click", function (event) {
      event.preventDefault();
      const sectionId = this.getAttribute("data-section");
      showSection(sectionId);

      navLinks.forEach((btn) => btn.classList.remove("active"));
      this.classList.add("active");
    });
  });

  showSection("all");
});
