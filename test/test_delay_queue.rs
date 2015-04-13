use syncbox::{DelayQueue, Queue, SyncQueue};
use time::Duration;

#[test]
fn test_ordering() {
    let queue = DelayQueue::new();

    queue.offer_delay(1i32, -Duration::days(1));
    queue.offer_delay(2i32, -Duration::days(3));
    queue.offer_delay(3i32, -Duration::days(2));

    assert_eq!(2, queue.take());
    assert_eq!(3, queue.take());
    assert_eq!(1, queue.take());
}

#[test]
fn test_poll() {
    let queue = DelayQueue::new();

    queue.offer_delay(1i32, Duration::nanoseconds(0));
    queue.offer_delay(2i32, Duration::days(1));

    assert_eq!(Some(1), queue.poll());
    assert_eq!(None, queue.poll());
}

#[test]
fn test_poll_timeout() {
    let queue = DelayQueue::new();

    queue.offer_delay(1i32, Duration::nanoseconds(0));
    queue.offer_delay(2i32, Duration::milliseconds(250));
    queue.offer_delay(3i32, Duration::days(1));

    assert_eq!(Some(1), queue.poll_timeout(Duration::milliseconds(250)));
    assert_eq!(Some(2), queue.poll_timeout(Duration::milliseconds(300)));
    assert_eq!(None, queue.poll_timeout(Duration::milliseconds(500)));
}
