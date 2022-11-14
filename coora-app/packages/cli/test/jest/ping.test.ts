import { ping } from '../../src/ping'

const ip = 'http://192.168.86.222/ping'

describe('ping', () => {
	
	beforeEach(() => {
		
	})

	it('works', async () => {
		await expect(ping('http://foo.com')).rejects.toThrow()
		// expect(ping('http://sjkdasjiodas.com')).toThrow()
		// ping("192.168.86.222")
		await expect(ping(ip)).resolves.toBe(true)
	})
})