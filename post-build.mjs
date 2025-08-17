import fs from 'node:fs'
import path from "node:path"


function find(dir, termination) {
  for(let n of fs.readdirSync(dir)) {
    if(n.endsWith(termination)) return n
  }
}

// identify built files
const dir = "dist",
  htmlFile = path.resolve(dir, find(dir, ".html")),
  jsFile = find(dir, ".js"),
  wasmFile = find(dir, ".wasm")

// rename files to standard files
const token = jsFile.replace(/([a-zA-Z_]+)-([0-9a-z]*)\.js/, "$2"),
  newJSfilename = jsFile.replace(/([a-zA-Z_]+)-([0-9a-z]*)\.js/, "$1.js"),
  newWasmFilename = wasmFile.replace(/([a-zA-Z_]+)-([0-9a-z]*)_bg\.wasm/, "$1.wasm")


// load HTML and reformulate it
let html = fs.readFileSync(htmlFile, {encoding: "utf-8"})
html = html.replaceAll(jsFile, newJSfilename + "?version=" + token).
    replaceAll(wasmFile, newWasmFilename + "?version=" + token)
fs.writeFileSync(htmlFile, html, {encoding:"utf-8"})

fs.renameSync(path.resolve(dir,jsFile), path.resolve(dir,newJSfilename))
fs.renameSync(path.resolve(dir,wasmFile), path.resolve(dir,newWasmFilename))
