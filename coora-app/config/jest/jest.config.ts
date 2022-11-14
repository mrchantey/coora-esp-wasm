import type { Config } from '@jest/types'
import type { InitialOptionsTsJest } from 'ts-jest'
const repoRoot = '<rootDir>/'
const jestConfigRoot = repoRoot + 'config/jest/'
const setupFilesAfterEnv = [jestConfigRoot + 'setup.ts']

const dontIgnore = [
	'node-fetch',
	'data-uri-to-buffer',
	'fetch-blob',
	'formdata-polyfill',
	'assemblyscript'
].join('|')

const globalJestIgnores = [
	'/node_modules/',
	repoRoot + 'packages/devops/dist-functions',
]

export default {
	rootDir: process.cwd(),
	//https://kulshekhar.github.io/ts-jest/docs/getting-started/options/tsconfig/
	transform: {
		'^.+\\.[tj]sx?$': ['ts-jest', {
			tsconfig: jestConfigRoot + 'tsconfig.json',
			useESM: true,
			isolatedModules: true//https://stackoverflow.com/questions/45087018/jest-simple-tests-are-slow
		}],
	},
	testEnvironment: 'node',
	modulePaths: [repoRoot],
	// transformIgnorePatterns: [],
	transformIgnorePatterns: [`<rootDir>/node_modules/(?!${dontIgnore})`],
	watchPathIgnorePatterns: [...globalJestIgnores],
	modulePathIgnorePatterns: [...globalJestIgnores],
	testPathIgnorePatterns: [...globalJestIgnores],
	coveragePathIgnorePatterns: [...globalJestIgnores],
	testMatch: [
		// repoRoot + `packages/${props.package ?? '*'}/test/${testDirName}/**/*.test.{ts,tsx}`
		repoRoot + 'packages/*/test/jest/**/*.test.{ts,tsx}'
	],
	roots: [
		repoRoot + 'packages/',
	],
	setupFilesAfterEnv,
	// preset: 'ts-jest',
	preset: 'ts-jest/presets/js-with-ts',
	// globals: {
	// 	'ts-jest': {
	// 		tsconfig: jestConfigRoot + 'tsconfig.json',
	// 	}
	// }

} as Config.InitialOptions| InitialOptionsTsJest