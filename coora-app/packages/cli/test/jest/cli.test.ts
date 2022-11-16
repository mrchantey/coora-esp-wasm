import crossSpawn from 'cross-spawn'

export const spawnCmd = (...args: string[]) =>
	spawnSync('node ./dist/main.js', ...args)


export const spawnSync = (...args: string[]) => {
	args = args.flatMap(arg => arg.split(' '))
	const result = crossSpawn.sync(args.shift()!, args, {
		cwd: 'packages/cli',
		// stdio: 'inherit',
		shell: true,
	})
	const stderr = result.stderr.toString()
	const stdout = result.stdout.toString()
	if (stderr.length > 0)
		throw new Error(stderr)
	return {  stdout }
}

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