use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use uuid::Uuid;

/// エンティティID
///
/// エンティティIDは、エンティティを一意に識別するためのIDを表現する。
/// エンティティIDは、UUIDを使用して生成される。
/// エンティティIDは、ジェネリック型`T`を持ち、エンティティの型を表現する。
/// これにより、異なるエンティティ（構造体）のIDが、同じUUIDを持っていても、型を区別する。
///
/// ```rust
/// use domain_primitives::entity_id::EntityId;
///
/// #[derive(Debug)]
/// struct Foo;
/// type FooId = EntityId<Foo>;
///
/// let id1 = FooId::new();
/// let id2 = FooId::from_uuid(id1.to_uuid());
/// assert_eq!(id1, id2);
/// ```
#[derive(Debug, Clone)]
pub struct EntityId<T>(Uuid, PhantomData<T>);

impl<T> EntityId<T> {
    /// コンストラクタ。
    pub fn new() -> Self {
        Self(Uuid::new_v4(), PhantomData)
    }

    /// UUIDからエンティティIDを生成する。
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid, PhantomData)
    }

    /// エンティティIDをUUIDに変換する。
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl<T> PartialEq for EntityId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for EntityId<T> {}

impl<T> Hash for EntityId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> std::fmt::Display for EntityId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 同じUUIDから作成してエンティティIDが等しいことを確認
    #[test]
    fn test_entity_id_equality() {
        let id1: EntityId<i32> = EntityId::new();
        let id2: EntityId<i32> = EntityId::from_uuid(id1.to_uuid());
        assert_eq!(id1, id2);
    }

    /// 異なるUUIDから作成してエンティティIDが等しくないことを確認
    #[test]
    fn test_entity_id_inequality() {
        let id1: EntityId<i32> = EntityId::new();
        let id2: EntityId<i32> = EntityId::new();
        assert_ne!(id1, id2);
    }

    /// エンティティIDがUUIDと同じハッシュ値を持つことを確認
    #[test]
    fn test_entity_id_hash() {
        let mut hasher = std::hash::DefaultHasher::new();
        let uuid = Uuid::new_v4();
        let id: EntityId<i32> = EntityId::from_uuid(uuid);
        assert_eq!(uuid.hash(&mut hasher), id.hash(&mut hasher));
    }

    /// エンティティIDがUUID文字列を表現することを確認
    #[test]
    fn test_entity_id_display() {
        let uuid = Uuid::new_v4();
        let id: EntityId<u32> = EntityId::from_uuid(uuid);
        assert_eq!(uuid.to_string(), id.to_string());
    }
}
