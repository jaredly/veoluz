import('./pkg/zenphoton')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');
        canvas.width = 1024
        canvas.height = 576

        const realInput = document.getElementById('real');
        const imaginaryInput = document.getElementById('imaginary');
        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            const real = parseFloat(realInput.value) || 0;
            const imaginary = parseFloat(imaginaryInput.value) || 0;
            wasm.draw(ctx, canvas.width, canvas.height, real, imaginary);
        });

        wasm.draw(ctx, canvas.width, canvas.height, -0.15, 0.65);
    })
    .catch(console.error);
