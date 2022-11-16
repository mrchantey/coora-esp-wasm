import { Command } from 'commander'
import fs from 'fs'
import fetch from 'node-fetch'
import { assertExists, consoleErrorOr, parseResponse } from './utility.js'

export const appendFlashCommand = (parent: Command) => {
	const cmd = parent.command('flash')
		.argument('<ip>', 'ip address')
		.argument('<location>', 'wasm file location')
	cmd.action(async(ip, location, options) => {
		consoleErrorOr(await flash(ip, location), ({ duration }) =>
			`FLASH - OK - completed in ${duration.toFixed(0)} ms`)
	})
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