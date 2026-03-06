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

