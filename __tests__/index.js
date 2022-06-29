const isOdd = require('../dist/index.js')

describe('is-odd', () => {
    it('should return true if number is odd', () => {
        expect(isOdd(1)).toBe(true)
        expect(isOdd(3)).toBe(true)
        expect(isOdd(5)).toBe(true)
        expect(isOdd(7)).toBe(true)
        expect(isOdd(9)).toBe(true)
    })

    it('should return false if number is even', () => {
        expect(isOdd(2)).toBe(false)
        expect(isOdd(4)).toBe(false)
        expect(isOdd(6)).toBe(false)
        expect(isOdd(8)).toBe(false)
        expect(isOdd(10)).toBe(false)
    })

    it('should return false if number is not a number', () => {
        expect(isOdd('1')).toBe(false)
        expect(isOdd(true)).toBe(false)
        expect(isOdd(false)).toBe(false)
        expect(isOdd(null)).toBe(false)
        expect(isOdd(undefined)).toBe(false)
        expect(isOdd({})).toBe(false)
        expect(isOdd([])).toBe(false)
    })
})