var waiting = null
onmessage = (evt) => {
  console.log('not ready yet')
  waiting = evt.data
}

let loop = (times, fn) => {
  if (times === 0) return
  setTimeout(() => {
    fn();
    loop(times - 1, fn)
  }, 1)
}

import("../crate/pkg").then(module => {
  let handle = data => {
    let res = module.process(data)
    postMessage(res.buffer, [res.buffer])
    loop(20, () => {
      let res = module.process(data)
      postMessage(res.buffer, [res.buffer])
    })
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
  }
  if (waiting) {
    handle(waiting)
  }
  onmessage = evt => handle(evt.data)
});
