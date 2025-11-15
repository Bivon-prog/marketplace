# MongoDB Setup Checklist

Quick checklist to get MarketHub running with MongoDB.

## ☐ Prerequisites

- [ ] Windows 10/11
- [ ] Internet connection (for downloads)
- [ ] Administrator access

## ☐ Step 1: Install MongoDB

Choose one method:

### Option A: Using Installer (Recommended)
- [ ] Download from: https://www.mongodb.com/try/download/community
- [ ] Run installer
- [ ] Choose "Complete" installation
- [ ] ✅ Check "Install MongoDB as a Service"
- [ ] ✅ Check "Install MongoDB Compass" (optional GUI)
- [ ] Complete installation

### Option B: Using winget
```bash
winget install MongoDB.Server
```

## ☐ Step 2: Verify MongoDB Installation

```bash
# Check version
mongosh --version

# Should show: MongoDB shell version v6.x.x
```

## ☐ Step 3: Start MongoDB Service

```bash
# Check if running
sc query MongoDB

# If not running, start it
net start MongoDB

# Verify connection
mongosh --eval "db.version()"
```

**Expected output**: MongoDB version number (e.g., 6.0.5)

## ☐ Step 4: Install Rust

```bash
# Download and run: https://rustup.rs/
# Or use winget
winget install Rustlang.Rustup
```

Verify:
```bash
rustc --version
cargo --version
```

## ☐ Step 5: Install Python

```bash
# Download from: https://www.python.org/downloads/
# Or use winget
winget install Python.Python.3.11
```

Verify:
```bash
python --version
```

## ☐ Step 6: Configure Backend

- [ ] Navigate to `backend` folder
- [ ] Open `.env` file
- [ ] Verify/update settings:

```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=marketplace_db
JWT_SECRET=your-secret-key-change-this
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

## ☐ Step 7: Initialize Database (Optional)

```bash
# Run initialization script
mongosh < backend/init_mongodb.js
```

**Note**: Backend will create collections automatically if you skip this.

## ☐ Step 8: Build Backend

```bash
cd backend
cargo build
```

**First build takes 5-10 minutes** (downloads dependencies)

## ☐ Step 9: Start Backend

```bash
cargo run
```

**Expected output**:
```
🚀 Server starting on http://127.0.0.1:8080
📦 Connected to MongoDB: marketplace_db
✅ Database indexes created successfully
```

## ☐ Step 10: Start Frontend

Open new terminal:

```bash
cd frontend
python -m http.server 3000
```

**Expected output**:
```
Serving HTTP on :: port 3000 (http://[::]:3000/) ...
```

## ☐ Step 11: Test Application

- [ ] Open browser: http://localhost:3000
- [ ] Click "Sign Up"
- [ ] Create test account
- [ ] Login with account
- [ ] Browse services
- [ ] Browse products

## ☐ Step 12: Verify Database

```bash
# Connect to MongoDB
mongosh marketplace_db

# Check collections
show collections

# View users
db.users.find().pretty()

# Exit
exit
```

## ✅ Success Checklist

- [ ] MongoDB service is running
- [ ] Backend server is running (port 8080)
- [ ] Frontend server is running (port 3000)
- [ ] Can access http://localhost:3000
- [ ] Can create account
- [ ] Can login
- [ ] Can view services/products
- [ ] Database contains data

## 🐛 Troubleshooting

### MongoDB won't start
```bash
# Check if port 27017 is in use
netstat -ano | findstr :27017

# Check MongoDB logs
# Location: C:\Program Files\MongoDB\Server\6.0\log\mongod.log

# Try manual start
mongod --dbpath C:\data\db
```

### Backend won't compile
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### Backend won't start
```bash
# Check MongoDB is running
mongosh --eval "db.version()"

# Check port 8080 is free
netstat -ano | findstr :8080

# Check .env file exists and is correct
```

### Frontend won't load
```bash
# Check backend is running first
# Check port 3000 is free
netstat -ano | findstr :3000

# Try different port
python -m http.server 8000
```

### Can't connect to database
```bash
# Verify MongoDB is running
net start MongoDB

# Test connection
mongosh mongodb://localhost:27017

# Check backend/.env
# MONGODB_URI should be: mongodb://localhost:27017
```

## 📚 Quick Reference

### Start Everything
```bash
# Terminal 1 - MongoDB (if not running as service)
net start MongoDB

# Terminal 2 - Backend
cd backend
cargo run

# Terminal 3 - Frontend
cd frontend
python -m http.server 3000
```

### Stop Everything
```bash
# Stop frontend: Ctrl+C in terminal
# Stop backend: Ctrl+C in terminal
# Stop MongoDB: net stop MongoDB
```

### View Logs
```bash
# Backend logs: In terminal where cargo run is running
# MongoDB logs: C:\Program Files\MongoDB\Server\6.0\log\mongod.log
# Frontend logs: In terminal where python server is running
```

### Access Points
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080/api
- MongoDB: mongodb://localhost:27017
- MongoDB Compass: mongodb://localhost:27017

## 🎯 Next Steps

Once everything is working:

1. [ ] Read MONGODB_GUIDE.md for database operations
2. [ ] Read API_EXAMPLES.md for API testing
3. [ ] Read QUICK_REFERENCE.md for development tips
4. [ ] Customize the platform for your needs
5. [ ] Add sample data
6. [ ] Test all features
7. [ ] Deploy to production

## 📞 Need Help?

Check these files:
- **MONGODB_GUIDE.md** - Complete MongoDB reference
- **SETUP_GUIDE.md** - Detailed setup instructions
- **QUICK_REFERENCE.md** - Common commands
- **MONGODB_MIGRATION.md** - Technical details
- **START_HERE.md** - Quick start guide

---

**Estimated Setup Time**: 30-45 minutes (including downloads)

**Difficulty**: ⭐⭐☆☆☆ (Beginner-friendly)

**Status**: Ready to use! 🚀
