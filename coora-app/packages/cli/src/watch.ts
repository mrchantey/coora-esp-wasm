import { Command } from 'commander'
import fs from 'fs'
import pkg from 'lodash'
import { build, BuildTarget } from './build.js'
import { flash } from './flash.js'
import { assertExists, consoleErrorOr } from './utility.js'
const { debounce } = pkg

export const appendWatchCommand = (parent: Command) => {
	const cmd = parent.command('watch')
		.argument('<ip>', 'ip address')
		.argument('<entry>', 'entrypoint')
		.argument('<watch>', 'watch location')
		.option('-f --flash', 'flash device on build')
	cmd.action(async(ip, location, watchDir, options) => {
		consoleErrorOr(await watch(ip, location, watchDir, options), () => false)		
	})
}
type Options ={
	target: BuildTarget
	flash: boolean
}

const defaultOptions: Options = {
	target: 'release',
	flash: false
}
const watch = async (ip: string, entry: string, watch: string, options: Options = defaultOptions) => {
	const err = assertExists(entry)
	if (err instanceof Error)	
		return err
	const err2 = assertExists(watch)
	if (err2 instanceof Error)
		return err2
	
	const func = async () => {
		console.clear()
		console.log('BUILD - building..')
		const now = performance.now()
		const result = await build(entry, options.target)
		if (result instanceof Error){
			// console.error(result)
			return
		}
		console.log('BUILD - success')
		if (!options.flash)
			return
		console.log('WATCH - uploading...')
		
		await flash(ip, result.names.wasm)
		const duration = performance.now() - now
		console.log(`WATCH - updated in ${duration.toFixed()} ms`)
	}

	await func()
	
	const debounceRun = debounce(func, 100)
	// try {
	fs.watch(watch, { recursive: true }, _ => 
		debounceRun()
	)
	// while (true){}
	// } catch (err){
	// 	return err
	// }
}