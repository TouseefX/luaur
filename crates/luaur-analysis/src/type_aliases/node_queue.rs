use crate::records::node::Node;
use luaur_common::records::vec_deque::VecDeque;

pub type NodeQueue = VecDeque<*mut Node>;
