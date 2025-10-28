use state_watcher::state_channel;

#[test]
fn single_thread() {
    let (writer, reader) = state_channel::<i32>();
    assert_eq!(reader.latest(), None);
    writer.update(22);
    assert_eq!(reader.latest(), Some(22));
    assert_eq!(reader.latest_and_clear(), Some(22));
    assert_eq!(reader.latest(), None);
}
