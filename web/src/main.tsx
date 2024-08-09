// import * as init from "../../pkg/rs_wgpu_engine";\
// init.run();
// console.log("init", init);
// // init().then(() => {
// //     console.log("WASM Loaded");
// // });
// @ts-ignore
import wasmUri from "../pkg/rs-wgpu-engine_bg.wasm";
// @ts-ignore
import * as importObject from "../pkg/rs-wgpu-engine_bg";

console.log("wasmUri", wasmUri);
console.log("importObject", importObject);

function loadWasm() {
    const config = {
        "./rs-wgpu-engine_bg.js": importObject,
    };
    if (typeof WebAssembly.compileStreaming == "function") {
        return WebAssembly.instantiateStreaming(fetch(wasmUri as any), config);
    }
    return fetch(wasmUri)
        .then(response =>
            response.arrayBuffer()
        ).then(bytes =>
            WebAssembly.instantiate(bytes, config)
        );
}

async function main() {
    const m = await loadWasm();
    const wasm = m.instance.exports;
    console.log("wasm", wasm);
    importObject.__wbg_set_wasm(wasm);
    // @ts-ignore
    wasm.run();
    console.log(m);
}

main();


export {};
