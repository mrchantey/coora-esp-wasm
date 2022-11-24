import { Command } from 'commander'
import fs from 'fs'
import fetch from 'node-fetch'
import { assertExists, consoleErrorOr, parseResponse } from './utility.js'

export const appendFlashCommand = (parent: Command) => {
	const cmd = parent.command('flash')
		.argument('<ip>', 'ip address')
		.argument('<location>', 'wasm file location')
	cmd.action(async(ip, location) => { 
		flashWithLog(ip, location)
	})
}

export const flashWithLog = async (ip: string, location: string) => {
	console.log('FLASH - uploading..')
	const result = await flash(ip, location)
	consoleErrorOr(await flash(ip, location), ({ duration }) =>
		`FLASH - success - ${duration.toFixed(0)} ms`)
	return result
}

export const flash = async (ip: string, location: string) => {
	const exists = assertExists(location)
	if (exists instanceof Error)
		return exists

	// const path = `../../dist/${location}/release.wasm`
	const stats = fs.statSync(location)
	const readStream = fs.createReadStream(location)
	
	const url = `http://${ip}/sketch`
	const prom = fetch(url, { method: 'POST',
		headers: {
			'Content-length': stats.size.toString()
		}, 
		body: readStream 
	})
	return parseResponse('FLASH', prom, ip)
}