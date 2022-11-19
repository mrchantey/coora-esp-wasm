import { spawnCmd } from './utils'


const spawnBuild = (name: string) => 
	spawnCmd('build', `../examples/src/entry/${name}.ts`)

describe('build', () => {
	
	
	test('build', () => {


		const { stderr, stdout, success } = spawnBuild('compile_error')
		expect(success).toBeFalsy()
		console.dir(stderr)

		// expect(false).toBeTruthy()
	})
})