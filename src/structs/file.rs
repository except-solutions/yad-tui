#[derive(Debug)]
pub struct File {
    pub name: String,
    pub file_type: NodeType,
}

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum NodeType {
    File,
    Dir,
}
