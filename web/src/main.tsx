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
    const config = {
        "./index_bg.js": importObject,
    }
    if (typeof WebAssembly.compileStreaming == "function") {
        return WebAssembly.instantiateStreaming(fetch(wasmUri as any), config);
    }
    return fetch(wasmUri)
        .then(response =>
            response.arrayBuffer()
        ).then(bytes =>
            WebAssembly.instantiate(bytes, config)
        )
}

async function main() {
    const m = await loadWasm();
    const wasm = m.instance.exports;
    importObject.__wbg_set_wasm(wasm);
    // @ts-ignore
    wasm.__wbindgen_start();
    console.log(m);
}

main();


export {};
