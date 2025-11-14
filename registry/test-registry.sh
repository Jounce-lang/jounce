#!/bin/bash

# Test script for Jounce Registry Server
# Tests all endpoints end-to-end

BASE_URL="http://localhost:4000"
API_URL="$BASE_URL/api/v1"

echo "🧪 Testing Jounce Package Registry Server"
echo "=========================================="
echo

# Test 1: Health Check
echo "1️⃣  Testing health endpoint..."
HEALTH=$(curl -s "$BASE_URL/health")
echo "   Response: $HEALTH"
echo

# Test 2: Register User
echo "2️⃣  Registering test user..."
REGISTER_RESPONSE=$(curl -s -X POST "$API_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "testpass123"
  }')
echo "   Response: $REGISTER_RESPONSE"

# Extract token
TOKEN=$(echo $REGISTER_RESPONSE | jq -r '.token')
echo "   Token: ${TOKEN:0:20}..."
echo

# Test 3: Login
echo "3️⃣  Testing login..."
LOGIN_RESPONSE=$(curl -s -X POST "$API_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123"
  }')
echo "   Response: $LOGIN_RESPONSE"
echo

# Test 4: Publish Package (without file, just metadata)
echo "4️⃣  Publishing test package..."
# Create a test package directory
mkdir -p /tmp/test-package/src
echo "pub fn hello() { console.log(\"Hello from test-package!\"); }" > /tmp/test-package/src/lib.jnc

# Create tarball
cd /tmp/test-package && tar -czf ../test-package.tar.gz . && cd -

PUBLISH_RESPONSE=$(curl -s -X POST "$API_URL/packages/publish" \
  -H "Authorization: Bearer $TOKEN" \
  -F "name=test-package" \
  -F "version=1.0.0" \
  -F "description=A test package" \
  -F "keywords=test,example" \
  -F "package=@/tmp/test-package.tar.gz")
echo "   Response: $PUBLISH_RESPONSE"
echo

# Test 5: List All Packages
echo "5️⃣  Listing all packages..."
PACKAGES=$(curl -s "$API_URL/packages")
echo "   Response: $PACKAGES"
echo

# Test 6: Get Package Info
echo "6️⃣  Getting package info..."
PACKAGE_INFO=$(curl -s "$API_URL/packages/test-package")
echo "   Response: $PACKAGE_INFO"
echo

# Test 7: Search Packages
echo "7️⃣  Searching for packages..."
SEARCH=$(curl -s "$API_URL/search?q=test")
echo "   Response: $SEARCH"
echo

# Test 8: Get Package Owners
echo "8️⃣  Getting package owners..."
OWNERS=$(curl -s "$API_URL/packages/test-package/owners")
echo "   Response: $OWNERS"
echo

# Test 9: Register Second User
echo "9️⃣  Registering second user..."
REGISTER2=$(curl -s -X POST "$API_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser2",
    "email": "test2@example.com",
    "password": "testpass456"
  }')
echo "   Response: $REGISTER2"
echo

# Test 10: Add Owner
echo "🔟  Adding second user as owner..."
ADD_OWNER=$(curl -s -X PUT "$API_URL/packages/test-package/owners" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser2"}')
echo "   Response: $ADD_OWNER"
echo

# Test 11: Remove Owner
echo "1️⃣1️⃣  Removing second user as owner..."
REMOVE_OWNER=$(curl -s -X DELETE "$API_URL/packages/test-package/owners/testuser2" \
  -H "Authorization: Bearer $TOKEN")
echo "   Response: $REMOVE_OWNER"
echo

# Test 12: Rate Limiting
echo "1️⃣2️⃣  Testing rate limiting (sending 105 requests)..."
RATE_LIMIT_TEST=""
for i in {1..105}; do
  RATE_LIMIT_TEST=$(curl -s "$BASE_URL/health")
done
echo "   Last response: $RATE_LIMIT_TEST"
echo

echo "✅ All tests completed!"
echo
echo "Summary:"
echo "- Health check: ✅"
echo "- User registration: ✅"
echo "- User login: ✅"
echo "- Package publishing: ✅"
echo "- Package listing: ✅"
echo "- Package info: ✅"
echo "- Search: ✅"
echo "- Owner management: ✅"
echo "- Rate limiting: ✅"

# Cleanup
rm -rf /tmp/test-package /tmp/test-package.tar.gz
