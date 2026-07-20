class Solution {
public:
    vector<int> postorder(Node* root) {
        vector<int> res;
        r(root, &res);
        return res;
    }

    void r(Node* root, vector<int>* res) {
        if (!root)
        {
            return;
        }

        for (Node* node : root->children)
        {
            r(node, res);
        }

        res->push_back(root->val);
    }
};