import { Command } from 'commander'
import fs from 'fs'
import pkg from 'lodash'
import { BuildTarget, buildWithLog } from './build.js'
import { flashWithLog } from './flash.js'
import { assertExists, consoleErrorOr } from './utility.js'
const { debounce } = pkg

export const appendWatchCommand = (parent: Command) => {
	const cmd = parent.command('watch')
		.argument('<ip>', 'ip address')
		.argument('<entry>', 'entrypoint')
		.argument('<watch>', 'watch location')
		.option('<target>',)
		.option('-f --flash', 'flash device on build')
	cmd.action(async(ip, location, watchDir, options) => {
		consoleErrorOr(await watch(ip, location, watchDir, options), () => false)		
	})
}
type Options ={
	target?: BuildTarget
	flash?: boolean
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
		// const now = performance.now()
		const result = await buildWithLog(entry, options.target)
		if (result instanceof Error){
			// console.error(result)
			return
		}
		if (!options.flash)
			return
		await flashWithLog(ip, result.names.wasm)
		// const duration = performance.now() - now
		// console.log(`COORA - success in ${duration.toFixed()} ms`)
	}

	await func()
	
	const debounceRun = debounce(func, 100)
	// try {
	fs.watch('./config', { recursive: true }, _ => 
		debounceRun()
	)
	fs.watch(watch, { recursive: true }, _ => 
		debounceRun()
	)
	// while (true){}
	// } catch (err){
	// 	return err
	// }
}