pub mod common;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

#[derive(Debug)]
struct Command<'a> {
    command: String,
    result: Vec<&'a str>,
}

// struct Folder {
//     path: String,
// }
#[derive(PartialEq, Debug)]
enum NodeType {
    FOLDER,
    FILE,
}
#[derive(Debug)]
struct TreeNode {
    size: usize,
    node_type: NodeType,
    name: String,
    children: Vec<TreeNode>,
    parent: &'static mut TreeNode,
    // parent: *const Option<&'static TreeNode>,
}

impl TreeNode {
    pub fn new(
        node_type: NodeType,
        node_name: String,
        // parent: *const Option<&'static TreeNode>,
        parent: &'static mut TreeNode,
    ) -> TreeNode {
        return TreeNode {
            size: 0,
            node_type: node_type,
            name: node_name,
            children: vec![],
            parent: parent,
        };
    }

    pub fn cd(&mut self, dest: &str) -> &TreeNode {
        let x = TreeNode::new(NodeType::FOLDER, dest.to_string(), self);
        self.children.push(x);

        return &self.children[self.children.len() - 1];
    }

    // pub fn cd_back(&mut self) -> &TreeNode {}
}

fn main() {
    let args = common::read_args();
    let commands = common::read_file(&args[1]).unwrap();

    let commands_with_result: Vec<&str> = commands.split("$").collect();

    let mut root = TreeNode::new(NodeType::FOLDER, "/".to_string());
    let mut current_leaf: &TreeNode = &root;

    let mut commands: Vec<Command> = Vec::new();

    for c in commands_with_result {
        let x = c.split("\n").collect::<Vec<&str>>();
        if x[0] != "" {
            commands.push(Command {
                command: x[0].to_string(),
                result: x[1..].to_vec(),
            })
        }
    }

    for command in commands {
        // if command.command.contains("cd") {
        //     let dest = &command.command.split(" ").collect::<Vec<&str>>()[2];

        //     if *dest == ".." {
        //         //
        //     } else if *dest == "/" {
        //         //
        //     } else {
        //         current_leaf = root.cd(dest);
        //     }

        //     dbg!(&dest);
        //     dbg!(current_leaf);
        // }
    }
}
