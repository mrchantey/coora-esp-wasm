//@ts-ignore
import { main } from 'assemblyscript/asc'
import { Command } from 'commander'
import { assertExists, consoleErrorOr, parseBuildFileNames } from './utility.js'
export const appendBuildCommand = (parent: Command) => {
	const cmd = parent.command('build')
		.argument('<entry>', 'entrypoint')
	cmd.action(async(entry, options) => 
		consoleErrorOr(await build(entry), ({ names, duration }) =>
			`BUILD - success - ${names.name} - ${duration.toFixed(0)} millis`))
}

export type BuildTarget = 'release' | 'debug'

export const build = async (entry: string, target: BuildTarget = 'release') => {
	const exists = assertExists(entry)
	if (exists instanceof Error)
		return exists
	
	const now = performance.now()
	const names = parseBuildFileNames(entry, target)
	const args = [
		entry,
		'--config', './config/assemblyscript/asconfig.json',
		'--target', target,
		'-o', names.wasm,
		'-t', names.wat,
		// --stackSize 65536 \
		// --lowMemoryLimit \
	]
	const result = await main(args)
		.catch((err: Error) => err)
	if (result instanceof Error)
		return result
	if (result.error)
		return new Error(`BUILD - failed, compile error \n${result.error}`)
		// err = result.error
	const duration = performance.now() - now
	return { duration, result, names }
}
