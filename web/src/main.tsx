// import * as init from "../../pkg/rs_wgpu_engine";\
// init.run();
// console.log("init", init);
// // init().then(() => {
// //     console.log("WASM Loaded");
// // });
import wasmUri from "../../pkg/rs_wgpu_engine_bg.wasm";
// @ts-ignore
import { __wbg_set_wasm } from "../../pkg/rs_wgpu_engine_bg";

console.log("wasmUri", wasmUri);
function loadWasm() {
    return fetch(wasmUri as any).then(response =>
        response.arrayBuffer()
    ).then(bytes =>
        WebAssembly.compile(bytes)
    ).then(module => {
        return new WebAssembly.Instance(module);
    });
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
