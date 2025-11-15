# MarketHub - Quick Reference Card

## 🚀 Quick Start Commands

### Start Everything (Windows)
```bash
# Double-click or run:
start.bat
```

### Start Backend Only
```bash
cd backend
cargo run
```

### Start Frontend Only
```bash
cd frontend
python -m http.server 3000
```

## 🔗 URLs

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Admin Dashboard**: http://localhost:3000/pages/admin.html

## 🔑 Test Accounts

```
Provider: provider@test.com / password123
Seller:   seller@test.com / password123
Customer: customer@test.com / password123
```

## 📡 API Quick Reference

### Auth
```bash
# Signup
POST /api/auth/signup
Body: {"name":"...", "email":"...", "password":"...", "user_type":"customer"}

# Login
POST /api/auth/login
Body: {"email":"...", "password":"..."}
```

### Services
```bash
# Get all services
GET /api/services

# Filter services
GET /api/services?category=home&location=Nairobi

# Create service (requires auth)
POST /api/services
Headers: Authorization: Bearer {token}
Body: {"title":"...", "description":"...", "category":"...", "price":50, "location":"..."}
```

### Products
```bash
# Get all products
GET /api/products

# Filter products
GET /api/products?category=business&search=template

# Create product (requires auth)
POST /api/products
Headers: Authorization: Bearer {token}
Body: {"title":"...", "description":"...", "category":"...", "price":10, "file_type":"PDF", "file_url":"..."}
```

### Niche Markets
```bash
# Get niche products
GET /api/niche/resume
GET /api/niche/business
GET /api/niche/student
GET /api/niche/creator
GET /api/niche/developer
```

### Bookings
```bash
# Create booking (requires auth)
POST /api/bookings
Headers: Authorization: Bearer {token}
Body: {"service_id":1, "booking_date":"2025-01-20", "booking_time":"10:00", "notes":"..."}

# Get user bookings (requires auth)
GET /api/bookings
Headers: Authorization: Bearer {token}
```

### Purchases
```bash
# Purchase product (requires auth)
POST /api/purchases
Headers: Authorization: Bearer {token}
Body: {"product_id":1, "payment_method":"mpesa"}

# Get user purchases (requires auth)
GET /api/purchases
Headers: Authorization: Bearer {token}
```

### Reviews
```bash
# Create review (requires auth)
POST /api/reviews
Headers: Authorization: Bearer {token}
Body: {"item_id":1, "item_type":"service", "rating":5, "comment":"Great service!"}

# Get reviews
GET /api/reviews/service/1
GET /api/reviews/product/1
```

## 🗄️ Database Commands

### Connect to Database
```bash
mongosh mongodb://localhost:27017/marketplace_db
```

### Common Queries
```javascript
// View all users
db.users.find().pretty()

// View all services
db.services.find().pretty()

// View all products
db.products.find().pretty()

// View bookings
db.bookings.find().pretty()

// View purchases
db.purchases.find().pretty()

// Get revenue statistics
db.purchases.aggregate([
  { $group: { _id: null, total: { $sum: "$amount" } } }
])

// Get top rated services
db.services.find().sort({ rating: -1 }).limit(10)

// Get most downloaded products
db.products.find().sort({ downloads: -1 }).limit(10)

// Count users by type
db.users.aggregate([
  { $group: { _id: "$user_type", count: { $sum: 1 } } }
])

// Find services by category
db.services.find({ category: "home" })

// Search products
db.products.find({ 
  $or: [
    { title: { $regex: "template", $options: "i" } },
    { description: { $regex: "template", $options: "i" } }
  ]
})
```

## 🎨 Frontend File Structure

```
frontend/
├── index.html          → Landing page
├── pages/
│   ├── services.html   → Local services
│   ├── digital.html    → Digital products
│   ├── niche.html      → Niche markets
│   └── admin.html      → Admin dashboard
├── css/
│   └── styles.css      → All styles
└── js/
    ├── main.js         → Auth & utilities
    ├── services.js     → Service logic
    ├── digital.js      → Product logic
    └── niche.js        → Niche logic
```

## 🦀 Backend File Structure

```
backend/src/
├── main.rs             → Server setup
├── models.rs           → Data structures
├── auth.rs             → JWT handling
├── db.rs               → Database init
└── handlers/
    ├── auth.rs         → Login/signup
    ├── services.rs     → Service CRUD
    ├── products.rs     → Product CRUD
    ├── niche.rs        → Niche filtering
    ├── bookings.rs     → Booking CRUD
    ├── purchases.rs    → Purchase CRUD
    └── reviews.rs      → Review CRUD
```

## 🔧 Common Issues & Fixes

### Backend won't start
```bash
# Check MongoDB is running
mongosh --eval "db.version()"
# Fix: net start MongoDB

# Check port 8080 is free
netstat -ano | findstr :8080
# Fix: Kill process or change port in .env
```

### Database connection error
```bash
# Verify MongoDB is running
net start MongoDB

# Test connection
mongosh mongodb://localhost:27017

# Check backend/.env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=marketplace_db
```

### CORS errors
```bash
# Make sure backend is running
# Check API_URL in frontend/js/main.js
const API_URL = 'http://localhost:8080/api';
```

### Frontend not loading
```bash
# Use proper HTTP server, not file://
cd frontend
python -m http.server 3000
```

## 📝 Environment Variables

### backend/.env
```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=marketplace_db
JWT_SECRET=your-secret-key-change-this-in-production
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

## 🎯 User Types

- **customer** - Can book services and buy products
- **provider** - Can offer services
- **seller** - Can sell digital products

## 💰 Categories

### Service Categories
- `home` - Home Services
- `personal` - Personal Services
- `tech` - Tech Services
- `business` - Business Services

### Product Categories
- `business` - Business Tools
- `career` - Career & Personal Branding
- `creative` - Creative & Design
- `development` - Development
- `education` - Education
- `productivity` - Productivity

### Niche Types
- `resume` - Resume & Career
- `business` - Business Tools
- `student` - Student Resources
- `creator` - Creator Tools
- `developer` - Developer Resources

## 🔐 Authentication Flow

1. User signs up → Password hashed → Stored in DB
2. User logs in → Password verified → JWT token generated
3. Token stored in localStorage
4. Token sent in Authorization header for protected routes
5. Backend verifies token → Allows/denies access

## 📊 Database Schema Quick View

```
users (id, name, email, password_hash, user_type, created_at)
  ↓
services (id, provider_id, title, description, category, price, location, rating)
  ↓
bookings (id, customer_id, service_id, booking_date, booking_time, status)

users (id, name, email, password_hash, user_type, created_at)
  ↓
products (id, seller_id, title, description, category, price, file_url, downloads)
  ↓
purchases (id, customer_id, product_id, payment_method, amount, status)

reviews (id, user_id, item_id, item_type, rating, comment)
```

## 🚦 Status Codes

- `200` - Success
- `400` - Bad Request
- `401` - Unauthorized
- `404` - Not Found
- `500` - Server Error

## 📦 Dependencies

### Backend (Rust)
- actix-web - Web framework
- mongodb - Database driver
- bson - MongoDB data format
- bcrypt - Password hashing
- jsonwebtoken - JWT
- serde - Serialization

### Frontend
- No dependencies (Vanilla JS)

## 🎓 Learning Resources

- Rust: https://doc.rust-lang.org/book/
- Actix-web: https://actix.rs/
- PostgreSQL: https://www.postgresql.org/docs/
- JWT: https://jwt.io/

---

**Quick Tip**: Keep this file open while developing! 🚀
