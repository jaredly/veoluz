let options = {
  root: null,
  rootMargin: "0px"
  // threshold: 1.0
};

let observer = new IntersectionObserver(evt => {
  evt.forEach(evt => {
    if (evt.isIntersecting) {
      const div = evt.target;
      if (!div.classList.contains("loading")) {
        return;
      }
      div.classList.remove("loading");
      const img = document.createElement("img");
      img.src = div.getAttribute("data-src");
      div.innerHTML = "";
      div.appendChild(img);
      observer.unobserve(div);
    }
  });
}, options);

[].forEach.call(document.querySelectorAll(".image.loading"), function(div) {
  // intrinsic 1024 x 576
  // img.src = "nope";

  observer.observe(div);
});
