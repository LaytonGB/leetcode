class Solution:
    def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if not root: return root
        layer = [root]
        while len(layer) > 0:
            for (n1, n2) in zip(layer, layer[1:]):
                n1.next = n2
            layer = [n for lis in ([n.left, n.right] if n.left and n.right else [n.left] if n.left else [n.right] if n.right else [] for n in layer) for n in lis]
        return root