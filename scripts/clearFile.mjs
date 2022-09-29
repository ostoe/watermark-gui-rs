
import fs from "fs"

fs.stat('./src-tauri/darwinBin/exiftool', function (err, stats) {
    // console.log(stats);
    if (err) {
        return console.log("[warning] no such file or directory");
    }
    fs.rmSync("./src-tauri/darwinBin/lib", { recursive: true, force: true });
    fs.unlink("./src-tauri/darwinBin/exiftool",
        (err) => {
            if (err) throw err;
            console.log('darwinBin/* was deleted')
        }
    )
});
fs.stat('./src-tauri/winBin/exiftool.exe', function (err, stats) {
    // console.log(stats);
    if (err) {
        return console.log("[warning] no such file or directory");
    }
    fs.unlink("./src-tauri/winBin/exiftool.exe",
        (err) => {
            if (err) throw err;
            console.log('winBin/* was deleted')
        }
    )
});