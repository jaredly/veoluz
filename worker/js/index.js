var waiting = null
onmessage = (evt) => {
  console.log('not ready yet')
  waiting = evt.data
}

let timeout = null
let requestId = 0;

let loop = (times, fn) => {
  if (times === 0) return
  timeout = setTimeout(() => {
    fn();
    loop(times - 1, fn)
  }, 1)
}

import("../crate/pkg").then(module => {
  let handle = data => {
    clearTimeout(timeout);
    // console.log('message', data)
    let res = module.process(data)
    postMessage({id: data.id, buffer: res.buffer}, [res.buffer])
    // loop(20, () => {
    //   let res = module.process(data)
    //   postMessage(res.buffer, [res.buffer])
    // })
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
