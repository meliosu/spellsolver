use std::fmt::Write;

const ALPHABET_SIZE: usize = 26;

pub struct TrieNode {
    terminal: bool,
    children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
}

impl std::fmt::Debug for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt(node: &TrieNode, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
            for (c, child) in node.children() {
                for _ in 0..depth {
                    f.write_char(' ')?;
                }

                f.write_char(c)?;

                if child.is_complete() {
                    f.write_char('*')?;
                }

                f.write_char('\n')?;
                fmt(child, f, depth + 1)?;
            }

            Ok(())
        }

        fmt(self, f, 0)
    }
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            terminal: false,
            children: [const { None }; ALPHABET_SIZE],
        }
    }

    pub fn is_complete(&self) -> bool {
        self.terminal
    }

    pub fn is_leaf(&self) -> bool {
        self.children.iter().all(|o| o.is_none())
    }

    pub fn child(&self, c: char) -> Option<&Self> {
        ('a'..='z')
            .contains(&c)
            .then(|| self.children[c as usize - 'a' as usize].as_deref())
            .flatten()
    }

    pub fn children(&self) -> impl Iterator<Item = (char, &Self)> {
        self.children.iter().enumerate().filter_map(|(i, child)| {
            if let Some(child) = child.as_deref() {
                Some((char::from_u32(i as u32 + 'a' as u32).unwrap(), child))
            } else {
                None
            }
        })
    }

    pub fn build<I, C>(items: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: AsRef<str>,
    {
        let mut root = TrieNode::new();

        for item in items {
            let mut curr = &mut root;

            for c in item.as_ref().chars() {
                if curr.children[c as usize - 'a' as usize].is_none() {
                    curr.children[c as usize - 'a' as usize] = Some(Box::new(TrieNode::new()));
                }

                curr = curr.children[c as usize - 'a' as usize]
                    .as_deref_mut()
                    .unwrap();
            }

            curr.terminal = true;
        }

        root
    }
}
