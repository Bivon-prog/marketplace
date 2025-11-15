use mongodb::{Database, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;

pub async fn init_db(db: &Database) -> Result<(), mongodb::error::Error> {
    // Create indexes for users collection
    let users = db.collection::<crate::models::User>("users");
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    users.create_index(email_index, None).await?;

    // Create indexes for services collection
    let services = db.collection::<crate::models::Service>("services");
    let service_indexes = vec![
        IndexModel::builder().keys(doc! { "category": 1 }).build(),
        IndexModel::builder().keys(doc! { "location": 1 }).build(),
        IndexModel::builder().keys(doc! { "provider_id": 1 }).build(),
    ];
    services.create_indexes(service_indexes, None).await?;

    // Create indexes for products collection
    let products = db.collection::<crate::models::Product>("products");
    let product_indexes = vec![
        IndexModel::builder().keys(doc! { "category": 1 }).build(),
        IndexModel::builder().keys(doc! { "seller_id": 1 }).build(),
    ];
    products.create_indexes(product_indexes, None).await?;

    // Create indexes for bookings collection
    let bookings = db.collection::<crate::models::Booking>("bookings");
    let booking_indexes = vec![
        IndexModel::builder().keys(doc! { "customer_id": 1 }).build(),
        IndexModel::builder().keys(doc! { "service_id": 1 }).build(),
    ];
    bookings.create_indexes(booking_indexes, None).await?;

    // Create indexes for purchases collection
    let purchases = db.collection::<crate::models::Purchase>("purchases");
    let purchase_indexes = vec![
        IndexModel::builder().keys(doc! { "customer_id": 1 }).build(),
        IndexModel::builder().keys(doc! { "product_id": 1 }).build(),
    ];
    purchases.create_indexes(purchase_indexes, None).await?;

    // Create indexes for reviews collection
    let reviews = db.collection::<crate::models::Review>("reviews");
    let review_indexes = vec![
        IndexModel::builder().keys(doc! { "item_id": 1, "item_type": 1 }).build(),
        IndexModel::builder().keys(doc! { "user_id": 1 }).build(),
    ];
    reviews.create_indexes(review_indexes, None).await?;

    println!("✅ Database indexes created successfully");

    Ok(())
}
