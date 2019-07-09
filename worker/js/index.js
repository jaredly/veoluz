var waiting = null;
onmessage = evt => {
  console.log("not ready yet");
  waiting = evt.data;
};

let timeout = null;
let requestId = 0;

let loopUntil = fn => {
  timeout = setTimeout(() => {
    if (fn()) {
      // console.log('finished')
      return;
    }
    loopUntil(fn);
  }, 1);
};

import("../crate/pkg").then(module => {
  let handle = data => {
    clearTimeout(timeout);
    const start = performance.now();
    const res = module.process(data);
    const end = performance.now();
    // console.log('res', res)
    let rays = res.rays();
    let buffer = res.data().buffer;
    let total_rays = rays;
    let total_seconds = (end - start) / 1000.0;
    postMessage({ id: data.id, buffer: buffer, total_rays, total_seconds }, [
      buffer
    ]);

    loopUntil(() => {
      const start = performance.now();
      const res = module.process(data);
      const end = performance.now();

      const rays = res.rays();
      total_rays += rays;
      total_seconds += (end - start) / 1000.0;
      let buffer = res.data().buffer;
      postMessage({ id: data.id, buffer: buffer, total_rays, total_seconds }, [
        buffer
      ]);
      // console.log('responded++')
      return total_rays >= data.count;
    });
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
    // res = module.process(data)
    // postMessage(res.buffer, [res.buffer])
  };
  if (waiting) {
    handle(waiting);
  }
  onmessage = evt => handle(evt.data);
});
