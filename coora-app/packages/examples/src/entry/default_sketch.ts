import { debug, ledStrip, math, time } from '../bindings'
//for testing
export function ping(): u64{
	return time.elapsed()
}

export function start(): void{
	debug.log('🤘running default sketch🤘')
	ledStrip.setLeds(0, 0, 0, 0)
	ledStrip.show()
}

export function update(): void{	
	const t = math.sin((time.elapsed() / 1000) as f32) / 2 + 0.5
	ledStrip.setLeds(0, t * 8 as u32, 0, 0)
	ledStrip.show()

}