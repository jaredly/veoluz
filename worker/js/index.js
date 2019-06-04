import("../crate/pkg").then(module => {
  global.hang_onto_this = module.run();
});
