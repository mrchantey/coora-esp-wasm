import { Command } from 'commander'
import { appendBuildCommand } from './build.js'
import { appendFlashCommand } from './flash.js'
import { appendPingCommand } from './ping.js'
import { appendWatchCommand } from './watch.js'

const helpMessage = `
🤘coora cli🤘
`

export const appendFooCommand = (parent: Command) => {
	const cmd = parent.command('foo')
		// .argument('<ip>', 'ip address')
		// .argument('<entry>', 'entrypoint')
		// .argument('<watch>', 'watch location')
		.option('<target>', 'select a target')
		.option('-f --flash', 'flash device on build')
		.option('-d --dino', 'is dino')
		.action(args => {
			console.dir(args)
		// consoleErrorOr(await watch(ip, location, watchDir, options), () => false)		
		})
}

async function main(args = process.argv){
	try {
		const cmd = new Command('main')
			.version('0.0.1', '-v -V --version')
			.on('--help', () => console.log(helpMessage))
		appendPingCommand(cmd)
		appendBuildCommand(cmd)
		appendFlashCommand(cmd)
		appendWatchCommand(cmd)
		appendFooCommand(cmd)
		cmd.parse(args)
	} catch (exc){
		console.error('something went wrong..')
		console.error(exc)		
	}
}
main()
