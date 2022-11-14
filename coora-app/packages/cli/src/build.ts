// import asc from 'assemblyscript/dist/asc'
//@ts-ignore
import { main } from 'assemblyscript/asc'
// import crossSpawn from 'cross-spawn'
// import { main } from './asc/asc'
// const { main } = require('assemblyscript/dist/asc.js')


// @build name *args:
// cd ./packages/examples && \
// {{npx}} asc ./src/entry/{{name}}.ts \
// -o ../../dist/{{name}}/{{target}}.wasm \
// -t ../../dist/{{name}}/{{target}}.wat \
// --config ./config/assemblyscript/asconfig.json \
// --target {{target}} \
// --stackSize 65536 \
// --lowMemoryLimit \

type Target = 'release' | 'debug'

const getArgs = (name: string, target: Target = 'release') => [
	`./src/entry/${name}.ts`,
	'-o', `../../dist/${name}/${target}.wasm`,
	'-t', `../../dist/${name}/${target}.wat`,
	'--config', './config/assemblyscript/asconfig.json',
	'--target', target,
]


export const build = async (name: string, target: Target = 'release') => {
	// const { main } = await import('assemblyscript/dist/asc')

	return await main(getArgs(name, target))
}



// export const buildSpawn = (name: string, target: Target = 'release') => {	
// 	const args = ['asc', ...getArgs(name, target)]
// 	return crossSpawn.sync('npx', args, {
// 		cwd: 'packages/examples',
// 		stdio: 'inherit',
// 		shell: true,
// 	})


// }