import('./pkg/zenphoton')
    .then(wasm => wasm.run())
    .catch(console.error);
