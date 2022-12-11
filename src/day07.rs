use std::{cell::RefCell, fmt::Display, rc::Rc};

use regex::Regex;

use ChangeDirectory::*;
use Inode::*;

#[derive(Debug, Clone)]
enum Inode<'a> {
   File {
      parent: Option<Rc<RefCell<Inode<'a>>>>,
      path: Option<String>,
      name: String,
      size: usize,
   },
   Directory {
      parent: Option<Rc<RefCell<Inode<'a>>>>,
      children: Vec<Rc<RefCell<Inode<'a>>>>,
      path: Option<String>,
      name: String,
      // Invariant: Because the directory size is a computed property, this
      // property should be updated in file creation and deletion operations
      // occurring within this directory in order to prevent drift.
      size: usize,
   },
}

impl<'a> Inode<'a> {
   fn set_parent(&mut self, new_parent: Option<Rc<RefCell<Inode<'a>>>>) {
      match self {
         File { ref mut parent, .. } => *parent = new_parent,
         Directory { ref mut parent, .. } => *parent = new_parent,
      }
   }

   fn get_parent(&self) -> Option<Rc<RefCell<Inode<'a>>>> {
      match self {
         File { parent, .. } => parent.clone(),
         Directory { parent, .. } => parent.clone(),
      }
   }

   fn get_children(&self) -> Vec<Rc<RefCell<Inode<'a>>>> {
      match self {
         File { .. } => vec![],
         Directory { children, .. } => children.clone(),
      }
   }

   fn set_path(&mut self, new_path: Option<String>) {
      match self {
         File { ref mut path, .. } => *path = new_path,
         Directory { ref mut path, .. } => *path = new_path,
      }
   }

   fn get_name(&self) -> String {
      match self {
         File { name, .. } => name.clone(),
         Directory { name, .. } => name.clone(),
      }
   }

   fn set_size(&mut self, new_size: usize) {
      match *self {
         File { ref mut size, .. } => *size = new_size,
         Directory { ref mut size, .. } => *size = new_size,
      }
   }

   fn get_size(&self) -> usize {
      match *self {
         File { size, .. } => size,
         Directory { size, .. } => size,
      }
   }

   fn is_file(&self) -> bool {
      match self {
         File { .. } => true,
         Directory { .. } => false,
      }
   }

   fn is_directory(&self) -> bool {
      !self.is_file()
   }

   fn iter(&self) -> InodeIter<'a> {
      InodeIter {
         current: Rc::new(RefCell::new(self.clone())),
         parent: None,
         children: self.get_children().clone(),
         seen: false,
      }
   }
}

impl<'a> Display for Inode<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         File { name, size, .. } => writeln!(f, "file: {} ({})", name, size)?,

         Directory {
            path,
            size,
            children,
            ..
         } => {
            writeln!(
               f,
               "dir: {} ({})",
               path.clone().unwrap_or("?".to_owned()),
               size
            )?;

            for c in children {
               let c = c.borrow();
               write!(f, "{}", *c)?;
            }
         }
      }

      Ok(())
   }
}

#[derive(Clone)]
struct InodeIter<'a> {
   current: Rc<RefCell<Inode<'a>>>,
   parent: Option<Box<InodeIter<'a>>>,
   children: Vec<Rc<RefCell<Inode<'a>>>>,
   seen: bool,
}

impl<'a> Iterator for InodeIter<'a> {
   type Item = Rc<RefCell<Inode<'a>>>;

   fn next(&mut self) -> Option<Self::Item> {
      // We may visit the same directory multiple times during traversal.
      // Ensure that they are returned from the iterator only once.
      if !self.seen {
         self.seen = true;
         return Some(self.current.clone());
      }

      match self.children.clone().get(0) {
         None => match self.parent.take() {
            Some(parent) => {
               *self = *parent;
               self.next()
            }
            None => None,
         },
         Some(inode) => {
            let item = self.children.remove(0);
            match *inode.clone().borrow() {
               File { .. } => Some(item),
               Directory { .. } => {
                  let current = self.clone();
                  *self = item.borrow().iter();
                  self.parent = Some(Box::new(current));
                  self.next()
               }
            }
         }
      }
   }
}

#[derive(Debug, Clone)]
struct FileSystem<'a> {
   tree: Rc<RefCell<Inode<'a>>>,
}

impl<'a> FileSystem<'a> {
   fn new() -> Self {
      FileSystem {
         tree: Rc::new(RefCell::new(Directory {
            parent: None,
            path: Some(String::from("/")),
            name: String::from("/"),
            size: 0,
            children: vec![],
         })),
      }
   }
}

struct FileSystemCursor<'a> {
   cwd: String,
   file_system: Rc<RefCell<Inode<'a>>>,
   cursor: Rc<RefCell<Inode<'a>>>,
}

impl<'a> FileSystemCursor<'a> {
   fn new(file_system: FileSystem<'a>) -> Self {
      FileSystemCursor {
         cwd: file_system.tree.clone().borrow().get_name(),
         file_system: file_system.tree.clone(),
         cursor: file_system.tree.clone(),
      }
   }

   fn change_directory(&mut self, command: ChangeDirectory) {
      match command {
         Root => {
            self.cwd = "/".to_owned();
            self.cursor = self.file_system.clone();
         }

         In(file_name) => {
            let cursor = self.cursor.clone();
            let current_directory = cursor.borrow();

            if let Directory { ref children, .. } = *current_directory {
               let subdirectory = children.iter().find(|file| {
                  let file = file.borrow();
                  file.is_directory() && file.get_name() == file_name
               });

               if let Some(subdirectory) = subdirectory {
                  self.cursor = subdirectory.clone();
                  self.cwd = self.cwd.clone() + &subdirectory.borrow().get_name() + "/";
               }
            }
         }

         Out => {
            let cursor = self.cursor.clone();
            let current_directory = cursor.borrow();

            if let Directory { ref parent, .. } = *current_directory {
               match parent {
                  Some(parent) => {
                     self.cursor = parent.clone();

                     let parts: Vec<_> = self.cwd.split_terminator('/').collect();
                     let parts_count = parts.len();

                     if parts_count == 0 {
                        self.cwd = String::from("/");
                     } else {
                        self.cwd = parts
                           .into_iter()
                           .take(parts_count - 1)
                           .collect::<Vec<_>>()
                           .join("/")
                           + "/";
                     }
                  }

                  // Cannot `cd ..` in root directory
                  None => (),
               }
            }
         }
      }
   }

   fn create_file(&mut self, file: Rc<RefCell<Inode<'a>>>) {
      if let Directory {
         ref mut parent,
         ref mut size,
         ref mut children,
         ..
      } = *self.cursor.borrow_mut()
      {
         // Update directory size for parent directory.
         *size += file.borrow().get_size();
         let mut iter_cursor = parent.clone();
         while let Some(inode) = iter_cursor {
            let mut inode = inode.borrow_mut();
            let new_size = inode.get_size() + file.borrow().get_size();
            inode.set_size(new_size);
            iter_cursor = inode.get_parent().clone();
         }

         // Link the file to the filesystem tree.
         let absolute_path = self.cwd.clone() + &file.borrow().get_name();
         file.borrow_mut().set_path(Some(absolute_path));
         file.borrow_mut().set_parent(Some(self.cursor.clone()));
         children.push(file);

         return;
      }

      // Invariant: Filesystem cursors can only point to directories.
      unreachable!()
   }
}

#[derive(Debug)]
enum ChangeDirectory {
   Root,
   In(&'static str),
   Out,
}

#[derive(Debug)]
enum Command {
   ListDirectory,
   ChangeDirectory(ChangeDirectory),
}

fn parse_command(input: &'static str) -> Option<Command> {
   if input == "$ ls" {
      return Some(Command::ListDirectory);
   }

   if input == "$ cd /" {
      return Some(Command::ChangeDirectory(Root));
   }

   if input == "$ cd .." {
      return Some(Command::ChangeDirectory(Out));
   }

   let cd_in = Regex::new(r"\$ cd (.*)").unwrap();
   if let Some(captures) = cd_in.captures(input) {
      return captures
         .get(1)
         .map(|filename| Command::ChangeDirectory(In(filename.as_str())));
   }

   None
}

fn parse_inode(input: &'static str) -> Option<Inode> {
   if input.starts_with("$") {
      return None;
   }

   let directory = Regex::new(r"dir (.*)").unwrap();
   if let Some(captures) = directory.captures(input) {
      return captures.get(1).map(|directory_name| Directory {
         parent: None,
         path: None,
         name: directory_name.as_str().to_owned(),
         size: 0,
         children: vec![],
      });
   }

   let file = Regex::new(r"(.*) (.*)").unwrap();
   if let Some(captures) = file.captures(input) {
      return captures.get(1).and_then(|size| {
         captures.get(2).and_then(|file_name| {
            Some(File {
               parent: None,
               path: None,
               name: file_name.as_str().to_owned(),
               size: size.as_str().parse().unwrap(),
            })
         })
      });
   }

   None
}

fn build_file_system() -> FileSystem<'static> {
   let console_output = include_str!("../input/day07.txt")
      .lines()
      .collect::<Vec<&'static str>>();

   let file_system = FileSystem::new();
   let mut cursor = FileSystemCursor::new(file_system.clone());

   for line in console_output {
      if let Some(command) = parse_command(line) {
         match command {
            Command::ListDirectory => (),
            Command::ChangeDirectory(command) => cursor.change_directory(command),
         }
      }

      if let Some(inode) = parse_inode(line) {
         cursor.create_file(Rc::new(RefCell::new(inode)));
      }
   }

   file_system.clone()
}

pub fn solve_part_one() -> usize {
   build_file_system()
      .tree
      .borrow()
      .iter()
      .filter(|file| file.borrow().is_directory())
      .map(|file| file.borrow().get_size())
      .filter(|&size| size <= 100000)
      .sum()
}

pub fn solve_part_two() -> usize {
   const TOTAL_DISK_SPACE: usize = 70000000;
   const TARGET_FREE_DISK_SPACE: usize = 30000000;

   let file_system = build_file_system();
   let current_disk_space = file_system.tree.borrow().get_size();
   let disk_space_to_free = current_disk_space - (TOTAL_DISK_SPACE - TARGET_FREE_DISK_SPACE);

   let mut directory_sizes = build_file_system()
      .tree
      .borrow()
      .iter()
      .filter(|file| file.borrow().is_directory())
      .map(|file| file.borrow().get_size())
      .collect::<Vec<_>>();

   directory_sizes.sort();
   for &directory_size in directory_sizes.iter() {
      if directory_size >= disk_space_to_free {
         return directory_size;
      }
   }

   unreachable!()
}
