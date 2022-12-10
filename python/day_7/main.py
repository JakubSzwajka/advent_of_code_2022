from enum import Enum
from typing import List
from itertools import chain

class NodeType(Enum):
    FILE: str = "FILE"
    DIR: str = "DIR"


class Node:
    def __init__(self, name, parent=None, node_type=NodeType.DIR, size=0) -> None:
        # self.childs: List[Node] = []
        self.childs = {}
        self.parent: Node = parent
        self.size = size
        self.name = name

        self.type = node_type

    def print(self, level=0, dir_only=False) -> str:

        prefix = "-" * level
        
        if dir_only and self.type == NodeType.DIR: 
            print(
                f"{prefix} {self.name} ({self.type.name.lower()} ,size={str(self.get_size())})"
            )
        elif not dir_only: 
            print(
                f"{prefix} {self.name} ({self.type.name.lower()} ,size={str(self.get_size())})"
            )
            
        for name, child in self.childs.items():
            child.print(level + 1, dir_only=dir_only)
            

    def get_size(self):
        if self.type == NodeType.FILE:
            return self.size
        elif self.type == NodeType.DIR:
            total_size = 0
            for name, child in self.childs.items():
                total_size += child.get_size()
            return total_size
        
    def get_sizes_over(self, threshold): 
        big_files = []
        for name, child in self.childs.items():
            if child.type == NodeType.DIR:
                if child.get_size() < threshold:
                    big_files.append(child)
                if child_big_files := child.get_sizes_over(threshold):
                    big_files += child_big_files

        return big_files
    
    def __repr__(self) -> str:
        return f"{self.type} {self.name} ({self.size})"
    
    def __str__(self) -> str:
        return f"{self.type} {self.name}"


class DirManager:
    def __init__(self) -> None:
        self.root = Node("/")
        self.current_node = self.root

    def cd(self, command: str):
        if "/" in command:
            self.current_node = self.root

        elif ".." in command:
            self.current_node = self.current_node.parent

        else:
            dir_name = command.strip().split(" ")[2]
            dir = Node(dir_name, self.current_node)
            self.current_node.childs[dir_name] = dir
            self.current_node = dir

    def ls(self, node: str):
        size, name = node.strip().split(" ")

        if size.isnumeric():
            self.current_node.childs[name] = Node(
                name=name, size=int(size), node_type=NodeType.FILE, parent=self.current_node
            )
        elif size == "dir":
            self.current_node.childs[name] = Node(
                name=name, node_type=NodeType.DIR, parent=self.current_node
            )

def main():
    manager = DirManager()

    with open("./input.txt", "r") as input:
        commands = input.readlines()

    for command in commands[1:]:
        if command.startswith("$"):
            if "cd" in command:
                manager.cd(command)
        else:
            manager.ls(command)

    # manager.root.print(dir_only=True)

    total_size = 0
    for file in manager.root.get_sizes_over(100000): 
        total_size += file.get_size()
        
    print(total_size)
        
if __name__ == "__main__":
    main()
