// dummy service worker to undo the old one
self.addEventListener("activate", function(e) {
  self.registration.unregister();
});
