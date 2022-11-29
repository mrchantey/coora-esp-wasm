import { Command } from 'commander'
import fs from 'fs'
import fetch from 'node-fetch'
import { assertExists, consoleErrorOr, parseResponse } from './utility.js'

export const appendFlashCommand = (parent: Command) => {
	const cmd = parent.command('flash')
		.argument('<ip>', 'ip address')
		.argument('<location>', 'wasm file location')
		.option('-s --save', 'save to nvs storage')
	cmd.action(async(ip, location, options) => { 
		flashWithLog(ip, location, options)
	})
}

type Options = {
	save?: boolean
}

const defaultOptions: Options = {
	save: false
	// save: true
}

export const flashWithLog = async (ip: string, location: string, options?: Options) => {
	console.log('FLASH - uploading..')
	const result = await flash(ip, location, options)
	consoleErrorOr(result, ({ duration }) =>
		`FLASH - success - ${duration.toFixed(0)} ms`)
	return result
}

export const flash = async (ip: string, location: string, options = defaultOptions) => {
	const exists = assertExists(location)
	if (exists instanceof Error)
		return exists

	// const path = `../../dist/${location}/release.wasm`
	const stats = fs.statSync(location)
	const readStream = fs.createReadStream(location)

	const urlPath = options.save ? 'sketch-nvs' : 'sketch'

	const url = `http://${ip}/${urlPath}`
	const prom = fetch(url, { method: 'POST',
		headers: {
			'Content-length': stats.size.toString()
		}, 
		body: readStream 
	})
	return parseResponse('FLASH', prom, ip)
}