/**
 * complexity O(n)
 * space O(n)
 *
 * @param array
 * @param sum
 */
function two_numbers(array: number[], sum: number): number[] {
    const acc: { [key: string]: boolean } = {};

    for (let i = 0; i < array.length; i++) {
        const summand = sum - array[i];

        if (acc[array[i]]) {
            return [summand, array[i]]
        }

        acc[summand] = true
    }

    return [0, 0]
}


console.log('two_numbers:', two_numbers([3, 5, -4, 8, 11, 11, -1, 6], 10))
console.log('two_numbers:', two_numbers([3, -5, 4, 8, 13, 12, -1, 6], 10))
