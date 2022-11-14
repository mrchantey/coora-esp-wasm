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
	const err = result.stderr.toString()
	const out = result.stdout.toString()
	if (err)
		throw new Error(err)
	return out
}

describe('ping', () => {
	
	test('spawn', async () => {
		const result = spawnSync('echo hi')
		expect(result).toBe('hi\r\n')
	})

	test('help', async () => {
		const result = spawnCmd('node ./dist/main.js --help')
		expect(result).toContain('coora cli')
	})
})