# MarketHub - API Examples

Complete collection of API request examples for testing.

## Base URL
```
http://localhost:8080/api
```

---

## 🔐 Authentication

### 1. User Signup

**Request:**
```bash
POST /api/auth/signup
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123",
  "user_type": "customer"
}
```

**Response:**
```json
{
  "success": true,
  "message": "User created successfully"
}
```

**User Types:**
- `customer` - Can book services and buy products
- `provider` - Can offer services
- `seller` - Can sell digital products

---

### 2. User Login

**Request:**
```bash
POST /api/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "user_type": "customer"
  }
}
```

---

## 🔧 Services

### 3. Get All Services

**Request:**
```bash
GET /api/services
```

**Response:**
```json
[
  {
    "id": 1,
    "provider_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Professional Plumbing",
    "description": "Expert plumbing services for homes and businesses",
    "category": "home",
    "price": 50.00,
    "location": "Nairobi CBD",
    "icon": "🔧",
    "rating": 4.8,
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

### 4. Get Services with Filters

**Request:**
```bash
GET /api/services?category=home&location=Nairobi&search=plumbing
```

**Query Parameters:**
- `category` - Filter by category (home, personal, tech, business)
- `location` - Filter by location (partial match)
- `search` - Search in title and description

---

### 5. Get Service by ID

**Request:**
```bash
GET /api/services/1
```

**Response:**
```json
{
  "id": 1,
  "provider_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Professional Plumbing",
  "description": "Expert plumbing services for homes and businesses",
  "category": "home",
  "price": 50.00,
  "location": "Nairobi CBD",
  "icon": "🔧",
  "rating": 4.8,
  "created_at": "2025-01-15T10:00:00Z"
}
```

---

### 6. Create Service (Auth Required)

**Request:**
```bash
POST /api/services
Authorization: Bearer {your_jwt_token}
Content-Type: application/json

{
  "title": "Home Cleaning Service",
  "description": "Professional cleaning for your home",
  "category": "home",
  "price": 30.00,
  "location": "Westlands, Nairobi",
  "icon": "🧹"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Service created successfully"
}
```

---

## 📦 Products

### 7. Get All Products

**Request:**
```bash
GET /api/products
```

**Response:**
```json
[
  {
    "id": 1,
    "seller_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Professional CV Template",
    "description": "Modern and clean CV template",
    "category": "career",
    "price": 5.00,
    "file_type": "PDF",
    "file_url": "https://example.com/cv-template.pdf",
    "icon": "📄",
    "rating": 4.7,
    "downloads": 150,
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

### 8. Get Products with Filters

**Request:**
```bash
GET /api/products?category=business&search=template&price=0-10
```

**Query Parameters:**
- `category` - Filter by category
- `search` - Search in title and description
- `price` - Price range filter

---

### 9. Get Product by ID

**Request:**
```bash
GET /api/products/1
```

---

### 10. Create Product (Auth Required)

**Request:**
```bash
POST /api/products
Authorization: Bearer {your_jwt_token}
Content-Type: application/json

{
  "title": "Business Plan Template",
  "description": "Complete business plan template with financial projections",
  "category": "business",
  "price": 15.00,
  "file_type": "DOCX",
  "file_url": "https://example.com/business-plan.docx",
  "icon": "💼"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Product created successfully"
}
```

---

## 🎯 Niche Markets

### 11. Get Resume & Career Products

**Request:**
```bash
GET /api/niche/resume
```

**Available Niches:**
- `resume` - Resume & Career resources
- `business` - Business tools & templates
- `student` - Student resources
- `creator` - Creator tools & assets
- `developer` - Developer resources

---

### 12. Get Niche Products with Filters

**Request:**
```bash
GET /api/niche/business?search=invoice&sort=popular
```

**Query Parameters:**
- `search` - Search term
- `sort` - Sort order (popular, recent, price-low, price-high)

---

## 📅 Bookings

### 13. Create Booking (Auth Required)

**Request:**
```bash
POST /api/bookings
Authorization: Bearer {your_jwt_token}
Content-Type: application/json

{
  "service_id": 1,
  "booking_date": "2025-01-20",
  "booking_time": "10:00",
  "notes": "Please bring necessary tools"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Booking created successfully"
}
```

---

### 14. Get User Bookings (Auth Required)

**Request:**
```bash
GET /api/bookings
Authorization: Bearer {your_jwt_token}
```

**Response:**
```json
[
  {
    "id": 1,
    "customer_id": "550e8400-e29b-41d4-a716-446655440000",
    "service_id": 1,
    "booking_date": "2025-01-20",
    "booking_time": "10:00",
    "notes": "Please bring necessary tools",
    "status": "pending",
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

## 💳 Purchases

### 15. Purchase Product (Auth Required)

**Request:**
```bash
POST /api/purchases
Authorization: Bearer {your_jwt_token}
Content-Type: application/json

{
  "product_id": 1,
  "payment_method": "mpesa"
}
```

**Payment Methods:**
- `mpesa` - MPESA mobile money
- `paypal` - PayPal
- `card` - Credit/Debit card

**Response:**
```json
{
  "success": true,
  "message": "Purchase successful",
  "download_url": "https://example.com/cv-template.pdf"
}
```

---

### 16. Get User Purchases (Auth Required)

**Request:**
```bash
GET /api/purchases
Authorization: Bearer {your_jwt_token}
```

**Response:**
```json
[
  {
    "id": 1,
    "customer_id": "550e8400-e29b-41d4-a716-446655440000",
    "product_id": 1,
    "payment_method": "mpesa",
    "amount": 5.00,
    "status": "completed",
    "download_url": "https://example.com/cv-template.pdf",
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

## ⭐ Reviews

### 17. Create Review (Auth Required)

**Request:**
```bash
POST /api/reviews
Authorization: Bearer {your_jwt_token}
Content-Type: application/json

{
  "item_id": 1,
  "item_type": "service",
  "rating": 5,
  "comment": "Excellent service! Very professional and on time."
}
```

**Item Types:**
- `service` - Review for a service
- `product` - Review for a product

**Rating:** 1-5 stars

**Response:**
```json
{
  "success": true,
  "message": "Review created successfully"
}
```

---

### 18. Get Reviews for Service

**Request:**
```bash
GET /api/reviews/service/1
```

**Response:**
```json
[
  {
    "id": 1,
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "item_id": 1,
    "item_type": "service",
    "rating": 5,
    "comment": "Excellent service! Very professional and on time.",
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

### 19. Get Reviews for Product

**Request:**
```bash
GET /api/reviews/product/1
```

---

## 🧪 Testing with cURL

### Example: Complete User Flow

```bash
# 1. Signup
curl -X POST http://localhost:8080/api/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@test.com","password":"password123","user_type":"customer"}'

# 2. Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"password123"}'

# Save the token from response
TOKEN="your_jwt_token_here"

# 3. Browse Services
curl http://localhost:8080/api/services

# 4. Book a Service
curl -X POST http://localhost:8080/api/bookings \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"service_id":1,"booking_date":"2025-01-20","booking_time":"10:00","notes":"Test booking"}'

# 5. Browse Products
curl http://localhost:8080/api/products

# 6. Purchase a Product
curl -X POST http://localhost:8080/api/purchases \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"product_id":1,"payment_method":"mpesa"}'

# 7. Leave a Review
curl -X POST http://localhost:8080/api/reviews \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"item_id":1,"item_type":"service","rating":5,"comment":"Great service!"}'
```

---

## 🧪 Testing with Postman

### Import Collection

Create a new Postman collection with these settings:

**Variables:**
- `base_url`: `http://localhost:8080/api`
- `token`: (will be set after login)

**Authorization:**
- Type: Bearer Token
- Token: `{{token}}`

### Test Sequence

1. **Signup** → Save user credentials
2. **Login** → Save token to collection variable
3. **Get Services** → Verify data
4. **Create Service** → Test with token
5. **Book Service** → Test with token
6. **Get Products** → Verify data
7. **Purchase Product** → Test with token
8. **Create Review** → Test with token

---

## 📊 Response Status Codes

- `200` - Success
- `400` - Bad Request (invalid data)
- `401` - Unauthorized (missing/invalid token)
- `404` - Not Found
- `500` - Internal Server Error

---

## 🔒 Authentication Notes

1. **Token Expiration**: 7 days
2. **Token Format**: `Bearer {jwt_token}`
3. **Token Storage**: Store in localStorage on frontend
4. **Protected Routes**: Require `Authorization` header

---

## 💡 Tips

1. Always include `Content-Type: application/json` header
2. For protected routes, include `Authorization: Bearer {token}`
3. Save the JWT token after login for subsequent requests
4. Check response status codes for error handling
5. Use query parameters for filtering and searching

---

**Happy Testing! 🚀**
