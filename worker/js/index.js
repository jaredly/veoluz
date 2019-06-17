var waiting = null
onmessage = (evt) => {
  console.log('not ready yet')
  waiting = evt.data
}

let timeout = null
let requestId = 0;

let loopUntil = (fn) => {
  timeout = setTimeout(() => {
    if (fn()) {
      // console.log('finished')
      return
    };
    loopUntil(fn)
  }, 1)
}

import("../crate/pkg").then(module => {
  let handle = data => {
    clearTimeout(timeout);
    // console.log('message', data)
    let res = module.process(data)
    // console.log('res', res)
    let rays = res.rays();
    let buffer = res.data().buffer;
    let total_rays = rays;
    postMessage({id: data.id, buffer: buffer}, [buffer])

    loopUntil(() => {
      let res = module.process(data)
      let rays = res.rays();
      total_rays += rays;
      let buffer = res.data().buffer;
      postMessage({id: data.id, buffer: buffer}, [buffer])
      return total_rays >= data.count
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
