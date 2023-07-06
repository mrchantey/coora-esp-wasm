# Coora App
The framework used to develop Coora Apps, targeting Web Assembly.

[play](https://stackblitz.com/~/github.com/mrchantey/coora/tree/main/coora-app)

## CLI


### Development

```
npm run watch -w packages/cli
just

```



## Languages

- Assemblyscript
Naturally many languages compile to WASM, but we are starting with writing binding for AssemblyScript.


## notes

- [asconfig options](https://github.com/AssemblyScript/assemblyscript/blob/main/cli/options.json)

```
npx onchange -i -k packages/*/dist/**/* -- node ./packages/cli/dist/main.js watch 58.110.38.98:6222 ./packages/examples/src/entry/hello_led.ts ./packages -f
```