// memory.

export function _size(): u32{
	return memory.size()
}
export function _load(ptr: i32): i32{
	// const a = 2
	return load<i32>(ptr)
}
export function _store(ptr: u32, val: i32): void{
	return store<i32>(ptr, val)
}

// export function add(a: i32, b: i32): i32{
// 	return a + b
// }

export function getString(): u32{
	const str = 'hello from wasm'
	return changetype<usize>(str) as u32
}
export function printHello(): void{
	const str = 'hello from wasm'
	println(str)
}

//@ts-ignore externam
@external("Serial", "println")
declare function _println(ptr: usize, len: i32): void;

export function println(str: string): void {
	const out = String.UTF8.encode(str)
	_println(changetype<usize>(out), out.byteLength)
}

export const a: string = '\1\2\3\4'