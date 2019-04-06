use super::Namespace;

pub struct NamespaceList {
    list: Vec<Namespace>
}

impl NamespaceList {
    pub fn empty() -> NamespaceList {
        NamespaceList {
            list: vec![]
        }
    }

    pub fn insert(&mut self, other: Namespace) {
        self.list.push(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::libcontainer::linux::namespace::Namespace;
    use crate::libcontainer::linux::namespace::NamespaceType;

    #[test]
    fn namespace_list_empty_returns_an_empty_list() {
        let namespaces = NamespaceList::empty();
        assert!(namespaces.list.is_empty(), "expect {:?} to be empty", &namespaces.list);
    }

    #[test]
    fn namespace_list_insert_adds_a_new_namespace() {
        let namespace = Namespace::new(NamespaceType::PID, None);

        let mut namespaces = NamespaceList::empty();
        namespaces.insert(namespace);

        assert_eq!(namespaces.list.len(), 1, "expect {:?} to be one element", &namespaces.list);
    }
}
