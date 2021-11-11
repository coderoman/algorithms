type Tree = {
    value: number;
    left: Tree;
    right: Tree
} | null

const tree: Tree = {
    value: 10,
    left: {
        value: 5,
        left: {
            value: 2,
            left: {
                value: 1,
                left: null,
                right: null
            },
            right: null
        },
        right: {
            value: 5,
            left: null,
            right: null
        }
    },
    right: {
        value: 15,
        left: {
            value: 13,
            right: {
                value: 14,
                left: null,
                right: null
            },
            left: null
        },
        right: {
            value: 22,
            right: null,
            left: null
        }
    }
}


/**
 * Search of closes value in binary search tree
 *
 * average complexity O(log(n))
 * worst is O(n) is one branch scenario
 * Space is the O(1) - constant
 *
 * @param tree
 * @param target
 *
 * @returns {number}
 */
function find_closest_value_BST_iteration(tree: Tree, target: number) {
    let current = tree
    let closest = Infinity

    while (current) {
        if (Math.abs(target - closest) > Math.abs(target - current.value)) {
            closest = current.value
        }

        if (target < current.value) {
            current = current.left
        } else if (target > current.value) {
            current = current.right
        } else if (target === current.value) {
            closest = current.value
            return
        }
    }

    return closest
}


console.log(find_closest_value_BST_iteration(tree, 12))
