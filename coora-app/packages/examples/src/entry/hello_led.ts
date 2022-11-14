import { core, led } from '../bindings'
import { millis } from '../bindings/core'

//for testing
export function _millis(): u64{
	return core.millis()
}

export function start(): void{
	// core.log("howdy")
}

export function run(): void{
	const a = millis() as i32 / 100
	led.setAll(32, 0, 0, 2)
	led.show()
}

// export function add(a:i32,b:i32):i32{
// 	return a+b
// }
