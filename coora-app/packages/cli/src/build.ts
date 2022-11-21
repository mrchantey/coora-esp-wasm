import { Command } from 'commander'
import fs from 'fs'
import { run } from './runAsc.js'
import { assertExists, consoleErrorOr, parseBuildFileNames } from './utility.js'
export const appendBuildCommand = (parent: Command) => {
	const cmd = parent.command('build')
		.argument('<entry>', 'entrypoint')
	cmd.action(async(entry, _options) => { buildWithLog(entry) })
}

export type BuildTarget = 'release' | 'debug'

export const buildWithLog = async(entry: string, target?: BuildTarget) => {
	const result = await build(entry, target)
	consoleErrorOr(result, ({ names, duration, size }) =>
		`BUILD - success - ${names.name} - ${duration.toFixed(0)} ms - ${(size / 1024).toFixed(2)} KB`)
	return result
}

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
		'--noExportMemory',
		'--importMemory',
		//-------------------------^^ required

		'--initialMemory', '1',
		'--maximumMemory', '1',
		'--optimizeLevel', '0', //0,1,2,3
		'--shrinkLevel', '2', //0,1,2
		// '-Osize',
		// '--lowMemoryLimit',
		// '--runtime', 'minimal',
		// '--exportRuntime',
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
	const size = fs.statSync(names.wasm).size
	return { duration, result, names, size }
}
