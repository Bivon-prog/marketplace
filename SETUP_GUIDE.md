# Quick Setup Guide for MarketHub

## Prerequisites Installation

### 1. Install Rust
```bash
# Windows (using PowerShell)
# Download and run: https://win.rustup.rs/x86_64

# Or use winget
winget install Rustlang.Rustup
```

### 2. Install MongoDB
```bash
# Download from: https://www.mongodb.com/try/download/community
# Or use winget
winget install MongoDB.Server
```

### 3. Install Python (for frontend server)
```bash
# Download from: https://www.python.org/downloads/
# Or use winget
winget install Python.Python.3.11
```

## Database Setup

### Step 1: Start MongoDB
MongoDB should start automatically as a Windows service after installation.

To verify it's running:
```bash
mongosh --eval "db.version()"
```

If not running, start it:
```bash
net start MongoDB
```

### Step 2: Initialize Database (Optional)
The backend will automatically create collections and indexes, but you can pre-initialize:

```bash
mongosh < backend/init_mongodb.js
```

This creates:
- Collections with validation rules
- Indexes for better performance
- Database structure

## Backend Setup

### Step 1: Configure Environment
Edit `backend/.env` file:

```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=marketplace_db
JWT_SECRET=your-secret-key-change-this-in-production
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

If your MongoDB requires authentication:
```env
MONGODB_URI=mongodb://username:password@localhost:27017
```

### Step 2: Build and Run
```bash
cd backend
cargo build --release
cargo run
```

The backend will start on `http://localhost:8080`

## Frontend Setup

### Option 1: Using Python
```bash
cd frontend
python -m http.server 3000
```

### Option 2: Using Node.js (if installed)
```bash
cd frontend
npx http-server -p 3000
```

### Option 3: Open directly in browser
Simply open `frontend/index.html` in your browser (some features may not work due to CORS)

The frontend will be available at `http://localhost:3000`

## Quick Start (Windows)

Simply double-click `start.bat` to start both backend and frontend automatically!

## Testing the Application

### 1. Open your browser
Navigate to `http://localhost:3000`

### 2. Create an account
Click "Sign Up" and create a test account:
- Name: Your Name
- Email: test@example.com
- Password: password123
- User Type: Customer

### 3. Test Features

#### Browse Services
- Click "Local Services"
- View available services
- Try filtering by category

#### Browse Digital Products
- Click "Digital Products"
- View available products
- Try searching

#### Explore Niche Markets
- Click "Niche Markets"
- Select a niche (Resume, Business, Student, etc.)
- Browse specialized products

#### Book a Service (requires login)
- Login with your account
- Go to Local Services
- Click "Book Now" on any service
- Fill in date and time

#### Purchase a Product (requires login)
- Login with your account
- Go to Digital Products
- Click "Buy Now" on any product
- Select payment method

## Default Test Accounts

The database comes with pre-configured test accounts:

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

## Troubleshooting

### Backend won't start
- Check if MongoDB is running: `mongosh --eval "db.version()"`
- Verify MONGODB_URI in `.env` is correct
- Make sure port 8080 is not in use
- Check backend logs for errors

### Frontend won't load
- Check if backend is running first
- Verify port 3000 is not in use
- Check browser console for errors

### Database connection error
- Verify MongoDB is running: `net start MongoDB`
- Check MONGODB_URI in `.env`
- Ensure MongoDB service is started
- Try connecting with mongosh: `mongosh mongodb://localhost:27017`

### CORS errors
- Make sure backend is running on port 8080
- Check that frontend is accessing correct API_URL
- Use a proper HTTP server (not file://)

## Next Steps

1. Customize the design in `frontend/css/styles.css`
2. Add more sample data to the database
3. Implement payment gateway integration
4. Add file upload functionality
5. Create admin dashboard
6. Add email notifications
7. Implement real-time chat

## API Testing

You can test the API using tools like:
- Postman
- Thunder Client (VS Code extension)
- curl commands

Example:
```bash
# Test signup
curl -X POST http://localhost:8080/api/auth/signup \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Test User\",\"email\":\"test@test.com\",\"password\":\"password123\",\"user_type\":\"customer\"}"

# Test login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```

## Support

For issues or questions:
1. Check the README.md for detailed documentation
2. Review the code comments
3. Check PostgreSQL and Rust logs for errors

Happy coding! 🚀
