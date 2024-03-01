//! Tree of keystrokes with commands as leaf nodes

use indextree::{Arena, Node, NodeId};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// A sequence of key strokes
#[derive(Debug, Clone)]
pub struct KeySequence( Vec<u16> );

impl KeySequence {
    /// create a new key sequence from &Vec[u16]
    pub fn new(seq: &Vec<u16>) -> Self {
        Self ( seq.clone() )
    }
}

impl PartialEq for KeySequence {
    /// check if two KeySequences are equal
    fn eq(&self, other: &Self) -> bool {
        (self.0.len() == other.0.len()) &&
            (self.0.iter().zip(&other.0).all(|(a, b)| *a == *b))
    }
}
impl Eq for KeySequence {}

impl From<&Vec<u16>> for KeySequence {
    /// convert &Vec<u16> into a KeySequence
    fn from(seq: &Vec<u16>) -> Self {
        Self (seq.clone())
    }
}

/// A tree that holds the key sequences and associated Strings
/// (e.g. commands). The tree is implemented as indextree, which works
/// well because no members are ever deleted.
///
pub struct KTree {
    arena: Arena<(u16, Option<String>)>,
    root: NodeId,
}

impl KTree {
    /// Create a new empty KTree
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node((0, None));
        Self { arena, root }
    }

    /// Dump the tree to stderr for debugging purposes
    pub fn dump(&self) {
        eprintln!("{:?}\n", self.root.debug_pretty_print(&self.arena));
    }

    /// Look up a KeySequence by walking the tree and return the
    /// associated command if found, or None
    pub fn find(&self, seq: &KeySequence) -> &Option<String> {
        let mut node_id = self.root;
        'key_loop:
        for key in &seq.0 {
            for ch in node_id.children(&self.arena) {
                let node = self.arena.get(ch).unwrap().get();
                if node.0 == *key {
                    node_id = ch;
                    continue 'key_loop;
                }
            }
            return &None;
        }
        &self.arena.get(node_id).unwrap().get().1
    }

    /// Add a KeySequence and associated String to the tree
    pub fn add(&mut self, seq: &KeySequence, command: Option<String>) {
        let mut node_id = self.root;
        'key_loop:
        for key in &seq.0 {
            for ch in node_id.children(&self.arena) {
                let node = self.arena.get(ch).unwrap().get();
                if node.0 == *key {
                    node_id = ch;
                    continue 'key_loop;
                }
            }
            node_id = node_id.append_value((*key, None), &mut self.arena);
        }

        let (ref mut _key, ref mut cmd) = self.arena.get_mut(node_id).unwrap().get_mut();
        *cmd = command;
    }
}

