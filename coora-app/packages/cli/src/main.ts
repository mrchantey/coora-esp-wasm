import { build } from './build.js'

// import fetch from 'node-fetch'
export { }

async function main(){

	const result = await build('pizza')

	// const foo = await fetch('http://www.google.com')
	console.dir(result)


}
main()
