import { Command } from 'commander'
import { appendBuildCommand } from './build.js'
import { appendFlashCommand } from './flash.js'
import { appendPingCommand } from './ping.js'
import { appendWatchCommand } from './watch.js'

const helpMessage = `
🤘coora cli🤘
`


async function main(args = process.argv){
	const cmd = new Command('main')
		.version('0.0.1', '-v -V --version')
		.on('--help', () => console.log(helpMessage))
	appendPingCommand(cmd)
	appendBuildCommand(cmd)
	appendFlashCommand(cmd)
	appendWatchCommand(cmd)

	cmd.parse(args)
}
main()

