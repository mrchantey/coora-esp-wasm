import { debug, ledStrip, math, time } from '../bindings'
//for testing
export function ping(): u64{
	return time.elapsed()
}

export function start(): void{
	ledStrip.setLeds(0, 0, 0, 0)
	ledStrip.show()
	debug.log('hello led!🤘🤘🤘🤘🤘🤘🤘🤘🤘🤘🤘🤘🤘🤘')
}

export function update(): void{	
	const t = math.sin((time.elapsed() as f32) / 1000) / 2 + 0.5
	ledStrip.setLeds(0, t * 32 as u32, 0, 0)
	ledStrip.show()

} 