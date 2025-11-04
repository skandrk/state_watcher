#[cfg(feature = "mutex")]
use statewatcher::state_readerwriter;

#[test]
#[cfg(feature = "mutex")]
fn single_thread() {
    let rw1 = state_readerwriter::<i32>();
    let rw2 = rw1.clone();
    assert_eq!(rw1.latest(), None);
    rw2.update(22);
    assert_eq!(rw2.latest(), Some(22));
    assert_eq!(rw1.latest(), Some(22));
    assert_eq!(rw1.latest_and_clear(), Some(22));
    assert_eq!(rw2.latest(), None);
    assert_eq!(rw2.latest(), rw1.latest());
}
