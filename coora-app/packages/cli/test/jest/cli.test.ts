import { spawnCmd, spawnSync } from './utils'



describe('cli', () => {
	
	test('spawn', async () => {
		const { stdout } = spawnSync('echo hi')
		expect(stdout).toBe('hi\r\n')
	})

	test('help', async () => {
		const { stdout } = spawnCmd('--help')
		expect(stdout).toContain('coora cli')
	})
})