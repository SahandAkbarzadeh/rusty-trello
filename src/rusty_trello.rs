pub mod rusty_trello {
    use rand::Rng;
    use std::collections::HashMap;

    use std::hash::{Hash, Hasher};

    pub enum Id {
        EntityId(String),
        DeletedEntityId(String),
        PlaceHolderId(String),
    }

    impl std::hash::Hash for Id {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match &self {
                Id::EntityId(value) => value.hash(state),
                Id::DeletedEntityId(value) => value.hash(state),
                Id::PlaceHolderId(value) => value.hash(state),
            }
        }
    }

    impl PartialEq for Id {
        fn eq(&self, other: &Id) -> bool {
            match (&self, &other) {
                (Id::EntityId(left), Id::EntityId(right)) => left == right,
                (Id::DeletedEntityId(left), Id::DeletedEntityId(right)) => left == right,
                (Id::PlaceHolderId(left), Id::PlaceHolderId(right)) => left == right,
                _ => false,
            }
        }
    }

    impl std::cmp::Eq for Id {}

    impl Id {
        pub fn generate_placeholder_id() -> Id {
            let mut rng = rand::thread_rng();
            let letter: char = rng.gen_range(b'A', b'Z') as char;
            let number: u32 = rng.gen_range(0, 999999);
            let s = format!("{}{:06}", letter, number);
            Id::PlaceHolderId(s)
        }
    }

    trait Entity {
        fn get_id(&self) -> &Box<Id>;
    }

    // trello entities

    enum Entities {
        Board(BoardEntity),
        List(ListEntity),
        Member(MemberEntity),
        Card(CardEntity),
    }

    struct BoardEntity {
        pub id: Box<Id>,
    }

    impl Entity for BoardEntity {
        fn get_id(&self) -> &Box<Id> {
            &self.id
        }
    }

    struct ListEntity {
        pub id: Box<Id>,
    }

    impl Entity for ListEntity {
        fn get_id(&self) -> &Box<Id> {
            &self.id
        }
    }

    struct MemberEntity {
        pub id: Box<Id>,
    }

    impl Entity for MemberEntity {
        fn get_id(&self) -> &Box<Id> {
            &self.id
        }
    }

    struct CardEntity {
        id: Box<Id>,
    }

    impl Entity for CardEntity {
        fn get_id(&self) -> &Box<Id> {
            &self.id
        }
    }

    enum Authorization {
        AppKeySecret(String, String),
    }

    struct Client<'a> {
        authorization: &'a Authorization,
        entities: HashMap<Id, Box<Entity>>,
    }

    impl<'a> Client<'a> {
        fn new(auth: &Authorization) -> Client {
            Client {
                authorization: auth,
                entities: HashMap::new(),
            }
        }

        fn get_entity_by_id(&self, id: &Id) -> Result<&Box<Entity>, &'static str> {
            match self.entities.get(id) {
                Some(value) => Ok(value),
                None => Err("Entity with given id not found"),
            }
        }
    }
}

#[cfg(test)]
mod test_setup {
    use super::rusty_trello::*;
    use std::env;
    #[test]
    fn env_vars_set() {
        match env::var("TRELLO_APP_KEY") {
            Ok(s) => assert!(!s.is_empty()),
            _ => assert!(false)
        }

        match env::var("TRELLO_SECRET") {
            Ok(s) => assert!(!s.is_empty()),
            _ => assert!(false)
        }
    }
}

#[cfg(test)]
mod test_basic_validations {
    use super::rusty_trello::*;
    use std::env;

    #[test]
    fn test_id_eq() {
        assert!(Id::EntityId("test".to_string()) == Id::EntityId("test".to_string()));
        assert!(Id::EntityId("test".to_string()) != Id::EntityId("pp".to_string()));
        assert!(Id::EntityId("test".to_string()) != Id::PlaceHolderId("test".to_string()));
        assert!(Id::EntityId("test".to_string()) != Id::DeletedEntityId("test".to_string()));
    }

    #[test]
    fn test_generated_placeholder_id_is_valid() {
        match Id::generate_placeholder_id() {
            (Id::EntityId(_)) => assert!(false),
            (Id::DeletedEntityId(_)) => assert!(false),
            (Id::PlaceHolderId(value)) => assert!(!value.is_empty()),
        }
    }

}