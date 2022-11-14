declare global {
	namespace jest {

		interface Matchers<R> {
			toBeTrue: () => CustomMatcherResult
			toBeFalse: () => CustomMatcherResult

		}
	}
}