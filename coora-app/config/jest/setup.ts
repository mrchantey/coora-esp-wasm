

const message = (expected: any, received: any) =>
	`expected:${expected}, received:${received}`

expect.extend({
	toBeFalse: (received: boolean) => ({
		pass: received === false,
		message: () => message(false, received)
	}),
	toBeTrue: (received: boolean) => ({
		pass: received === true,
		message: () => message(true, received)
	})
})