import { console, ledStrip, time } from '../bindings'
//for testing
export function ping(): u64{
	return time.elapsed()
}

export function start(): void{
	console.log('🤘🤘lets do this!🤘🤘')
}

export function run(): void{	
	// const a = time.elapsed() as i32 / 100
	ledStrip.setLeds(0, 32, 0, 0)
	ledStrip.show()
}
