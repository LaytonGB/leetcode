/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

class FindElements {
    root: TreeNode;
    
    constructor(root: TreeNode) {
        this.root = root;
        this.reset();
    }

    reset() {
        this.resetNode({node: this.root, value: 0})
    }

    resetNode({node, value}: {node: TreeNode, value: number}) {
        node.val = value;
        
        if (node.left !== null) {
            this.resetNode({node: node.left, value: 2 * value + 1});
        }
        if (node.right !== null) {
            this.resetNode({node: node.right, value: 2 * value + 2});
        }
    }

    find(target: number): boolean {
        return this.findNode({target, node: this.root});
    }

    findNode({target, node}: {target: number, node: TreeNode}): boolean {
        if (target < node.val) {
            return false;
        }

        if (target === node.val) {
            return true;
        }

        return node.left !== null && this.findNode({target, node: node.left})
            || node.right !== null && this.findNode({target, node: node.right});
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * var obj = new FindElements(root)
 * var param_1 = obj.find(target)
 */