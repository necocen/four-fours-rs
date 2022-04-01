import * as Comlink from 'comlink';

async function initFourFours() {
    const fourFours = await import('four-fours');
    // await fourFours.default(); // init
    // await fourFours.initThreadPool(navigator.hardwareConcurrency);
    fourFours.initLog();
    return Comlink.proxy(fourFours.searchWasm);
}

Comlink.expose(initFourFours);
