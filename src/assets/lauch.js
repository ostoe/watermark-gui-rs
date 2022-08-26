import { invoke } from '@tauri-apps/api/tauri'
//  import { invoke } from "../../node_modules/@tauri-apps/api/tauri.js";

function btnAction()  {
    console.log("will boo");
}
function btnAction1() {
    // With the Tauri API npm package:
    // With the Tauri global script:
const test = "../../node_modules/@tauri-apps/api/tauri.js";
        const invoke = window.__TAURI__.invoke
         console.log("--" + invoke);
        function btnAction() {
            
            console.log("will boo");
            invoke('close_splashscreen')
            console.log("boo");
        }
    // document.addEventListener('DOMContentLoaded', () => {
    //     // This will wait for the window to load, but you could
    //     // run this function on whatever trigger you want

    // })
}