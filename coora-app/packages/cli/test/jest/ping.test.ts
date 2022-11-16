import { spawnCmd } from './cli.test'



const ip = '192.168.86.222'

describe('ping', () => {

	it('doesnt throw', async () => {
		spawnCmd('ping', ip)
		// expect(stderr.length).toBe(0)
	})

})