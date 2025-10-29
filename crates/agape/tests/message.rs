use agape::message::MessageQueue;

#[test]
fn add_and_remove_message() {
    let mut messages = MessageQueue::new();
    messages.add(String::from("Message"));
    assert_eq!(messages.len(), 1);
    let message = messages.remove::<String>().unwrap();
    assert!(messages.is_empty());
    assert_eq!(&message, "Message");
}
