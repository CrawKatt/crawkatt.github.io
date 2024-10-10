document.addEventListener("DOMContentLoaded", function () {
  const sections = document.querySelectorAll(".section");
  const navLinks = document.querySelectorAll("nav button");

  function showSection(sectionId) {
    sections.forEach((section) => {
      sectionId === "all" || section.id === sectionId
        ? section.classList.remove("hidden", "hide")
        : (() => {
            section.classList.add("hidden");
            setTimeout(() => {
              section.classList.add("hide");
            }, 550);
          })();
    });
  }

  // Click Handler del menú
  navLinks.forEach((link) => {
    link.addEventListener("click", function (event) {
      event.preventDefault();
      const sectionId = event.target.getAttribute("data-section");
      showSection(sectionId);

      navLinks.forEach((link) => link.classList.remove("active"));
      event.target.classList.add("active");
    });
  });

  showSection("all");
});

/*
  // TypeWriter

  // Animación del título
  const h1Text =
    "Modder de Minecraft, Programador en Rust y Administrador de Sistemas";

  const h1Element = document.getElementById("typing-h1");

  const typingSpeed = 100;
  const erasingSpeed = 50;
  const delayBetweenTexts = 2000;
  const delayBetweenLoops = 1000;

  function typeText(element, text, index, callback) {
    if (index < text.length) {
      element.textContent += text.charAt(index);
      setTimeout(() => {
        typeText(element, text, index + 1, callback);
      }, typingSpeed);
    } else {
      setTimeout(callback, delayBetweenTexts);
    }
  }

  function eraseText(element, callback) {
    const text = element.textContent;
    if (text.length > 0) {
      element.textContent = text.substring(0, text.length - 1);
      setTimeout(() => {
        eraseText(element, callback);
      }, erasingSpeed);
    } else {
      setTimeout(callback, delayBetweenLoops);
    }
  }

  function loopTyping() {
    typeText(h1Element, h1Text, 0, () => {
      eraseText(h1Element, () => {
        loopTyping();
      });
    });
  }

  // Iniciar la animación
  loopTyping();
});
*/
