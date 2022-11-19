import { ledStrip, time } from '../bindings'
//for testing
export function _millis(): u64{
	return time.elapsed()
}

export function start(): void{
	// core.log("howdy")
}

export function run(): void{
	// const a = time.elapsed() as i32 / 100
	ledStrip.setLeds(0, 0, 44, 0)
	ledStrip.show()
}

// export function add(a:i32,b:i32):i32{
// 	return a+b
// }
