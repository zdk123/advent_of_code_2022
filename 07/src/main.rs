use std::fs::File;
use std::io::{BufRead, BufReader};
use slab_tree::*;


#[derive(Debug)]
struct IFile {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct IDir {
    name: String,
    files: Vec<IFile>,
    size: u32,
}

impl IDir {
    fn add_file(&mut self, f: IFile) {
        self.files.push(f);
    }
    fn add_size(&mut self, s: u32) {
        self.size += s;
    }
}

fn main() {

    // Open the file and read
    let file = File::open("input7.txt").unwrap();
    let reader = BufReader::new(file);
    
    // Set up a tree-based file system  to walk around in
    let mut file_system = TreeBuilder::new().with_root(IDir { name: String::from("/"), files: Vec::new(), size: 0 }).build();

    let root_id = file_system.root_id().unwrap();
    let mut current_id = root_id; // initiate as a reference to the root node

    // Read each line of the file
    let mut num_dir = 1; // keep track of file system size.
    for line in reader.lines() {
        let line = line.unwrap();
        match line {
            line if line.starts_with("$ cd") => {
                // changing to some directory
                let dir_name = line.split(" ").nth(2).unwrap();
                // change to root
                if dir_name == "/" {
                    // root already instantiated, skip

                // change to parent
                } else if dir_name == ".." {
                    // Get current dir
                   let mut current_dir = file_system.get_mut(current_id).unwrap();
                    // set id to parent
                   current_id = current_dir.parent().unwrap().node_id();
                // change to a child
                } else {
                    num_dir += 1;
                    let mut current_dir = file_system.get_mut(current_id).unwrap();
                    let new_dir = current_dir.append(IDir { name: String::from(dir_name), files: Vec::new(), size: 0 });
                    let current_dir = new_dir;
                    current_id = current_dir.node_id();
                }
            }
            line if line.starts_with("$ ls") => {
                // skip
            }
            line if line.starts_with("dir") => {
                // skip
            }
            line => {
                // found a file!!
                let file_info: Vec<&str> = line.split(" ").collect();
                let file_name = file_info[1].to_string();
                let file_size = file_info[0].parse().unwrap();
                let new_file = IFile { name: file_name, size: file_size };
                // add file to current directory
                let mut dir_node = file_system.get_mut(current_id).unwrap();
                let dir = dir_node.data();
                dir.add_file(new_file);
            }
        }

    }
    // uncomment to print a nice tree
    // let mut s = String::new();
    // file_system.write_formatted(&mut s).unwrap();
    // println!("{}", s)

    // Post-order traverse to calculate directory sizes
    let mut dir_sizes: Vec<u32> = Vec::new();

    for i in 0..num_dir {
        let node_id = file_system.root().unwrap().traverse_post_order().skip(i).next()
                         .unwrap().node_id();
        let mut dir_node = file_system.get_mut(node_id).unwrap();

        // Dir size is the sum of all the child directories and files sizes
        let mut child_sizes = 0;
        for child in dir_node.as_ref().children() {
            child_sizes += child.data().size
        }
        let mut file_sizes = 0;
        for file in dir_node.data().files.iter() {
            file_sizes += file.size
        }
        
        let node_data = dir_node.data();
        node_data.add_size(child_sizes + file_sizes);
        
        dir_sizes.push(child_sizes + file_sizes);
    }

    // For the final answer, total the size of directories 
    let mut total_size_under = 0;
    for i in 0..num_dir {
        let dsize = dir_sizes[i];
        if dsize <= 100000 {
            total_size_under += dsize;
        }
    }
    println!("Total size under 100000: {:?}", total_size_under);


    // PART II
    let root_dir = file_system.get(root_id).unwrap();
    let max_space = 70000000 - 30000000; // value the current fs cannot exceed 
    let target_for_deletion = root_dir.data().size-max_space;
    //println!("Target: {:?}", target_for_deletion);
    let mut candidates:  Vec<u32> = Vec::new();
    for i in 0..num_dir {
        let dsize = dir_sizes[i];
        if dsize > target_for_deletion {
            candidates.push(dsize);
        }
    }
    candidates.sort();
    println!("Size of deleted directory: {:?}", candidates[0]);

}