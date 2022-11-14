import { build, buildSpawn } from 'packages/cli/src/build'

describe('build', () => {
	

	const numTests = 10
	it.skip('works', async () => {
		const start = Date.now()
		for (let i = 0; i < numTests; i++){
			buildSpawn('hello_world')
			console.log('tick')
		}
		console.log(`complete in ${Date.now() - start} ms`)
		
		// await expect(ping('http://foo.com')).rejects.toThrow()
		// expect(ping('http://sjkdasjiodas.com')).toThrow()
		// ping("192.168.86.222")
		// await expect(ping(ip)).resolves.toBe(true)
	})
	it('works', async () => {
		const start = Date.now()
		for (let i = 0; i < numTests; i++){
			await build('hello_world')
			console.log('tick')
		}
		console.log(`complete in ${Date.now() - start} ms`)
		
		// await expect(ping('http://foo.com')).rejects.toThrow()
		// expect(ping('http://sjkdasjiodas.com')).toThrow()
		// ping("192.168.86.222")
		// await expect(ping(ip)).resolves.toBe(true)
	})
})