let currentNiche = null;

function loadNiche(niche) {
    currentNiche = niche;
    document.getElementById('nicheContent').style.display = 'block';
    
    const titles = {
        resume: 'Resume & Career Resources',
        business: 'Business Tools & Templates',
        student: 'Student Resources',
        creator: 'Creator Tools & Assets',
        developer: 'Developer Resources'
    };
    
    document.getElementById('nicheTitle').textContent = titles[niche];
    fetchNicheProducts(niche);
}

async function fetchNicheProducts(niche) {
    try {
        const response = await fetch(`${API_URL}/niche/${niche}`);
        const products = await response.json();
        displayNicheProducts(products);
    } catch (error) {
        console.error('Error loading niche products:', error);
    }
}

function displayNicheProducts(products) {
    const grid = document.getElementById('nicheGrid');
    grid.innerHTML = products.map(product => `
        <div class="listing-card">
            <div class="listing-image">${product.icon || '🎯'}</div>
            <div class="listing-content">
                <h3 class="listing-title">${product.title}</h3>
                <p class="listing-description">${product.description}</p>
                <div class="listing-meta">
                    <span class="listing-price">$${product.price}</span>
                    <span class="listing-rating">⭐ ${product.rating || '5.0'}</span>
                </div>
                <p style="font-size: 0.9rem; color: #6b7280;">${product.category} • ${product.downloads || 0} sales</p>
                <div class="listing-actions">
                    <button class="btn-primary btn-small" onclick="purchaseNicheProduct(${product.id})">Purchase</button>
                    <button class="btn-secondary btn-small" onclick="viewNicheDetails(${product.id})">View</button>
                </div>
            </div>
        </div>
    `).join('');
}

async function filterNiche() {
    const search = document.getElementById('searchInput').value;
    const sort = document.getElementById('sortFilter').value;

    const params = new URLSearchParams();
    params.append('niche', currentNiche);
    if (search) params.append('search', search);
    if (sort) params.append('sort', sort);

    try {
        const response = await fetch(`${API_URL}/niche/${currentNiche}?${params}`);
        const products = await response.json();
        displayNicheProducts(products);
    } catch (error) {
        console.error('Error filtering niche products:', error);
    }
}

function purchaseNicheProduct(productId) {
    const token = localStorage.getItem('token');
    if (!token) {
        alert('Please login to purchase');
        return;
    }
    window.location.href = `../pages/digital.html?product=${productId}`;
}

function viewNicheDetails(productId) {
    window.location.href = `product-details.html?id=${productId}`;
}
