// import fs from "fs-extra";
import zlib from "zlib";
import unzip from "zlib"
import fs from 'fs';

// import path from "path";
import AdmZip from "adm-zip";
// import fetch from "node-fetch";
// import proxyAgent from "https-proxy-agent";
// import { execSync } from "child_process";

const { platform, arch } = process;
import tauriConfJson from "../src-tauri/tauri.conf.json" assert { type: 'json' };
// import tauriConfJson   from "../src-tauri/tauri.conf.json";
if (platform === "win32") {
    tauriConfJson.tauri.bundle.resources = ["resources", "winBin/exiftool.exe"]
    // const zip = new AdmZip("./scripts/winBin.zip")
    // zip.extractAllTo("./src-tauri/winBin/", true, true)
} else if (platform === "darwin") {
    tauriConfJson.tauri.bundle.resources = ["resources", "darwinBin"]
    const zip = new AdmZip("./scripts/darwinBin.zip")
    zip.extractAllTo("./src-tauri/darwinBin/", true, true)
}
fs.writeFile(
    "./src-tauri/tauri.conf.json",
    JSON.stringify(tauriConfJson, undefined, 2),
    (err) => {
        if (err) throw err;
        console.log('The file has been saved!');
      }
);

