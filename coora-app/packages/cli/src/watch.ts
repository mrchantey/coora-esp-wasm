import { Command } from 'commander'
import fs from 'fs'
import pkg from 'lodash'
import { build } from './build.js'
import { flash } from './flash.js'
const { debounce } = pkg

export const appendWatchCommand = (parent: Command) => {
	const cmd = parent.command('watch')
		.argument('<ip>', 'ip address')
		.argument('<location>', 'wasm file location')
	cmd.action(async(ip, location, options) => {
		await watch(ip, location)
	})
}


const watch = (ip: string, name: string) => {

	const func = async () => {
		const now = performance.now()
		await build(name)
		await flash(ip, name)
		const duration = performance.now() - now
		console.log(`WATCH - updated in ${duration.toFixed()} ms`)
	}

	func()
	
	const debounceRun = debounce(func, 100)
	fs.watch('../examples/src', { recursive: true }, _ => 
		debounceRun()
	)
}