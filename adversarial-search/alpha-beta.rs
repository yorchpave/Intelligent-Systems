use std::cmp::{min, max};

#[derive(Debug)]
struct TreeNode {
    childs: Option<Vec<TreeNode>>,
    value: Option<i32>
}

impl TreeNode {
    fn new_with_value(value: i32) -> TreeNode {
        TreeNode { childs: Option::None, value: Option::Some(value) }
    }

    fn new_with_childs(child: Vec<TreeNode>) -> TreeNode {
        TreeNode { childs: Option::Some(child), value: Option::None }
    }

    fn is_leaf(&self) -> bool {
        match self.childs {
            Option::None => true,
            _ => false
        }
    }
}

struct Game;

impl Game {
    fn alpha_beta_pruning(root:TreeNode) -> i32 {
        Game::ab_pruning(root, i32::min_value(), i32::max_value(), true)
    }

    fn ab_pruning(root:TreeNode, alpha:i32, beta:i32, is_maximizing_player:bool) -> i32 {
        if root.is_leaf() {
            return root.value.unwrap_or(0);
        }

        let children = root.childs.unwrap();
        if is_maximizing_player {
            return Game::max_player(children, alpha, beta);
        }

        Game::min_player(children, alpha, beta)
    }

    fn min_player(children:Vec<TreeNode>, alpha:i32, beta:i32) -> i32 {
        let mut value = 0;
        for child in children {
            value = min(i32::max_value(), Game::ab_pruning(child, alpha, beta, true));
            let beta = min(beta, value);
            if alpha >= beta {
                break;
            }
        }
        return value;
    }

    fn max_player(children:Vec<TreeNode>, alpha:i32, beta:i32) -> i32 {
        let mut value = 0;
        for child in children {
            value = max(i32::min_value(), Game::ab_pruning(child, alpha, beta, false));
            let alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        return value;
    }
}


fn main() {
    // CASE 1
    let tree_a = get_tree_a();
    println!("Tree: {:?}", &tree_a);
    let result = Game::alpha_beta_pruning(tree_a);
    println!("Result -> {}", result);

    //CASE 2
    let tree_b = get_tree_b();
    println!("Tree: {:?}", &tree_b);
    let result = Game::alpha_beta_pruning(tree_b);
    println!("Result -> {}", result);

    //EXTRA TREE
    let extra_tree = get_extra_tree();
    println!("Tree: {:?}", &extra_tree);
    let result = Game::alpha_beta_pruning(extra_tree);
    println!("Result -> {}", result);
}

fn get_tree_a() -> TreeNode {
    let l_node = TreeNode::new_with_value(2);
    let s_node = TreeNode::new_with_value(3);
    let t_node = TreeNode::new_with_value(5);
    let u_node = TreeNode::new_with_value(4);
    let v_node = TreeNode::new_with_value(3);

    let k_node = TreeNode::new_with_childs(vec![s_node, t_node]);
    let m_node = TreeNode::new_with_childs(vec![u_node, v_node]);

    let e_node = TreeNode::new_with_childs(vec![l_node, k_node, m_node]);
    TreeNode::new_with_childs(vec![e_node])
}

fn get_tree_b() -> TreeNode {
    let l_node = TreeNode::new_with_value(2);
    let s_node = TreeNode::new_with_value(3);
    let t_node = TreeNode::new_with_value(5);
    let u_node = TreeNode::new_with_value(4);
    let v_node = TreeNode::new_with_value(3);

    let k_node = TreeNode::new_with_childs(vec![s_node, t_node]);
    let m_node = TreeNode::new_with_childs(vec![u_node, v_node]);

    let e_node = TreeNode::new_with_childs(vec![k_node, m_node, l_node]);
    TreeNode::new_with_childs(vec![e_node])
}

fn get_extra_tree() -> TreeNode {
    let a_node = TreeNode::new_with_value(3);
    let b_node = TreeNode::new_with_value(7);
    let c_node = TreeNode::new_with_value(5);

    let subtree_a = get_subtree_a();
    let subtree_b = get_subtree_b();
    let subtree_c = get_subtree_c();

    let d_node = TreeNode::new_with_childs(vec![subtree_a, a_node]);
    let e_node = TreeNode::new_with_childs(vec![subtree_b, b_node]);
    let f_node = TreeNode::new_with_childs(vec![c_node, subtree_c]);

    TreeNode::new_with_childs(vec![d_node, e_node, f_node])
}

fn get_subtree_a() -> TreeNode {
    let a_node = TreeNode::new_with_value(5);
    let b_node = TreeNode::new_with_value(6);
    let c_node = TreeNode::new_with_value(7);
    let d_node = TreeNode::new_with_value(4);
    let e_node = TreeNode::new_with_value(5);

    let f_node = TreeNode::new_with_childs(vec!(a_node, b_node));
    let g_node = TreeNode::new_with_childs(vec!(c_node, d_node, e_node));

    TreeNode::new_with_childs(vec![f_node, g_node])
}

fn get_subtree_b() -> TreeNode {
    let a_node = TreeNode::new_with_value(6);
    let b_node = TreeNode::new_with_value(6);
    let c_node = TreeNode::new_with_value(9);
    
    let d_node = TreeNode::new_with_childs(vec![b_node, c_node]);

    TreeNode::new_with_childs(vec![a_node, d_node])
}

fn get_subtree_c() -> TreeNode {
    let a_node = TreeNode::new_with_value(9);
    let b_node = TreeNode::new_with_value(8);
    let c_node = TreeNode::new_with_value(6);

    let d_node = TreeNode::new_with_childs(vec![a_node, b_node]);

    TreeNode::new_with_childs(vec![d_node, c_node])
}
