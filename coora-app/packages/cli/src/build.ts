//@ts-ignore
import { main } from 'assemblyscript/asc'
import { Command } from 'commander'


export const appendBuildCommand = (parent: Command) => {
	const cmd = parent.command('build')
		.argument('<entry>', 'entrypoint')
	cmd.action(async(entry, options) => {
		const { duration } = await build(entry)
		console.log(`BUILD - success - ${name} - ${duration.toFixed(0)} millis`)	
	})
}

type Target = 'release' | 'debug'

export const build = async (name: string, target: Target = 'release') => {
	const now = performance.now()
	const args = [
		`../examples/src/entry/${name}.ts`,
		'--config', '../../config/assemblyscript/asconfig.json',
		'--target', target,
		'-o', `../../dist/${name}/${target}.wasm`,
		'-t', `../../dist/${name}/${target}.wat`,
		// --stackSize 65536 \
		// --lowMemoryLimit \
	]
	const result = await main(args).catch((_: any) => { 
		throw new Error('BUILD - failed, unexpected error') })
	if (result.error)
		throw new Error('BUILD - failed, compile error')
	const duration = performance.now() - now
	return { duration, result }
}
