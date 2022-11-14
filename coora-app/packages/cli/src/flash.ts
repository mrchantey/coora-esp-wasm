import { Command } from 'commander'
import fs from 'fs'
import fetch from 'node-fetch'
import { parseResponse } from './utility.js'

export const appendFlashCommand = (parent: Command) => {
	const cmd = parent.command('flash')
		.argument('<ip>', 'ip address')
		.argument('<location>', 'wasm file location')
	cmd.action(async(ip, location, options) => {
		await flash(ip, location)
	})
}

export const flash = async (ip: string, location: string) => {

	const path = `../../dist/${location}/release.wasm`
	const stats = fs.statSync(path)
	const readStream = fs.createReadStream(path)
	
	const url = `http://${ip}/sketch`
	const prom = fetch(url, { method: 'POST',
		headers: {
			'Content-length': stats.size.toString()
		}, 
		body: readStream 
	})
	return parseResponse('FLASH', prom, ip)
}