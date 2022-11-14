//until stackblitz supports node 17
// import fetch from 'node-fetch'
export { }

console.log('howdy!')
async function main(){

	const foo = await fetch('http://www.google.com')
	console.dir(foo)


}
main()
