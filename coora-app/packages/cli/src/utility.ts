import fs from 'fs'
import { Response } from 'node-fetch'
import path from 'path'
import { BuildTarget } from './build.js'
export const parseBuildFileNames = (file: string, target: BuildTarget) => {
	const name  = path.basename(file, path.extname(file))
	const wasm = `./dist/${name}/${target}.wasm`
	const wat = `./dist/${name}/${target}.wat`
	return {
		file,
		name,
		wasm,
		wat
	}
}

export const assertExists = (path: string) =>
	fs.existsSync(path) ? true : new Error(`file not found: ${process.cwd()}/${path}`)

export const parseResponse = async (prefix: string, prom: Promise<Response>, ip: string) => {
	const start = performance.now()
	const response = await prom
		.catch(_ => new Error(`${prefix} - failed to connect\n\tip: ${ip}`))
	if (response instanceof Error)
		return response
	if (response.status < 200 || response.status >= 300)
		return new Error(`${prefix} - bad response: ${response.status}\n\tip: ${ip}`)
	const txt = await response.text()
	if (txt !== 'ok')
		return new Error(`${prefix} - expected 'ok', received '${txt}'\n\tip: ${ip}`)
	const duration = performance.now() - start
	return { response, txt, duration }
}

export const consoleErrorOr = <T>(val: Error|T, func: (data: T) => string|false) => {
	if (val instanceof Error){
		console.error(val)
		return val
	}
	const str = func(val)
	if (str !== false)
		console.log(str)
}
