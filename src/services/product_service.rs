use crate::errors::{AppError, ValidationError};
use crate::models::order::Order;
use crate::models::product::{CreateProduct, Product};
use crate::repositories::product_repo::ProductRepository;

pub struct ProductService {
    product_repo: ProductRepository,
}

impl ProductService {
    pub fn new(product_repo: ProductRepository) -> Self {
        Self { product_repo }
    }

    pub async fn create(&self, input: CreateProduct) -> Result<Product, AppError> {
        if input.name.trim().is_empty() {
            return Err(ValidationError::EmptyField("name").into());
        }

        let product = self.product_repo.create(input).await?;

        Ok(product)
    }
}
