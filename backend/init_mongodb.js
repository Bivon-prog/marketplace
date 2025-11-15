// MongoDB Initialization Script for MarketHub
// Run this with: mongosh marketplace_db < init_mongodb.js
// Or: mongosh
//     use marketplace_db
//     load('init_mongodb.js')

// Switch to marketplace_db database
db = db.getSiblingDB('marketplace_db');

// Create collections with validation
db.createCollection("users", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["name", "email", "password_hash", "user_type", "created_at"],
      properties: {
        name: { bsonType: "string" },
        email: { bsonType: "string" },
        password_hash: { bsonType: "string" },
        user_type: { bsonType: "string", enum: ["customer", "provider", "seller"] },
        created_at: { bsonType: "date" }
      }
    }
  }
});

db.createCollection("services", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["provider_id", "title", "description", "category", "price", "location", "created_at"],
      properties: {
        provider_id: { bsonType: "string" },
        title: { bsonType: "string" },
        description: { bsonType: "string" },
        category: { bsonType: "string" },
        price: { bsonType: "double" },
        location: { bsonType: "string" },
        icon: { bsonType: "string" },
        rating: { bsonType: "double" },
        created_at: { bsonType: "date" }
      }
    }
  }
});

db.createCollection("products", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["seller_id", "title", "description", "category", "price", "file_type", "file_url", "created_at"],
      properties: {
        seller_id: { bsonType: "string" },
        title: { bsonType: "string" },
        description: { bsonType: "string" },
        category: { bsonType: "string" },
        price: { bsonType: "double" },
        file_type: { bsonType: "string" },
        file_url: { bsonType: "string" },
        icon: { bsonType: "string" },
        rating: { bsonType: "double" },
        downloads: { bsonType: "int" },
        created_at: { bsonType: "date" }
      }
    }
  }
});

db.createCollection("bookings");
db.createCollection("purchases");
db.createCollection("reviews");

// Create indexes
db.users.createIndex({ "email": 1 }, { unique: true });
db.services.createIndex({ "category": 1 });
db.services.createIndex({ "location": 1 });
db.services.createIndex({ "provider_id": 1 });
db.products.createIndex({ "category": 1 });
db.products.createIndex({ "seller_id": 1 });
db.bookings.createIndex({ "customer_id": 1 });
db.bookings.createIndex({ "service_id": 1 });
db.purchases.createIndex({ "customer_id": 1 });
db.purchases.createIndex({ "product_id": 1 });
db.reviews.createIndex({ "item_id": 1, "item_type": 1 });
db.reviews.createIndex({ "user_id": 1 });

print("✅ Database and collections created successfully!");
print("✅ Indexes created successfully!");

// Insert sample data
// Note: You'll need to run the backend first to hash passwords properly
// These are just placeholders

print("\n📝 To add sample data:");
print("1. Start the backend server");
print("2. Use the signup API to create test users");
print("3. Use the create service/product APIs to add listings");

print("\n🎉 MongoDB initialization complete!");
