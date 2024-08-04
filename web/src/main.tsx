// import * as init from "../../pkg/rs_wgpu_engine";\
// init.run();
// console.log("init", init);
// // init().then(() => {
// //     console.log("WASM Loaded");
// // });
import wasmUri from "../pkg/index_bg.wasm";
// @ts-ignore
import * as importObject from "../pkg/index_bg";

console.log("wasmUri", wasmUri);
console.log("importObject", importObject);
function loadWasm() {
    return WebAssembly.instantiateStreaming(fetch(wasmUri as any), importObject);
}

async function main() {
    const wasm = await loadWasm();
    console.log(wasm);
    // __wbg_set_wasm(wasm);
    //
    // wasm.__wbindgen_start();
}

main();


export {};
