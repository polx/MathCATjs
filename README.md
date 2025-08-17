# MathCATjs: JavaScript API to the Math Capable Assistive Technology
MathCAT is a rust library that supports conversion of MathML to speech and braille among other things.
Visit [the MathCAT project page](https://nsoiffer.github.io/MathCAT/) for more info or if you want to play around, [try out the demo](https://nsoiffer.github.io/MathCATDemo/) of which this project gets its basis.
This project offers a basic javascript interface for inclusion of MathCAT in web-pages.

## Local builds
To build this and run locally, you need to download and install [trunk](https://docs.trunk.io/docs/install). Then type
```
trunk serve
```

## Static builds

```
trunk build
node postbuild.mjs
```

The built directory (`dist`) contains a `.wasm` and a `.js` file which are both needed
to run MathCAT. The HTML file contains a sample JS to initialize and use.

## TODOs

Package it into a simple module API that is imported.
Document it.
Expand it to contain more than `initMathCAT` and `mathmlToSpeech`.

Make it available on NPM and make it compatible to packaging systems such as webpack.

