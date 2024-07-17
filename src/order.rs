use rand::distributions::{Alphanumeric, DistString};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::{self, Duration, SystemTime};

#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct OrderIndex {
    id: i64,
    price: i64,
    timestamp: time::SystemTime,
    order_side: OrderSide,
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.price < other.price {
            match self.order_side {
                OrderSide::Bid => Ordering::Less,
                OrderSide::Ask => Ordering::Greater,
            }
        } else if self.price > other.price {
            match self.order_side {
                OrderSide::Bid => Ordering::Greater,
                OrderSide::Ask => Ordering::Less,
            }
        } else {
            other.timestamp.cmp(&self.timestamp)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderIndex {
    fn eq(&self, other: &Self) -> bool {
        if self.price > other.price || self.price < other.price {
            false
        } else {
            self.timestamp == other.timestamp
        }
    }
}

impl Eq for OrderIndex {}

#[derive(Debug)]
pub struct Order {
    pub id: i64,
    pub sticker_id: String,
    pub creator_user_id: String,
    pub fulfiller_user_id: Option<String>,
    pub is_fulfilled: bool,
    pub price: i64,
    pub order_side: OrderSide,
    pub created_at: time::SystemTime,
}
// Main order book for all stickers
pub struct OrderBook {
    sticker_order_map: HashMap<String, StickerOrderBook>,
}
impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            sticker_order_map: HashMap::new(),
        }
    }
}
// Order book for each individual sticker
pub struct StickerOrderBook {
    order_map: HashMap<i64, Order>,
    order_queue_ask: BinaryHeap<OrderIndex>,
    order_queue_bid: BinaryHeap<OrderIndex>,
}
impl StickerOrderBook {
    fn new() -> Self {
        StickerOrderBook {
            order_map: HashMap::new(),
            order_queue_ask: BinaryHeap::new(),
            order_queue_bid: BinaryHeap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        let new_order_index = OrderIndex {
            id: order.id,
            order_side: order.order_side,
            price: order.price,
            timestamp: order.created_at,
        };
        match order.order_side {
            OrderSide::Bid => self.order_queue_bid.push(new_order_index),
            OrderSide::Ask => self.order_queue_ask.push(new_order_index),
        }
        self.order_map.insert(order.id, order);
    }

    fn remove_order(&mut self, order_id: i64) {
        self.order_queue_bid.retain(|index| index.id != order_id);
        self.order_queue_ask.retain(|index| index.id != order_id);
        self.order_map.remove_entry(&order_id);
    }

    fn next_bid_order(&mut self) -> Option<OrderIndex> {
        return self.order_queue_bid.pop();
    }
    fn next_ask_order(&mut self) -> Option<OrderIndex> {
        return self.order_queue_ask.pop();
    }
    fn match_order(&mut self) {
        // pop ask first, pop until you get a different price
        // pop bid, see if number works
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn _create_test_order(
        id: i64,
        price: i64,
        created_at: SystemTime,
        order_type: OrderSide,
    ) -> Order {
        Order {
            id: id,
            sticker_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            creator_user_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            fulfiller_user_id: None,
            is_fulfilled: false,
            price: price,
            order_side: order_type,
            created_at: created_at,
        }
    }
    #[test]
    fn order_book_test_basic_add_sell_order_price1() {
        let id1 = 1;
        let id2 = 2;
        assert_ne!(id1, id2);
        let order1: Order = _create_test_order(id1, 25, SystemTime::now(), OrderSide::Ask);
        let order2: Order = _create_test_order(id2, 15, SystemTime::now(), OrderSide::Ask);
        let mut order_book = StickerOrderBook::new();
        order_book.add_order(order1);
        order_book.add_order(order2);
        let res1 = order_book.next_ask_order();
        let res2 = order_book.next_ask_order();
        let res3 = order_book.next_ask_order();
        assert!(res1.is_some());
        assert!(res2.is_some());
        assert_eq!(res3, None);
        if let Some(first_option) = res1 {
            assert_eq!(first_option.id, id2);
        } else {
            panic!("first_option should not be null");
        }
        if let Some(second_option) = res2 {
            assert_eq!(second_option.id, id1);
        } else {
            panic!("second_option should not be null");
        }
    }
    #[test]
    fn order_book_test_basic_add_sell_order_time1() {
        let id1 = 1;
        let id2 = 2;
        let now = SystemTime::now();
        let ten_minutes = Duration::from_secs(600);
        let future_time = now + ten_minutes;
        let order1: Order = _create_test_order(id1, 15, SystemTime::now(), OrderSide::Ask);
        let order2: Order = _create_test_order(id2, 15, future_time, OrderSide::Ask);

        let mut order_book = StickerOrderBook::new();
        order_book.add_order(order1);
        order_book.add_order(order2);
        let res1 = order_book.next_ask_order();
        let res2 = order_book.next_ask_order();
        let res3 = order_book.next_ask_order();
        assert!(res1.is_some());
        assert!(res2.is_some());
        assert_eq!(res3, None);
        if let Some(first_option) = res1 {
            assert_eq!(first_option.id, id1);
        } else {
            panic!("first_option should not be null");
        }
        if let Some(second_option) = res2 {
            assert_eq!(second_option.id, id2);
        } else {
            panic!("second_option should not be null");
        }
    }
    #[test]
    fn order_book_test_basic_add_buy_order_price1() {
        let id1 = 1;
        let id2 = 2;
        assert_ne!(id1, id2);
        let order1: Order = _create_test_order(id1, 25, SystemTime::now(), OrderSide::Bid);
        let order2: Order = _create_test_order(id2, 15, SystemTime::now(), OrderSide::Bid);

        let mut order_book = StickerOrderBook::new();
        order_book.add_order(order1);
        order_book.add_order(order2);
        let res1 = order_book.next_bid_order();
        let res2 = order_book.next_bid_order();
        let res3 = order_book.next_bid_order();
        assert!(res1.is_some());
        assert!(res2.is_some());
        assert_eq!(res3, None);
        if let Some(first_option) = res1 {
            assert_eq!(first_option.id, id1);
        } else {
            panic!("first_option should not be null");
        }
        if let Some(second_option) = res2 {
            assert_eq!(second_option.id, id2);
        } else {
            panic!("second_option should not be null");
        }
    }
    #[test]
    fn order_book_test_basic_add_buy_order_time1() {
        let id1 = 1;
        let id2 = 2;
        let now = SystemTime::now();
        let ten_minutes = Duration::from_secs(600);
        let future_time = now + ten_minutes;
        let order1: Order = _create_test_order(id1, 15, SystemTime::now(), OrderSide::Bid);
        let order2: Order = _create_test_order(id2, 15, future_time, OrderSide::Bid);
        let mut order_book = StickerOrderBook::new();
        order_book.add_order(order1);
        order_book.add_order(order2);
        let res1 = order_book.next_bid_order();
        let res2 = order_book.next_bid_order();
        let res3 = order_book.next_bid_order();
        assert!(res1.is_some());
        assert!(res2.is_some());
        assert_eq!(res3, None);
        if let Some(first_option) = res1 {
            assert_eq!(first_option.id, id1);
        } else {
            panic!("first_option should not be null");
        }
        if let Some(second_option) = res2 {
            assert_eq!(second_option.id, id2);
        } else {
            panic!("second_option should not be null");
        }
    }
}
