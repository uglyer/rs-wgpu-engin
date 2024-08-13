use std::sync::atomic::{AtomicUsize, Ordering};

static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);

pub type UId = usize;

// 生成自增 id (同一应用内唯一)
pub fn generate_id() -> UId {
    CURRENT_ID.fetch_add(1, Ordering::SeqCst)
}

#[test]
fn test_generate_id() {
    let id1 = generate_id();
    let id2 = generate_id();
    assert_eq!(id1 + 1, id2);
}
