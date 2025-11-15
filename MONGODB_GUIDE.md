# MongoDB Setup Guide for MarketHub

## Installation

### Windows

**Option 1: Using Installer**
1. Download MongoDB Community Server from: https://www.mongodb.com/try/download/community
2. Run the installer
3. Choose "Complete" installation
4. Install MongoDB as a Windows Service (recommended)
5. Install MongoDB Compass (GUI tool) - optional but helpful

**Option 2: Using winget**
```bash
winget install MongoDB.Server
```

### Verify Installation

```bash
# Check MongoDB version
mongosh --version

# Check if MongoDB is running
mongosh --eval "db.version()"
```

## Starting MongoDB

### Windows Service (Automatic)
MongoDB should start automatically after installation as a Windows service.

**Check service status:**
```bash
# Check if running
sc query MongoDB

# Start service
net start MongoDB

# Stop service
net stop MongoDB

# Restart service
net stop MongoDB && net start MongoDB
```

### Manual Start (if not using service)
```bash
# Start MongoDB server
mongod --dbpath C:\data\db
```

## Database Initialization

### Option 1: Automatic (Recommended)
The Rust backend automatically creates collections and indexes when it starts.

Just run:
```bash
cd backend
cargo run
```

### Option 2: Manual Initialization
Run the initialization script:

```bash
mongosh < backend/init_mongodb.js
```

This creates:
- Database: `marketplace_db`
- Collections: users, services, products, bookings, purchases, reviews
- Indexes for better query performance
- Validation rules for data integrity

## Connecting to MongoDB

### Using mongosh (MongoDB Shell)

```bash
# Connect to local MongoDB
mongosh

# Connect to specific database
mongosh mongodb://localhost:27017/marketplace_db

# With authentication (if configured)
mongosh mongodb://username:password@localhost:27017/marketplace_db
```

### Using MongoDB Compass (GUI)

1. Open MongoDB Compass
2. Connection string: `mongodb://localhost:27017`
3. Click "Connect"
4. Navigate to `marketplace_db` database

## Database Structure

### Collections

**users**
- Stores user accounts (customers, providers, sellers)
- Unique index on email
- Fields: name, email, password_hash, user_type, created_at

**services**
- Local service listings
- Indexes on: category, location, provider_id
- Fields: provider_id, title, description, category, price, location, icon, rating, created_at

**products**
- Digital product listings
- Indexes on: category, seller_id
- Fields: seller_id, title, description, category, price, file_type, file_url, icon, rating, downloads, created_at

**bookings**
- Service bookings
- Indexes on: customer_id, service_id
- Fields: customer_id, service_id, booking_date, booking_time, notes, status, created_at

**purchases**
- Product purchases
- Indexes on: customer_id, product_id
- Fields: customer_id, product_id, payment_method, amount, status, download_url, created_at

**reviews**
- Reviews and ratings
- Indexes on: item_id + item_type, user_id
- Fields: user_id, item_id, item_type, rating, comment, created_at

## Common MongoDB Operations

### View Data

```javascript
// Switch to database
use marketplace_db

// View all collections
show collections

// Count documents
db.users.countDocuments()
db.services.countDocuments()
db.products.countDocuments()

// Find all users
db.users.find().pretty()

// Find specific user
db.users.findOne({ email: "test@example.com" })

// Find services by category
db.services.find({ category: "home" })

// Find products with price filter
db.products.find({ price: { $lte: 20 } })
```

### Insert Data

```javascript
// Insert a user (Note: password should be hashed by backend)
db.users.insertOne({
  name: "Test User",
  email: "test@example.com",
  password_hash: "$2b$12$...",
  user_type: "customer",
  created_at: new Date()
})

// Insert a service
db.services.insertOne({
  provider_id: "user_id_here",
  title: "Test Service",
  description: "Test description",
  category: "home",
  price: 50.00,
  location: "Nairobi",
  icon: "🔧",
  created_at: new Date()
})
```

### Update Data

```javascript
// Update user
db.users.updateOne(
  { email: "test@example.com" },
  { $set: { name: "Updated Name" } }
)

// Update service rating
db.services.updateOne(
  { _id: ObjectId("...") },
  { $set: { rating: 4.5 } }
)

// Increment product downloads
db.products.updateOne(
  { _id: ObjectId("...") },
  { $inc: { downloads: 1 } }
)
```

### Delete Data

```javascript
// Delete a user
db.users.deleteOne({ email: "test@example.com" })

// Delete all bookings with status 'cancelled'
db.bookings.deleteMany({ status: "cancelled" })
```

### Aggregation Queries

```javascript
// Total revenue
db.purchases.aggregate([
  { $group: { _id: null, total: { $sum: "$amount" } } }
])

// Count users by type
db.users.aggregate([
  { $group: { _id: "$user_type", count: { $sum: 1 } } }
])

// Average rating per category
db.services.aggregate([
  { $group: { _id: "$category", avgRating: { $avg: "$rating" } } }
])

// Top 10 most downloaded products
db.products.aggregate([
  { $sort: { downloads: -1 } },
  { $limit: 10 },
  { $project: { title: 1, downloads: 1, price: 1 } }
])
```

## Backup and Restore

### Backup Database

```bash
# Backup entire database
mongodump --db marketplace_db --out C:\backups\mongodb

# Backup specific collection
mongodump --db marketplace_db --collection users --out C:\backups\mongodb
```

### Restore Database

```bash
# Restore entire database
mongorestore --db marketplace_db C:\backups\mongodb\marketplace_db

# Restore specific collection
mongorestore --db marketplace_db --collection users C:\backups\mongodb\marketplace_db\users.bson
```

## Performance Optimization

### Indexes

The backend automatically creates these indexes:

```javascript
// Users
db.users.createIndex({ email: 1 }, { unique: true })

// Services
db.services.createIndex({ category: 1 })
db.services.createIndex({ location: 1 })
db.services.createIndex({ provider_id: 1 })

// Products
db.products.createIndex({ category: 1 })
db.products.createIndex({ seller_id: 1 })

// Bookings
db.bookings.createIndex({ customer_id: 1 })
db.bookings.createIndex({ service_id: 1 })

// Purchases
db.purchases.createIndex({ customer_id: 1 })
db.purchases.createIndex({ product_id: 1 })

// Reviews
db.reviews.createIndex({ item_id: 1, item_type: 1 })
db.reviews.createIndex({ user_id: 1 })
```

### Check Index Usage

```javascript
// View all indexes
db.services.getIndexes()

// Explain query plan
db.services.find({ category: "home" }).explain("executionStats")
```

## Security

### Enable Authentication (Production)

1. Create admin user:
```javascript
use admin
db.createUser({
  user: "admin",
  pwd: "secure_password",
  roles: [ { role: "userAdminAnyDatabase", db: "admin" } ]
})
```

2. Create database user:
```javascript
use marketplace_db
db.createUser({
  user: "marketplace_user",
  pwd: "secure_password",
  roles: [ { role: "readWrite", db: "marketplace_db" } ]
})
```

3. Update backend/.env:
```env
MONGODB_URI=mongodb://marketplace_user:secure_password@localhost:27017
```

4. Restart MongoDB with authentication:
```bash
mongod --auth --dbpath C:\data\db
```

## Monitoring

### Check Database Stats

```javascript
// Database statistics
db.stats()

// Collection statistics
db.users.stats()
db.services.stats()

// Current operations
db.currentOp()

// Server status
db.serverStatus()
```

### Monitor Performance

```javascript
// Slow queries
db.setProfilingLevel(1, { slowms: 100 })
db.system.profile.find().pretty()

// Connection info
db.serverStatus().connections
```

## Troubleshooting

### MongoDB won't start

```bash
# Check if port 27017 is in use
netstat -ano | findstr :27017

# Check MongoDB logs
# Location: C:\Program Files\MongoDB\Server\6.0\log\mongod.log

# Try starting manually
mongod --dbpath C:\data\db
```

### Connection refused

```bash
# Verify MongoDB is running
sc query MongoDB

# Check firewall settings
# Allow port 27017 in Windows Firewall

# Test connection
mongosh mongodb://localhost:27017
```

### Database not found

```bash
# MongoDB creates databases automatically
# Just connect and insert data
use marketplace_db
db.users.insertOne({ test: "data" })
```

## Useful Commands

```bash
# Show all databases
show dbs

# Switch database
use marketplace_db

# Show collections
show collections

# Drop database (careful!)
db.dropDatabase()

# Drop collection
db.users.drop()

# Clear collection
db.users.deleteMany({})

# Export collection to JSON
mongoexport --db marketplace_db --collection users --out users.json

# Import collection from JSON
mongoimport --db marketplace_db --collection users --file users.json
```

## MongoDB Compass (GUI)

MongoDB Compass provides a visual interface for:
- Browsing collections
- Running queries
- Creating indexes
- Viewing performance
- Importing/exporting data

**Connection String:**
```
mongodb://localhost:27017
```

## Resources

- Official Documentation: https://docs.mongodb.com/
- MongoDB University: https://university.mongodb.com/
- Community Forums: https://www.mongodb.com/community/forums/

---

**Quick Reference:**
- Start: `net start MongoDB`
- Stop: `net stop MongoDB`
- Connect: `mongosh`
- View data: `db.collection.find().pretty()`
- Backup: `mongodump --db marketplace_db`
