use crate::errors::{AppError, OrderError, ProductError, ValidationError};
use crate::models::order::{CreateOrder, Order};
use crate::models::order_item::{CreateOrderItem, InsertOrderItem, OrderItem};
use crate::repositories::order_item_repo::OrderItemRepository;
use crate::repositories::order_repo::OrderRepository;
use crate::repositories::product_repo::ProductRepository;
use rust_decimal::Decimal;

pub struct OrderService {
    order_repo: OrderRepository,
    product_repo: ProductRepository,
    order_item_repo: OrderItemRepository,
}

impl OrderService {
    pub fn new(
        order_repo: OrderRepository,
        product_repo: ProductRepository,
        order_item_repo: OrderItemRepository,
    ) -> Self {
        Self {
            order_repo,
            product_repo,
            order_item_repo,
        }
    }

    pub async fn create(&self, input: CreateOrder) -> Result<Order, AppError> {
        let order = self.order_repo.create(input).await?;

        Ok(order)
    }

    pub async fn add_item(
        &self,
        order_id: i64,
        input_item: CreateOrderItem,
    ) -> Result<OrderItem, AppError> {
        if input_item.quantity <= 0 {
            return Err(ValidationError::MustBePositive("quantity").into());
        }

        let _order = self
            .order_repo
            .find_by_id(order_id)
            .await?
            .ok_or(OrderError::NotFound)?;

        let product = self
            .product_repo
            .find_by_id(input_item.product_id)
            .await?
            .ok_or(ProductError::NotFound)?;

        let unit_price = product.price.ok_or(OrderError::ProductPriceMissing)?;
        let line_total = Decimal::from(input_item.quantity) * unit_price;

        let order_item = self
            .order_item_repo
            .create(InsertOrderItem {
                order_id,
                product_id: input_item.product_id,
                quantity: input_item.quantity,
                unit_price,
                line_total,
            })
            .await?;

        let stock_decreased = self
            .product_repo
            .decreasing_stock(input_item.product_id, input_item.quantity)
            .await?;

        if !stock_decreased {
            return Err(ProductError::NotFound.into());
        }

        let recalculated = self.order_repo.recalculate_total_price(order_id).await?;

        if !recalculated {
            return Err(OrderError::NotFound.into());
        }

        Ok(order_item)
    }
}
