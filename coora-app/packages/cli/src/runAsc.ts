//@ts-ignore dunno
import { main } from 'assemblyscript/asc'


export const run = async (args: string[]) => {

	return await main(args, {
		// readFile(filename, baseDir) {
		// 	const filePath = path.join(baseDir, filename)
		// 	if (fileMap.has(filePath)) return fileMap.get(filePath)!
		// 	try {
		// 		const contents = readFileSync(filePath, 'utf8')
		// 		fileMap.set(filePath, contents)
		// 		return contents
		// 	} catch (ex) {
		// 		return null
		// 	}
		// },
		// writeFile(filename, contents, _baseDir) {
		// 	files.set(filename, contents)
		// },
		// listFiles(dirname, baseDir) {
		// 	const folder = path.join(baseDir, dirname)
		// 	if (folderMap.has(folder)) return folderMap.get(folder)!
	
		// 	try {
		// 		const files = readdirSync(folder).filter((file) => /^(?!.*\.d\.ts$).*\.ts$/.test(file))
		// 		folderMap.set(folder, files)
		// 		return files
		// 	} catch (ex) {
		// 		return null
		// 	}
		// },
		stderr: process.stderr,
		stdout: process.stdout,
	})
	
	// if (compiled.error) {
	// 	console.error(compiled.error)
	// 	process.exit(1)
	// }

}
