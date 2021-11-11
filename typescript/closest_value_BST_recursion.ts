type Tree = {
    value: number;
    left: Tree;
    right: Tree
} | null

const tree1: Tree = {
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

function find_closest_value_BST_helper(tree: Tree, target: number, closest: number): number {
    // Base case - return without making a recursive call
    if (!tree) {
        return closest
    }

    if (Math.abs(target - closest) > Math.abs(target - tree.value)) {
        closest = tree.value
    }

    // Choose branch, in bst left branch nodes always lesser than root
    if (target < tree.value) {
        return find_closest_value_BST_helper(tree.left, target, closest)
    }
    if (target > tree.value) {
        return find_closest_value_BST_helper(tree.right, target, closest)
    }

    // if tree.value is equal to target
    return closest
}


/**
 * Search of closes value in binary search tree
 *
 * average complexity O(log(n))
 * worst is O(n) is one branch scenario
 * Space is the same as complexity because of recursion
 *
 * @param tree
 * @param target
 *
 * @returns {number}
 */
function find_closest_value_BST_recursion(tree: Tree, target: number) {
    return find_closest_value_BST_helper(tree, target, Infinity)
}


console.log(find_closest_value_BST_recursion(tree1, 12))
