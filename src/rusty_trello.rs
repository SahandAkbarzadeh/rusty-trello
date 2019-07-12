pub mod rusty_trello {
    use rand::Rng;
    use std::collections::HashMap;

    use std::hash::{Hash, Hasher};
    use std::error::Error;

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
        fn get_id(&self) -> &Id;
    }

    // trello entities

    pub enum Entities<'a> {
        Board(BoardEntity<'a>),
        List(ListEntity<'a>),
        Member(MemberEntity<'a>),
        Card(CardEntity<'a>),
    }

    pub struct BoardEntity<'a> {
        id: &'a Id,
    }

    impl<'a> Entity for BoardEntity<'a> {
        fn get_id(&self) -> &Id {
            self.id
        }
    }

    pub struct ListEntity<'a> {
        id: &'a Id,
    }

    impl<'a> Entity for ListEntity<'a> {
        fn get_id(&self) -> &Id {
            self.id
        }
    }

    pub struct MemberEntity<'a> {
        id: &'a Id,
    }

    impl<'a> MemberEntity<'a> {
        pub fn new(id: &Id) -> MemberEntity {
            MemberEntity { id }
        }
    }

    impl<'a> Entity for MemberEntity<'a> {
        fn get_id(&self) -> &Id {
            self.id
        }
    }

    pub struct CardEntity<'a> {
        id: &'a Id,
    }

    impl<'a> Entity for CardEntity<'a> {
        fn get_id(&self) -> &Id {
            self.id
        }
    }

    pub enum Authorization {
        AppKeyAndToken(String, String),
    }

    pub struct Client {
        authorization: Authorization,
        entities: HashMap<Id, Box<Entity>>,
    }

    impl Client {
        pub fn new(auth: Authorization) -> Client {
            Client {
                authorization: auth,
                entities: HashMap::new(),
            }
        }

        pub fn create_by_environment_variables() -> Result<Client, Box<dyn Error>> {
            use std::env;
            let app_key = env::var("TRELLO_APP_KEY")?;
            let token = env::var("TRELLO_TOKEN")?;
            Ok(Client::new(Authorization::AppKeyAndToken(app_key, token)))
        }

        pub fn get_entity_by_id(&self, id: &Id) -> Result<&Box<Entity>, &'static str> {
            match self.entities.get(id) {
                Some(value) => Ok(value),
                None => Err("Entity with given id not found"),
            }
        }

        pub fn get_or_create_card_by_id(&mut self, id: Id) -> Result<&Entity, &'static str> {
            match self.get_entity_by_id(&id) {
                Ok(value) => Ok(&**value),
                _ => match self.entities.insert(id, Box::new(MemberEntity::new(&id))) {
                    Some(value) => Ok(&*value),
                    None => Err("could not insert new entity to the client's container")
                }
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

        match env::var("TRELLO_TOKEN") {
            Ok(s) => assert!(!s.is_empty()),
            _ => assert!(false)
        }
    }
}

#[cfg(test)]
mod test_client_auth {
    use super::rusty_trello::*;

    #[test]
    fn test_authorization() {
        let client = Client::create_by_environment_variables().unwrap();
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
            Id::EntityId(_) => assert!(false),
            Id::DeletedEntityId(_) => assert!(false),
            Id::PlaceHolderId(value) => assert!(!value.is_empty()),
        }
    }
}