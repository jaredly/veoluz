var waiting = null
onmessage = (evt) => {
  console.log('not ready yet')
  waiting = evt.data
}


import("../crate/pkg").then(module => {
  let handle = data => {
    let res = module.process(data)
    // console.log('result', res)
    postMessage(res, [res.buffer])
  }
  if (waiting) {
    handle(waiting)
  }
  onmessage = evt => handle(evt.data)
});
