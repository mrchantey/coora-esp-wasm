import { Command } from 'commander'
import fetch from 'node-fetch'
import { parseResponse } from './utility.js'

export const appendPingCommand = (parent: Command) => {
	const cmd = parent.command('ping')
		.argument('<ip-address>', 'IP Address, ie 192.168.39.28')
		.option('-f', 'something')
	// .arg
	// .argument(flags, description, fn)
	cmd.action(async (ip, options) => {
		const { duration } = await ping(ip)
		console.log(`ok - ${duration.toFixed(0)} ms`)
	})
}

export const ping = async (ip: string) => {
	//we need to make this a get
	const url = `http://${ip}/ping`
	const prom = fetch(url, { method: 'POST' })
	return parseResponse('PING', prom, ip)
}
