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
    }

    find(target: number): boolean {
        let vals = [];
        while (target > 0) {
            vals.push(target);

            if (target % 2 === 0) {
                target = (target - 2) / 2;
            } else {
                target = (target - 1) / 2;
            }
        }

        vals.reverse();
        
        let node = this.root;
        let nodeVal = 0;
        for (const i in vals) {            
            if (vals[i] === nodeVal * 2 + 1) {
                node = node.left;
                nodeVal = nodeVal * 2 + 1;
            } else {
                node = node.right;
                nodeVal = nodeVal * 2 + 2;
            }

            if (node === null) {
                return false;
            }
        }

        return true;
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * var obj = new FindElements(root)
 * var param_1 = obj.find(target)
 */