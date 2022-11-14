import { spawnCmd } from './cli.test'



const ip = '192.168.86.222'

describe('ping', () => {

	test('ping', async () => {
		const result = spawnCmd('ping', ip)
		expect(result).toContain('ok')
		const duration = parseInt(result.split('-')[1])
		expect(duration).toBeLessThan(1000)
		expect(duration).toBeGreaterThan(100)
	})

})