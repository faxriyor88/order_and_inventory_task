use crate::errors::{AppError, OrderError, ProductError, ValidationError};
use crate::models::order::{CreateOrder, Order};
use crate::models::order_item::{CreateOrderItem, InsertOrderItem, OrderItem};
use crate::repositories::order_item_repo::OrderItemRepository;
use crate::repositories::order_repo::OrderRepository;
use crate::repositories::product_repo::ProductRepository;
use rust_decimal::Decimal;
use sqlx::PgPool;

pub struct OrderService {
    pool: PgPool,
    order_repo: OrderRepository,
    order_item_repo: OrderItemRepository,
}

impl OrderService {
    pub fn new(
        pool: PgPool,
        order_repo: OrderRepository,
        order_item_repo: OrderItemRepository,
    ) -> Self {
        Self {
            pool,
            order_repo,
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

        // open transaction
        let mut tx = self.pool.begin().await?;

        let _order = OrderRepository::find_by_id_with(&mut *tx, order_id)
            .await?
            .ok_or(OrderError::NotFound)?;

        let product = ProductRepository::decrease_stock_returning_with(
            &mut *tx,
            input_item.product_id,
            input_item.quantity,
        )
        .await?
        .ok_or(ProductError::NotFound)?;

        let unit_price = product.price.ok_or(OrderError::ProductPriceMissing)?;
        let line_total = Decimal::from(input_item.quantity) * unit_price;

        let order_item = OrderItemRepository::create_with(
            &mut *tx,
            InsertOrderItem {
                order_id,
                product_id: input_item.product_id,
                quantity: input_item.quantity,
                unit_price,
                line_total,
            },
        )
        .await?;

        let recalculated =
            OrderRepository::recalculate_total_price_with(&mut *tx, order_id).await?;

        if !recalculated {
            return Err(OrderError::NotFound.into());
        }

        // close transaction
        tx.commit().await?;
        Ok(order_item)
    }

    pub async fn list(&self) -> Result<Vec<Order>, AppError> {
        let list = self.order_repo.list().await?;

        Ok(list)
    }

    pub async fn list_items(&self, order_id: i64) -> Result<Vec<OrderItem>, AppError> {
        let list_items = self.order_item_repo.list_by_order_id(order_id).await?;

        Ok(list_items)
    }
}
