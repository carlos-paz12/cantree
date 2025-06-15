use super::node::Node;
use crate::utils::pair::OrderedPair;

#[derive(Debug)]
pub struct BinarySearchTree
{
  pub root: Option<Box<Node>>,
}

impl BinarySearchTree
{
  pub fn new() -> Self
  {
    BinarySearchTree { root: None }
  }

  pub fn insert(&mut self, pair: OrderedPair) -> bool
  {
    let new_node = Node::new(pair);
    Self::insert_node(&mut self.root, new_node)
  }

  pub fn search(&self, key: u64) -> Option<&OrderedPair>
  {
    Self::search_node(&self.root, key)
  }

  pub fn remove(&mut self, key: u64) -> Option<Box<Node>>
  {
    Self::remove_node(&mut self.root, key)
  }

  fn insert_node(current: &mut Option<Box<Node>>, new_node: Node) -> bool
  {
    match current
    {
      None =>
      {
        *current = Some(Box::new(new_node));
        return true;
      }
      Some(existing_node) =>
      {
        if new_node.key < existing_node.key
        {
          Self::insert_node(&mut existing_node.left, new_node)
        }
        else if new_node.key > existing_node.key
        {
          Self::insert_node(&mut existing_node.right, new_node)
        }
        else
        {
          return false;
        }
      }
    }
  }

  fn search_node(current: &Option<Box<Node>>, key: u64)
  -> Option<&OrderedPair>
  {
    match current
    {
      Some(existing_node) =>
      {
        if key == existing_node.key
        {
          Some(&existing_node.pair)
        }
        else if key < existing_node.key
        {
          Self::search_node(&existing_node.left, key)
        }
        else
        {
          Self::search_node(&existing_node.right, key)
        }
      }
      None => None,
    }
  }

  fn remove_node(root: &mut Option<Box<Node>>, key: u64) -> Option<Box<Node>>
  {
    match root
    {
      None =>
      {
        return None;
      }
      Some(node) =>
      {
        if key < node.key
        {
          return Self::remove_node(&mut node.left, key);
        }
        else if key > node.key
        {
          return Self::remove_node(&mut node.right, key);
        }
        else
        {
          match (node.left.take(), node.right.take())
          {
            (None, None) =>
            {
              return root.take();
            }
            (Some(child), None) | (None, Some(child)) =>
            {
              let node_to_remove = root.take().unwrap();
              *root = Some(child);
              return Some(node_to_remove);
            }
            (Some(left), Some(right)) =>
            {
              let successor_key;
              let successor_pair;
              {
                let mut successor = right.as_ref();
                while let Some(ref left_child) = successor.left
                {
                  successor = left_child.as_ref();
                }
                successor_key = successor.key;
                successor_pair = successor.pair.clone();
              }

              node.key = successor_key;
              node.pair = successor_pair;
              node.left = Some(left);
              node.right = Some(right);

              Self::remove_node(&mut node.right, successor_key);

              return Some(node.clone());
            }
          }
        }
      }
    }
  }

  pub fn to_string(&self) -> String
  {
    fn helper(
      node: &Option<Box<Node>>,
      prefix: String,
      is_left: bool,
      is_root: bool,
    ) -> String
    {
      let mut output = String::new();

      output += &prefix;
      if is_root
      {
        output += ".\n";
      }

      output += if is_root
      {
        "└──"
      }
      else if is_left
      {
        "├E─"
      }
      else
      {
        "└D─"
      };

      match node
      {
        Some(n) =>
        {
          // Com cores (ANSI escape) para terminal, se quiser tirar cores,
          // remova \x1b[..m
          output += &format!(
            "\x1b[32m◉ [Key={}, Pair=({}, {})]\x1b[0m\n",
            n.key, n.pair.first, n.pair.second
          );

          let new_prefix = prefix
            + if is_root
            {
              "   "
            }
            else if is_left
            {
              "│  "
            }
            else
            {
              "   "
            };

          output += &helper(&n.left, new_prefix.clone(), true, false);
          output += &helper(&n.right, new_prefix.clone(), false, false);
        }
        None =>
        {
          output += "\x1b[31mx\x1b[0m\n";
        }
      }

      output
    }

    helper(&self.root, "".to_string(), false, true)
  }
}
