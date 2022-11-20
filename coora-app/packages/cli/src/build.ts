import { Command } from 'commander'
import { run } from './runAsc.js'
import { assertExists, consoleErrorOr, parseBuildFileNames } from './utility.js'
export const appendBuildCommand = (parent: Command) => {
	const cmd = parent.command('build')
		.argument('<entry>', 'entrypoint')
	cmd.action(async(entry, _options) => 
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
		'--disable', 'bulk-memory',
		// '--importMemory',
		// '--lowMemoryLimit',
		// '--runtime', 'stub', //this will have big consequences
		// '--stackSize', '65536',
		// '--initialMemory 30',
		//THIS IS BAD, we should implement abort!
		'--use', 'abort=',
		'--use', 'trace=',
		'--use', 'seed=',
		// '--use', 'abort=packages/examples/src/utility/env/abortStub',
		// '--use', 'trace=packages/examples/src/utility/env/traceStub',
		// '--use', 'seed=packages/examples/src/utility/env/seedStub',
	]
	const result = await run(args)
		.catch((err: Error) => err)
	if (result instanceof Error)
		return result
	if (result.error)
		return new Error(`BUILD - failed, compile error \n${result.error}`)
		// err = result.error
	const duration = performance.now() - now
	return { duration, result, names }
}
