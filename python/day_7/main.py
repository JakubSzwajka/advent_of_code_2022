from enum import Enum
from typing import List
from itertools import chain
from copy import deepcopy

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
                if child.get_size() > threshold:
                    big_files.append(child)
                if child_big_files := child.get_sizes_over(threshold):
                    big_files += child_big_files

        return big_files
    
    def __repr__(self) -> str:
        return f"{self.type} {self.name} ({self.get_size()})"
    
    def __str__(self) -> str:
        return f"{self.type} {self.name} ({self.get_size()})"

    def __eq__(self, __o: object) -> bool:
        return self.get_size() == __o.get_size()

    def __lt__(self, __o: object) -> bool:
        return self.get_size() == __o.get_size()
    
    def __gt__(self, __o: object) -> bool:
        return self.get_size() == __o.get_size()
    
    def __le__(self, __o: object) -> bool:
        return self.get_size() == __o.get_size()
    
    def __ge__(self, __o: object) -> bool:
        return self.get_size() == __o.get_size()


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
    
    disc_space = 70000000
    update_size = 30000000

    with open("./input.txt", "r") as input:
        commands = input.readlines()

    for command in commands[1:]:
        if command.startswith("$") and "cd" in command:
            manager.cd(command)
        else:
            manager.ls(command)

    corrupted_space = manager.root.get_size()
    free_space = disc_space - corrupted_space
    space_to_obtain = update_size - free_space 
        
    print(f"Disc space: {disc_space}")
    print(f"Update size: {update_size}")
    print(f"Corrupted: {corrupted_space}")
    print(f"Free space: {free_space}")
    print(f"Space to obtain: {space_to_obtain}")    

    smallest_to_delete = space_to_obtain * 10
    
    for dir in manager.root.get_sizes_over(space_to_obtain):
        if dir.get_size( ) < smallest_to_delete:
            smallest_to_delete = dir.get_size()
        
    print('------------------------->', smallest_to_delete)
if __name__ == "__main__":
    main()
