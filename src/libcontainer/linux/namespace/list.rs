use super::Namespace;
use super::NamespaceType;

#[derive(Debug)]
pub struct NamespaceList {
    list: Vec<Namespace>
}

impl NamespaceList {
    pub fn empty() -> NamespaceList {
        NamespaceList {
            list: vec![]
        }
    }

    pub fn as_vec(&self) -> &Vec<Namespace> {
        &self.list
    }

    pub fn insert(&mut self, namespace: Namespace) {
        self.list.push(namespace);
    }

    pub fn contains_type(&self, ns_type: NamespaceType) -> bool {
        self.list.iter().any(|ns|
             ns.r#type == ns_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::libcontainer::linux::namespace::Namespace;
    use crate::libcontainer::linux::namespace::NamespaceType;

    #[test]
    fn namespace_list_as_vec_returns_a_vector() {
        let namespaces = NamespaceList::empty();
        assert_eq!(namespaces.as_vec().len(), 0);
    }

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

    #[test]
    fn namespace_list_contains_returns_false_if_ns_type_is_not_present() {
        let namespaces = NamespaceList::empty();
        assert_eq!(namespaces.contains_type(NamespaceType::UTS), false);
    }

    #[test]
    fn namespace_list_contains_returns_true_if_ns_type_is_present() {
        let mut namespaces = NamespaceList::empty();
        namespaces.insert(Namespace::new(NamespaceType::UTS, None));
        assert_eq!(namespaces.contains_type(NamespaceType::UTS), true);
    }
}
