use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug)]
pub struct Entity {
    pub name:String,
    pub entity_type:EntityType
}


#[derive(Debug)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Product,
    Technology,
    Event,
    Money,
    Number,
}


#[async_trait]
pub trait EntityExtractor {
    async fn extract(&self,markdown:&str) -> Result<Vec<Entity>>;
}