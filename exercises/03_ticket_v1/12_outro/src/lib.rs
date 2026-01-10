// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u64,
    unit_price: u64,
}

impl Order {

    pub fn new(product_name: String, quantity:u64,  unit_price:u64) -> Self {
        assert!(!product_name.is_empty());
        assert!(product_name.len() <= 300);
        assert!(quantity > 0);
        assert!(unit_price > 0);

        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn total(&self) -> u64 {
        return self.quantity * self.unit_price
    }

    // getters

    pub fn product_name(&self) -> &str {
        return &self.product_name
    }

    pub fn quantity(&self) -> &u64 {
        return &self.quantity
    }

    pub fn unit_price(&self) -> &u64 {
        return &self.unit_price
    }

    //setters

    pub fn set_product_name(&mut self,new_name: String) {
        assert!(!new_name.is_empty());
        assert!(new_name.len() <= 300);
        self.product_name = new_name
    }

    pub fn set_quantity(&mut self, new_quantity: u64) {
        assert!(new_quantity > 0);
        self.quantity = new_quantity
    }

    pub fn set_unit_price(&mut self, new_price: u64) {
        assert!(new_price > 0);
        self.unit_price = new_price
    }


}

// TODO: Define a new `Quote` type.
//   It should keep track of three pieces of information: `symbol`, `bid`, and `ask`.
//   The `symbol` must be a non-empty string and no longer than 8 bytes.
//   Both `bid` and `ask` are prices in cents.
//   `bid` must be strictly greater than zero.
//   `ask` must be strictly greater than zero.
//   Additionally, `bid` must always be strictly less than `ask`.
//     (In real markets, "crossed" markets are invalid.)
//
//   `Quote` must include a method named `midpoint` that returns:
//       (bid + ask) / 2   (integer division in cents)
//   Do not use floats.
//
//   `Quote` must provide setters and getters for each field.
//   All setters must re-enforce the invariants.
//
// Notes:
//   Tests are located in the `tests` folder.
//   Integration tests will only have access to the public API of your project.
//   Pay attention to visibility; integration tests can't access private or `pub(crate)` items.
//
// Hints:
//   — This models a Level 1 quote feed used by many trading firms.
//   — Using cents avoids floating-point and reduces rounding latency.
//   — Midpoint is used for fair price calculation and risk checks.

pub struct Quote {
    symbol: String, 
    bid: u64, 
    ask: u64
}

impl Quote {

    pub fn new(symbol: String, bid: u64, ask: u64) -> Self {
        assert!(!symbol.is_empty());
        assert!(symbol.len() <= 8);
        assert!(bid > 0);
        assert!(ask > 0);
        assert!(bid < ask);

        Self {
            symbol,
            bid,
            ask
        }
    }

    pub fn midpoint(&self) -> u64 {
        return (self.bid + self.ask) / 2
    }

    //getters

    pub fn symbol(&self) -> &str {
        return &self.symbol
    }

    pub fn bid(&self) -> u64 {
        return self.bid
    }

    pub fn ask(&self) -> u64 {
        return self.ask
    }

    //setters

    pub fn set_symbol(&mut self, new_symbol: String) {
        assert!(!new_symbol.is_empty());
        assert!(new_symbol.len() <= 8);
        self.symbol = new_symbol;
    }

    pub fn set_bid(&mut self, new_bid: u64) {
        assert!(new_bid > 0);
        assert!(new_bid < self.ask);
        self.bid = new_bid;
    }

    pub fn set_ask(&mut self, new_ask: u64) {
        assert!(new_ask > 0);
        assert!(self.bid < new_ask);
        self.ask = new_ask;
    }

}