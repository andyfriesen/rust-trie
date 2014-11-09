use std::str::CharRange;

struct El<T> {
    ch : char,
    children : Trie<T>
}

pub struct Trie<T> {
    elements : Vec<El<T>>,
    data : Option<T>
}

impl<T> Trie<T> {
    pub fn insert(self : &mut Trie<T>, key:&str, data: T) {
        if 0 == key.len() {
            self.data = Some(data);
            return;
        }

        let CharRange {ch, next} = key.char_range_at(0);

        for i in range(0, self.elements.len()) {
            let ech = self.elements[i].ch;
            if ech == ch {
                let e = &mut self.elements[i];
                e.children.insert(key.slice_from(next), data);
                return;
            } else if ech >= ch {
                self.elements.insert(i, El { ch : ch, children: new_trie(key.slice_from(next), data) });
                return;
            }
        }

        self.elements.push(El { ch : ch, children: new_trie(key.slice_from(next), data) });
    }

    pub fn find<'a>(self:&'a Trie<T>, key:&str) -> Option<&'a T> {
        if 0 == key.len() {
            return match self.data {
                Some(ref d) => Some(d),
                None => None
            }
        }

        let CharRange {ch, next} = key.char_range_at(0);

        // TODO binary search
        for el in self.elements.iter() {
            if el.ch == ch {
                return el.children.find(key.slice_from(next));
            } else if el.ch > ch {
                return None;
            }
        }

        return None;
    }
}

fn new_trie<T>(s:&str, data: T) -> Trie<T> {
    if 0 == s.len() {
        return Trie{ elements: Vec::new(), data: Some(data) };
    } else {
        let CharRange {ch, next} = s.char_range_at(0);
        let mut v = Vec::new();
        v.push(El { ch: ch, children: new_trie(s.slice_from(next), data)});
        return Trie{elements: v, data: None};
    }
}

pub fn new<T>() -> Trie<T> {
    return Trie{elements: Vec::new(), data: None};
}
