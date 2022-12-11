use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
    children: Vec<File>,
}

impl From<&str> for File {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            size: 0,
            children: Vec::new(),
        }
    }
}

impl File {
    /// Find a child `File` by its path, relative to the current parent.
    fn try_find_child(&mut self, path: &str) -> Option<&mut File> {
        self.children.iter_mut().find(|f| f.name == path)
    }

    /// Find a child `File` by its paths, relative to the current parent.
    /// Replaces the current parent with the final result.
    /// Panics if unsuccessfull.
    fn find_child(&mut self, paths: &[String]) -> &mut Self {
        let mut current = self;

        for path in paths {
            current = current.try_find_child(path).unwrap();
        }

        current
    }

    /// Checks whether a child `File` exists at the specified path.
    fn has_child(&mut self, path: &str) -> bool {
        self.children.iter().any(|x| x.name == path)
    }

    /// Recursively calculate the size of the current file and its children.
    /// (We could also do this at runtime but since the file tree never changes it makes more sense to calculate it only once)
    fn calculate_sizes(&mut self) -> usize {
        for file in &mut self.children {
            self.size += file.calculate_sizes();
        }

        self.size
    }

    /// Returns all child files recursively.
    fn get_files(&self) -> HashSet<Self> {
        let mut files = HashSet::new();

        for child in &self.children {
            files.insert(child.clone());
            files.extend(child.get_files());
        }

        files
    }

    /// Whether it's a directory.
    fn is_dir(&self) -> bool {
        self.children.len() > 0
    }
}

fn parse_file_tree(raw: &str) -> File {
    let mut tree = File::from("/");
    let mut path = Vec::new();

    for line in raw.lines() {
        let replaced = line.replace("$", "");
        let parts = replaced.trim().split_whitespace().collect::<Vec<_>>();

        let output = parts[0];

        if output == "cd" {
            let dir_name = parts[1];

            match dir_name {
                "/" => continue,
                ".." => {
                    path.pop().unwrap();
                    continue;
                }
                _ => {}
            }

            path.push(dir_name.to_owned());

            let parent = tree.find_child(&path);

            if parent.has_child(dir_name) {
                continue;
            }

            parent.children.push(File::from(dir_name));

            continue;
        }

        if output == "dir" {
            let dir_name = parts[1];
            let parent = tree.find_child(&path);

            if parent.has_child(dir_name) {
                continue;
            }

            parent.children.push(File::from(dir_name));

            continue;
        }

        if let Ok(size) = output.parse::<usize>() {
            let file_name = parts[1];

            let mut child = File::from(file_name);
            child.size = size;

            tree.find_child(&path).children.push(child);
        }
    }

    tree.calculate_sizes();

    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = parse_file_tree(input)
        .get_files()
        .iter()
        .filter(|file| file.is_dir() && file.size <= 100000)
        .fold(0, |x, y| x + y.size);

    Some(size as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = parse_file_tree(input);

    let space = 30_000_000 - (70_000_000 - tree.size);

    let children = tree.get_files();
    let mut children_mut = children.iter().collect::<Vec<_>>();
    children_mut.sort_by(|a, b| a.size.cmp(&b.size));

    let size = children_mut
        .iter()
        .find(|x| x.is_dir() && x.size > space)
        .unwrap()
        .size;

    Some(size as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
