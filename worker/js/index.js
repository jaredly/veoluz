var waiting = null
onmessage = (evt) => {
  console.log('not ready yet')
  waiting = evt

}
import("../crate/pkg").then(module => {
  global.hang_onto_this = module.run();
  onmessage(waiting)
});
