import {Tree} from './interfaces';

const tree: Tree<number> = {
    value: 1,
    left: {
        value: 2,
        left: {
            value: 4,
            left: {
                value: 8,
                left: null,
                right: null
            },
            right: {
                value: 9,
                left: null,
                right: null
            }
        },
        right: {
            value: 5,
            left: null,
            right: {
                value: 10,
                left: null,
                right: null
            },
        }
    },
    right: {
        value: 3,
        left: {
            value: 6, // leaf node - no branches here
            left: null,
            right: null,
        },
        right: {
            value: 7,
            left: null,
            right: null
        }
    }
}

function calculation_helper(tree: Tree<number> | null, sum: number, sums: number[]): number | undefined {
    if (tree === null) {
        return
    }

    const newSum = sum + tree.value;

    if (tree.left === null && tree.right === null) {
        return sums.push(newSum)  // returned index ignored
    }

    calculation_helper(tree.left, newSum, sums)
    calculation_helper(tree.right, newSum, sums)

}

/**
 * time complexity O(n)
 * space complexity avg O(n)
 * @param tree {Tree}
 */
function branch_sum_recursion(tree: Tree<number>) {
    const sums: number[] = []

    calculation_helper(tree, 0, sums)

    return sums
}

console.log(branch_sum_recursion(tree))
