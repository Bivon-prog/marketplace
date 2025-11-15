# MarketHub - System Architecture

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         FRONTEND                             │
│                    (HTML/CSS/JavaScript)                     │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Home    │  │ Services │  │ Digital  │  │  Niche   │   │
│  │  Page    │  │   Page   │  │   Page   │  │   Page   │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Admin Dashboard                          │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/REST API
                            │ (JSON)
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                         BACKEND                              │
│                      (Rust + Actix-web)                      │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  API Routes                           │  │
│  │  /auth  /services  /products  /bookings  /purchases  │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   Handlers                            │  │
│  │  Auth  Services  Products  Niche  Bookings  Reviews  │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Authentication Layer                     │  │
│  │              (JWT + Bcrypt)                           │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ SQLx
                            │ (Connection Pool)
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                       DATABASE                               │
│                      (PostgreSQL)                            │
│                                                              │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐       │
│  │  users  │  │services │  │products │  │bookings │       │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘       │
│                                                              │
│  ┌─────────┐  ┌─────────┐                                  │
│  │purchases│  │ reviews │                                  │
│  └─────────┘  └─────────┘                                  │
└─────────────────────────────────────────────────────────────┘
```

## Request Flow

### 1. User Registration Flow
```
User → Frontend (signup form)
  ↓
  POST /api/auth/signup
  ↓
Backend (auth handler)
  ↓
  Hash password (bcrypt)
  ↓
Database (INSERT user)
  ↓
Backend (success response)
  ↓
Frontend (show success message)
```

### 2. User Login Flow
```
User → Frontend (login form)
  ↓
  POST /api/auth/login
  ↓
Backend (auth handler)
  ↓
  Verify password (bcrypt)
  ↓
  Generate JWT token
  ↓
Backend (return token + user data)
  ↓
Frontend (store token in localStorage)
  ↓
Frontend (update UI for logged-in user)
```

### 3. Service Booking Flow
```
User → Frontend (select service)
  ↓
  Click "Book Now"
  ↓
  Fill booking form (date, time, notes)
  ↓
  POST /api/bookings
  Headers: Authorization: Bearer {token}
  ↓
Backend (verify JWT token)
  ↓
Backend (bookings handler)
  ↓
Database (INSERT booking)
  ↓
Backend (success response)
  ↓
Frontend (show confirmation)
```

### 4. Product Purchase Flow
```
User → Frontend (select product)
  ↓
  Click "Buy Now"
  ↓
  Select payment method
  ↓
  POST /api/purchases
  Headers: Authorization: Bearer {token}
  ↓
Backend (verify JWT token)
  ↓
Backend (purchases handler)
  ↓
  Get product details
  ↓
Database (INSERT purchase)
  ↓
Database (UPDATE product downloads)
  ↓
Backend (return download URL)
  ↓
Frontend (show download link)
```

## Data Flow Diagram

```
┌──────────┐
│  User    │
└────┬─────┘
     │
     ├─── Browse Services/Products (No Auth)
     │    └─→ GET /api/services
     │    └─→ GET /api/products
     │    └─→ GET /api/niche/{type}
     │
     ├─── Register/Login
     │    └─→ POST /api/auth/signup
     │    └─→ POST /api/auth/login
     │         └─→ Receive JWT Token
     │
     ├─── Book Service (Auth Required)
     │    └─→ POST /api/bookings
     │         └─→ Send JWT in Header
     │
     ├─── Purchase Product (Auth Required)
     │    └─→ POST /api/purchases
     │         └─→ Send JWT in Header
     │
     └─── Leave Review (Auth Required)
          └─→ POST /api/reviews
               └─→ Send JWT in Header
```

## Database Relationships

```
┌─────────────┐
│    users    │
│  (id: UUID) │
└──────┬──────┘
       │
       ├──────────────────────────────┐
       │                              │
       ↓                              ↓
┌─────────────┐              ┌─────────────┐
│  services   │              │  products   │
│ (provider_id)│              │ (seller_id) │
└──────┬──────┘              └──────┬──────┘
       │                            │
       ↓                            ↓
┌─────────────┐              ┌─────────────┐
│  bookings   │              │  purchases  │
│(customer_id)│              │(customer_id)│
│(service_id) │              │(product_id) │
└─────────────┘              └─────────────┘

       ┌─────────────┐
       │   reviews   │
       │  (user_id)  │
       │  (item_id)  │
       │ (item_type) │
       └─────────────┘
```

## Authentication Architecture

```
┌──────────────────────────────────────────────────┐
│              Authentication Flow                  │
└──────────────────────────────────────────────────┘

1. User Signup:
   Password → Bcrypt Hash → Store in DB

2. User Login:
   Password → Verify Hash → Generate JWT
   
3. JWT Structure:
   {
     "sub": "user_id",
     "exp": "expiration_timestamp"
   }

4. Protected Routes:
   Request → Extract JWT → Verify Signature → Extract User ID
   
5. Token Storage:
   Frontend: localStorage.setItem('token', jwt)
   
6. Token Usage:
   Headers: { Authorization: "Bearer {jwt}" }
```

## API Layer Architecture

```
┌─────────────────────────────────────────────────┐
│                  API Routes                      │
└─────────────────────────────────────────────────┘

/api
 ├── /auth
 │   ├── POST /signup
 │   └── POST /login
 │
 ├── /services
 │   ├── GET  /           (public)
 │   ├── GET  /{id}       (public)
 │   └── POST /           (protected)
 │
 ├── /products
 │   ├── GET  /           (public)
 │   ├── GET  /{id}       (public)
 │   └── POST /           (protected)
 │
 ├── /niche
 │   └── GET  /{type}     (public)
 │
 ├── /bookings
 │   ├── GET  /           (protected)
 │   └── POST /           (protected)
 │
 ├── /purchases
 │   ├── GET  /           (protected)
 │   └── POST /           (protected)
 │
 └── /reviews
     ├── GET  /{type}/{id} (public)
     └── POST /            (protected)
```

## Frontend Architecture

```
┌─────────────────────────────────────────────────┐
│              Frontend Structure                  │
└─────────────────────────────────────────────────┘

index.html (Landing)
 │
 ├── main.js (Auth & Common)
 │   ├── Login Modal
 │   ├── Signup Modal
 │   ├── JWT Storage
 │   └── API Helper Functions
 │
 ├── services.html
 │   └── services.js
 │       ├── Load Services
 │       ├── Filter Services
 │       ├── Book Service
 │       └── View Details
 │
 ├── digital.html
 │   └── digital.js
 │       ├── Load Products
 │       ├── Filter Products
 │       ├── Purchase Product
 │       └── View Details
 │
 ├── niche.html
 │   └── niche.js
 │       ├── Select Niche
 │       ├── Load Niche Products
 │       ├── Filter by Niche
 │       └── Purchase
 │
 └── admin.html
     └── Admin Dashboard
         ├── Statistics
         ├── User Management
         ├── Listings Management
         └── Transaction History
```

## Security Layers

```
┌─────────────────────────────────────────────────┐
│              Security Architecture               │
└─────────────────────────────────────────────────┘

Layer 1: Password Security
  └─→ Bcrypt hashing (cost factor: 12)

Layer 2: Authentication
  └─→ JWT tokens (7-day expiration)

Layer 3: Authorization
  └─→ Token verification on protected routes

Layer 4: Database Security
  └─→ Parameterized queries (SQL injection prevention)

Layer 5: CORS
  └─→ Configured allowed origins

Layer 6: Environment Variables
  └─→ Sensitive data in .env file
```

## Scalability Considerations

```
Current Architecture:
  Single Server → Single Database

Future Scaling Options:

1. Horizontal Scaling:
   ┌────────┐  ┌────────┐  ┌────────┐
   │Server 1│  │Server 2│  │Server 3│
   └────┬───┘  └────┬───┘  └────┬───┘
        └───────┬────────────┘
                ↓
         ┌─────────────┐
         │Load Balancer│
         └─────────────┘

2. Database Scaling:
   ┌──────────┐
   │  Master  │ (Write)
   └────┬─────┘
        │
   ┌────┴─────┬─────────┐
   ↓          ↓         ↓
   Replica1  Replica2  Replica3 (Read)

3. Caching Layer:
   Frontend → Redis Cache → Backend → Database

4. File Storage:
   Products → AWS S3 / CDN

5. Microservices:
   Auth Service | Service Marketplace | Product Marketplace
```

## Performance Optimization

```
Frontend:
  ├── Minimal JavaScript (no frameworks)
  ├── CSS Grid/Flexbox (no heavy libraries)
  ├── Lazy loading for images
  └── Local storage for auth tokens

Backend:
  ├── Rust (zero-cost abstractions)
  ├── Async/await (non-blocking I/O)
  ├── Connection pooling
  └── Efficient SQL queries

Database:
  ├── Indexes on frequently queried columns
  ├── Foreign key constraints
  └── Optimized query patterns
```

---

This architecture is designed for:
- ✅ Simplicity (easy to understand)
- ✅ Scalability (can grow with demand)
- ✅ Security (multiple layers of protection)
- ✅ Performance (fast and efficient)
- ✅ Maintainability (clean code structure)
