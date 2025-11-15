let currentProductId = null;

async function loadProducts() {
    try {
        const response = await fetch(`${API_URL}/products`);
        const products = await response.json();
        displayProducts(products);
    } catch (error) {
        console.error('Error loading products:', error);
    }
}

function displayProducts(products) {
    const grid = document.getElementById('productsGrid');
    grid.innerHTML = products.map(product => `
        <div class="listing-card">
            <div class="listing-image">${product.icon || '📦'}</div>
            <div class="listing-content">
                <h3 class="listing-title">${product.title}</h3>
                <p class="listing-description">${product.description}</p>
                <div class="listing-meta">
                    <span class="listing-price">$${product.price}</span>
                    <span class="listing-rating">⭐ ${product.rating || '5.0'}</span>
                </div>
                <p style="font-size: 0.9rem; color: #6b7280;">💾 ${product.file_type} • ${product.downloads || 0} downloads</p>
                <div class="listing-actions">
                    <button class="btn-primary btn-small" onclick="openPurchaseModal(${product.id})">Buy Now</button>
                    <button class="btn-secondary btn-small" onclick="viewProductDetails(${product.id})">Preview</button>
                </div>
            </div>
        </div>
    `).join('');
}

function openPurchaseModal(productId) {
    const token = localStorage.getItem('token');
    if (!token) {
        alert('Please login to purchase');
        return;
    }
    currentProductId = productId;
    document.getElementById('purchaseModal').style.display = 'block';
}

function closePurchaseModal() {
    document.getElementById('purchaseModal').style.display = 'none';
}

document.getElementById('purchaseForm').onsubmit = async (e) => {
    e.preventDefault();
    
    const purchaseData = {
        product_id: currentProductId,
        payment_method: document.getElementById('paymentMethod').value
    };

    try {
        const response = await fetch(`${API_URL}/purchases`, {
            method: 'POST',
            headers: getAuthHeaders(),
            body: JSON.stringify(purchaseData)
        });

        const data = await response.json();

        if (response.ok) {
            alert('Purchase successful! Download link: ' + data.download_url);
            closePurchaseModal();
        } else {
            alert('Purchase failed. Please try again.');
        }
    } catch (error) {
        console.error('Purchase error:', error);
        alert('Purchase failed. Please try again.');
    }
};

async function filterProducts() {
    const search = document.getElementById('searchInput').value;
    const category = document.getElementById('categoryFilter').value;
    const price = document.getElementById('priceFilter').value;

    const params = new URLSearchParams();
    if (search) params.append('search', search);
    if (category) params.append('category', category);
    if (price) params.append('price', price);

    try {
        const response = await fetch(`${API_URL}/products?${params}`);
        const products = await response.json();
        displayProducts(products);
    } catch (error) {
        console.error('Error filtering products:', error);
    }
}

function viewProductDetails(productId) {
    window.location.href = `product-details.html?id=${productId}`;
}

loadProducts();
