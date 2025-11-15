# MarketHub - Multi-Marketplace Platform

A comprehensive marketplace platform combining three marketplace types:
1. **Local Service Marketplace** - Connect customers with local service providers
2. **Digital Product Marketplace** - Buy and sell digital products instantly
3. **Niche Marketplaces** - Specialized markets for specific categories

## Tech Stack

### Frontend
- HTML5
- CSS3
- Vanilla JavaScript

### Backend
- Rust
- Actix-web (Web framework)
- SQLx (Database)
- PostgreSQL
- JWT Authentication
- Bcrypt (Password hashing)

## Features

### For Customers
- Browse services and products
- Search and filter by category, location, price
- Book services with date/time selection
- Purchase digital products with instant download
- Leave reviews and ratings
- Secure payment processing (MPESA, PayPal, Card)

### For Service Providers
- Create service listings
- Manage bookings
- View earnings and analytics
- Receive customer reviews

### For Digital Sellers
- Upload digital products
- Track downloads and sales
- Manage product listings
- Withdraw earnings

### Admin Features
- User management
- Approve/reject listings
- Handle disputes
- Track commissions
- Analytics dashboard

## Setup Instructions

### Prerequisites
- Rust (latest stable version)
- MongoDB 6.0+
- Python 3.x (for frontend server)

### Database Setup

1. Install MongoDB
   - Download from: https://www.mongodb.com/try/download/community
   - Or use package manager: `winget install MongoDB.Server`

2. Start MongoDB service:
```bash
# Windows - MongoDB should start automatically as a service
# Or manually: net start MongoDB

# Verify it's running
mongosh --eval "db.version()"
```

3. Initialize the database (optional - backend will create collections automatically):
```bash
mongosh < backend/init_mongodb.js
```

### Backend Setup

1. Navigate to backend directory:
```bash
cd backend
```

2. Update `.env` file with your MongoDB connection:
```
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=marketplace_db
JWT_SECRET=your-secret-key-change-this
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

3. Build and run:
```bash
cargo build --release
cargo run
```

The server will start on `http://localhost:8080`

### Frontend Setup

1. Open `frontend/index.html` in a web browser, or serve it using a simple HTTP server:

```bash
# Using Python
cd frontend
python -m http.server 3000

# Or using Node.js http-server
npx http-server frontend -p 3000
```

2. Access the application at `http://localhost:3000`

## API Endpoints

### Authentication
- `POST /api/auth/signup` - Register new user
- `POST /api/auth/login` - Login user

### Services
- `GET /api/services` - Get all services (with filters)
- `GET /api/services/{id}` - Get service by ID
- `POST /api/services` - Create new service (auth required)

### Products
- `GET /api/products` - Get all products (with filters)
- `GET /api/products/{id}` - Get product by ID
- `POST /api/products` - Create new product (auth required)

### Niche Markets
- `GET /api/niche/{niche_type}` - Get products by niche (resume, business, student, creator, developer)

### Bookings
- `POST /api/bookings` - Create booking (auth required)
- `GET /api/bookings` - Get user bookings (auth required)

### Purchases
- `POST /api/purchases` - Purchase product (auth required)
- `GET /api/purchases` - Get user purchases (auth required)

### Reviews
- `POST /api/reviews` - Create review (auth required)
- `GET /api/reviews/{item_type}/{item_id}` - Get reviews for item

## Default Test Accounts

```
Provider Account:
Email: provider@test.com
Password: password123

Seller Account:
Email: seller@test.com
Password: password123

Customer Account:
Email: customer@test.com
Password: password123
```

## Revenue Models

1. **Commission per transaction** (10-30%)
2. **Subscription plans** for providers/sellers
3. **Featured listings** (paid promotion)
4. **Premium memberships** for customers
5. **Advertisements**

## Project Structure

```
.
├── frontend/
│   ├── index.html
│   ├── pages/
│   │   ├── services.html
│   │   ├── digital.html
│   │   └── niche.html
│   ├── css/
│   │   └── styles.css
│   └── js/
│       ├── main.js
│       ├── services.js
│       ├── digital.js
│       └── niche.js
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── auth.rs
│   │   ├── db.rs
│   │   └── handlers/
│   │       ├── auth.rs
│   │       ├── services.rs
│   │       ├── products.rs
│   │       ├── niche.rs
│   │       ├── bookings.rs
│   │       ├── purchases.rs
│   │       └── reviews.rs
│   ├── Cargo.toml
│   ├── .env
│   └── init_db.sql
└── README.md
```

## Future Enhancements

- Real-time chat between customers and providers
- Advanced analytics dashboard
- Mobile app (React Native)
- AI-powered recommendations
- Multi-language support
- Payment gateway integration (MPESA API, Stripe)
- Email notifications
- SMS notifications
- File upload for digital products
- Image upload for services
- Advanced search with Elasticsearch
- Caching with Redis

## License

MIT License
