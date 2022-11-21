import { console, ledStrip, time } from '../bindings'
//for testing
export function ping(): u64{
	return time.elapsed()
}

export function start(): void{
	console.log('🤘🤘lets do this pwoita!🤘🤘')
}

export function getValue(): Array<u32>{
	const arr: Array<u32> = []
	arr.push(10)
	return arr
}

export function run(): void{	
	// const a = time.elapsed() as i32 / 100
	const arr: Array<u32> = []
	// arr.slice(2, 2)
	// arr.push(10)
	arr.push(10 as u32)
	const a = arr[0]
	// console.log(`${arr.length}`)
	// console.log('🤘🤘lets do this monkey doodle!🤘🤘')
	ledStrip.setLeds(0, 0, 0, 6)
	ledStrip.show()
}