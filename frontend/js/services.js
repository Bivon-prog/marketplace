let currentServiceId = null;

async function loadServices() {
    try {
        const response = await fetch(`${API_URL}/services`);
        const services = await response.json();
        displayServices(services);
    } catch (error) {
        console.error('Error loading services:', error);
    }
}

function displayServices(services) {
    const grid = document.getElementById('servicesGrid');
    grid.innerHTML = services.map(service => `
        <div class="listing-card">
            <div class="listing-image">${service.icon || '🔧'}</div>
            <div class="listing-content">
                <h3 class="listing-title">${service.title}</h3>
                <p class="listing-description">${service.description}</p>
                <div class="listing-meta">
                    <span class="listing-price">$${service.price}/hr</span>
                    <span class="listing-rating">⭐ ${service.rating || '5.0'}</span>
                </div>
                <p style="font-size: 0.9rem; color: #6b7280;">📍 ${service.location}</p>
                <div class="listing-actions">
                    <button class="btn-primary btn-small" onclick="openBookingModal(${service.id})">Book Now</button>
                    <button class="btn-secondary btn-small" onclick="viewServiceDetails(${service.id})">Details</button>
                </div>
            </div>
        </div>
    `).join('');
}

function openBookingModal(serviceId) {
    const token = localStorage.getItem('token');
    if (!token) {
        alert('Please login to book a service');
        return;
    }
    currentServiceId = serviceId;
    document.getElementById('bookingModal').style.display = 'block';
}

function closeBookingModal() {
    document.getElementById('bookingModal').style.display = 'none';
}

document.getElementById('bookingForm').onsubmit = async (e) => {
    e.preventDefault();
    
    const bookingData = {
        service_id: currentServiceId,
        booking_date: document.getElementById('bookingDate').value,
        booking_time: document.getElementById('bookingTime').value,
        notes: document.getElementById('bookingNotes').value
    };

    try {
        const response = await fetch(`${API_URL}/bookings`, {
            method: 'POST',
            headers: getAuthHeaders(),
            body: JSON.stringify(bookingData)
        });

        if (response.ok) {
            alert('Booking confirmed!');
            closeBookingModal();
        } else {
            alert('Booking failed. Please try again.');
        }
    } catch (error) {
        console.error('Booking error:', error);
        alert('Booking failed. Please try again.');
    }
};

async function filterServices() {
    const search = document.getElementById('searchInput').value;
    const category = document.getElementById('categoryFilter').value;
    const location = document.getElementById('locationFilter').value;
    const price = document.getElementById('priceFilter').value;

    const params = new URLSearchParams();
    if (search) params.append('search', search);
    if (category) params.append('category', category);
    if (location) params.append('location', location);
    if (price) params.append('price', price);

    try {
        const response = await fetch(`${API_URL}/services?${params}`);
        const services = await response.json();
        displayServices(services);
    } catch (error) {
        console.error('Error filtering services:', error);
    }
}

function viewServiceDetails(serviceId) {
    window.location.href = `service-details.html?id=${serviceId}`;
}

loadServices();
