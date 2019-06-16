import localForage from 'localforage'

const rid = () => Math.random().toString(36).slice(2)
const uuid = () => rid() + rid()

const log = fn => (...args) => fn(...args).catch(err => console.error(err))

let hideTimeout = null

const makeSceneNode = (wasm, id, blob) => {
    const scenes_node = document.getElementById('saves')
    const time = new Date(parseInt(id.split(':')[0]))
    const div = document.createElement('div')
    scenes_node.appendChild(div);
    div.appendChild(document.createTextNode(time.toLocaleString()))
    const img = document.createElement('img')
    img.style.display = 'block'
    div.appendChild(img)
    img.style.backgroundColor = 'black'
    img.style.width = '150px'
    img.onmouseover = evt => {
        clearTimeout(hideTimeout)
        let preview = document.getElementById('preview')
        preview.src = img.src
        preview.style.width = img.naturalWidth + 'px'
        preview.style.height = img.naturalHeight + 'px'
        preview.style.position = 'fixed';
        preview.style.bottom = '16px';
        preview.style.left = '16px';
        preview.style.pointerEvents = 'none'
        preview.style.display = 'block'
        preview.style.zIndex = '100'
        preview.style.background = 'black'
    }
    img.onmouseout = () => {
        hideTimeout = setTimeout(() => {
            document.getElementById('preview').style.display = 'none'
        }, 200)
    }
    blob.then(blob => img.src = URL.createObjectURL(blob));
    // localForage.getItem(id).then(blob => );

    const bt = document.createElement('button')
    div.appendChild(bt)
    bt.textContent = 'Restore'
    bt.style.cursor = 'pointer'
    bt.onclick = () => {
        localForage.getItem(id.slice(0, -':image'.length)).then(config => {
            for (const node of document.querySelectorAll('.selected')) {
                node.classList.remove('selected')
            }
            div.classList.add('selected')
            // console.log('Restoring', config)
            wasm.restore(config)
        })
    }

    const ubt = document.createElement('button')
    div.appendChild(ubt)
    ubt.textContent = 'Update'
    ubt.style.cursor = 'pointer'
    ubt.onclick = log(async () => {
        const config = wasm.save();
        const canvas = document.getElementById('drawing')
        const blob = await new Promise(res => canvas.toBlob(res));
        img.src = URL.createObjectURL(blob);
        await localForage.setItem(id.slice(0, -':image'.length), config);
        await localForage.setItem(id, blob);
    })
}

const setup = async (wasm) => {
    const saved_scenes = (await localForage.keys()).filter(name => name.endsWith(':image')).sort();
    saved_scenes.forEach(id => {
        makeSceneNode(wasm, id, localForage.getItem(id))
    })
    const canvas = document.getElementById('drawing')
    document.getElementById('save').onclick = log(async () => {
        const config = wasm.save();
        const id = Date.now() + ':' + rid();
        const blob = await new Promise(res => canvas.toBlob(res));
        saved_scenes.push(id + ':image')
        makeSceneNode(wasm, id + ':image', Promise.resolve(blob))
        await localForage.setItem(id, config);
        await localForage.setItem(id + ':image', blob);
        console.log('saved!')
    })
}

import('./pkg/zenphoton')
    .then(wasm => {
        wasm.run();
        setup(wasm).catch(err => {
            console.log('Failed')
            console.error(err)
        })
    })
    .catch(console.error);
