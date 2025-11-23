use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    #[serde(
        serialize_with = "serialize_price",
        deserialize_with = "deserialize_price"
    )]
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub in_stock: bool,
    #[serde(skip_serializing, default)]
    pub internal_id: u64,
}

fn serialize_price<S>(price: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert f64 to kopecks (integer)
    let kopecks = (*price * 100.0) as i64;
    serializer.serialize_i64(kopecks)
}

fn deserialize_price<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let kopecks = i64::deserialize(deserializer)?;
    Ok(kopecks as f64 / 100.0)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub product: Product,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    #[serde(skip_serializing, default)]
    pub internal_id: Uuid,
    pub user_id: Uuid,
    #[serde(deserialize_with = "validate_email")]
    pub customer_email: String,
    pub items: Vec<OrderItem>,
    #[serde(
        serialize_with = "serialize_price",
        deserialize_with = "deserialize_price"
    )]
    pub total: f64,
    pub status: OrderStatus,
    #[serde(
        skip_serializing,
        deserialize_with = "chrono::serde::ts_seconds::deserialize",
        default = "Utc::now"
    )]
    pub created_at: DateTime<Utc>,
}

fn validate_email<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let email = String::deserialize(deserializer)?;
    // Email validation: must contain @, have at least one character before and after @
    // and contain at least one dot after the @
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2
        && !parts[0].is_empty()
        && !parts[1].is_empty()
        && parts[1].contains('.')
        && !email.starts_with('@')
        && !email.ends_with('@')
    {
        Ok(email)
    } else {
        Err(serde::de::Error::custom(format!(
            "invalid email format: {}",
            email
        )))
    }
}

fn main() {
    // Example usage
    let product_json = r#"
    {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "MacBook Pro",
        "price": 199999,
        "category": "Electronics",
        "in_stock": true
    }
    "#;

    let product: Product = serde_json::from_str(product_json).unwrap();
    println!("Deserialized product: {:?}", product);

    let serialized = serde_json::to_string_pretty(&product).unwrap();
    println!("Serialized product:\n{}", serialized);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_serializes_price_in_cents() {
        let product = Product {
            id: Uuid::nil(),
            name: "MacBook Pro".into(),
            price: 1999.99,
            category: Some("Electronics".into()),
            in_stock: true,
            internal_id: 42,
        };

        let json = serde_json::to_string(&product).unwrap();
        assert!(json.contains("\"price\":199999"));
        assert!(!json.contains("internal_id"));

        let decoded: Product = serde_json::from_str(&json).unwrap();
        assert!((decoded.price - 1999.99).abs() < f64::EPSILON);
        assert_eq!(decoded.internal_id, 0);
    }

    #[test]
    fn order_serialization_hides_internal_fields() {
        let product = Product {
            id: Uuid::new_v4(),
            name: "Keyboard".into(),
            price: 99.95,
            category: None,
            in_stock: true,
            internal_id: 1,
        };
        let order = Order {
            id: Uuid::new_v4(),
            internal_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            customer_email: "user@example.com".into(),
            items: vec![OrderItem {
                product: product.clone(),
                quantity: 2,
            }],
            total: 199.90,
            status: OrderStatus::Processing,
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&order).unwrap();
        assert!(!json.contains("internal_id"));
        assert!(json.contains("\"status\":\"processing\""));
        assert!(json.contains("\"price\":9995"));

        let decoded: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.customer_email, "user@example.com");
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.internal_id, Uuid::nil());
    }

    #[test]
    fn invalid_email_fails_to_deserialize() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "internal_id": Uuid::new_v4(),
            "user_id": Uuid::new_v4(),
            "customer_email": "invalid",
            "items": [],
            "total": 0,
            "status": "pending",
            "created_at": 1_600_000_000
        })
        .to_string();

        let err = serde_json::from_str::<Order>(&json).unwrap_err();
        assert!(err.to_string().contains("invalid email format"));
    }
}
