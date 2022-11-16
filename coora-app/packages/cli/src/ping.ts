import { Command } from 'commander'
import fetch from 'node-fetch'
import { consoleErrorOr, parseResponse } from './utility.js'

export const appendPingCommand = (parent: Command) => {
	const cmd = parent.command('ping')
		.argument('<ip-address>', 'IP Address, ie 192.168.39.28')
		.option('-f', 'something')
	// .arg
	// .argument(flags, description, fn)
	cmd.action(async (ip, options) => 		
		consoleErrorOr(await ping(ip), ({ duration }) =>
			`PING - ok - ${duration?.toFixed(0)} ms`))
}

export const ping = async (ip: string) => {
	//we need to make this a get
	const url = `http://${ip}/ping`
	const prom = fetch(url, { method: 'GET' })
	return parseResponse('PING', prom, ip)
}
