import init from './pkg/wasm.js';
import * as wasm from './pkg/wasm.js';

async function run(){
    await init();
    wasm.greet();
}

run();