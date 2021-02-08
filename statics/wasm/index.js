import init from './pkg/wasm_game_of_life.js';
import * as wasm from './pkg/wasm_game_of_life.js';

async function run(){
    await init();
    wasm.greet();
}

run();