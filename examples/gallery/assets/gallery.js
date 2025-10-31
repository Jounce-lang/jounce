// Jounce Examples Gallery - Interactive Features

let allApps = [];
let currentCategory = 'all';
let currentSearch = '';

// Load apps catalog
async function loadApps() {
    try {
        const response = await fetch('data/apps.json');
        const data = await response.json();
        allApps = data.apps;

        renderCategories(data.categories);
        renderApps();
    } catch (error) {
        console.error('Failed to load apps:', error);
        document.getElementById('apps-grid').innerHTML = '<p>Failed to load examples. Please try again.</p>';
    }
}

// Render category filter buttons
function renderCategories(categories) {
    const container = document.getElementById('categories');

    // Count apps per category
    const counts = { all: allApps.length };
    categories.forEach(cat => {
        counts[cat] = allApps.filter(app => app.category === cat).length;
    });

    // Create buttons
    const buttons = [
        `<button class="category-btn active" data-category="all">All (${counts.all})</button>`,
        ...categories.sort().map(cat =>
            `<button class="category-btn" data-category="${cat}">${cat} (${counts[cat]})</button>`
        )
    ];

    container.innerHTML = buttons.join('');

    // Add click listeners
    container.querySelectorAll('.category-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            currentCategory = btn.dataset.category;
            updateActiveCategory(btn);
            renderApps();
        });
    });
}

// Update active category button
function updateActiveCategory(activeBtn) {
    document.querySelectorAll('.category-btn').forEach(btn => {
        btn.classList.remove('active');
    });
    activeBtn.classList.add('active');
}

// Render apps grid
function renderApps() {
    const grid = document.getElementById('apps-grid');
    const emptyState = document.getElementById('empty-state');

    // Filter apps
    let filteredApps = allApps;

    // Filter by category
    if (currentCategory !== 'all') {
        filteredApps = filteredApps.filter(app => app.category === currentCategory);
    }

    // Filter by search
    if (currentSearch) {
        const search = currentSearch.toLowerCase();
        filteredApps = filteredApps.filter(app =>
            app.name.toLowerCase().includes(search) ||
            app.description.toLowerCase().includes(search) ||
            app.tags.some(tag => tag.toLowerCase().includes(search)) ||
            app.features.some(feature => feature.toLowerCase().includes(search))
        );
    }

    // Show/hide empty state
    if (filteredApps.length === 0) {
        grid.style.display = 'none';
        emptyState.style.display = 'block';
        return;
    } else {
        grid.style.display = 'grid';
        emptyState.style.display = 'none';
    }

    // Render cards
    grid.innerHTML = filteredApps.map(app => createAppCard(app)).join('');
}

// Create app card HTML
function createAppCard(app) {
    const featuresHTML = app.features.slice(0, 4).map(f =>
        `<span class="feature-tag">${f}</span>`
    ).join('');

    const description = app.description || 'Example Jounce application';

    return `
        <div class="app-card" onclick="viewApp('${app.id}')">
            <div class="app-header">
                <h3 class="app-name">${app.name}</h3>
                <span class="app-category">${app.category}</span>
            </div>
            <p class="app-description">${description}</p>
            <div class="app-features">
                ${featuresHTML}
            </div>
            <div class="app-meta">
                <span class="lines">${app.lines} lines</span>
                <span class="difficulty ${app.difficulty}">${app.difficulty}</span>
            </div>
        </div>
    `;
}

// View app details (opens viewer page)
function viewApp(appId) {
    window.location.href = `viewer.html?app=${appId}`;
}

// Search functionality
const searchInput = document.getElementById('search');
searchInput.addEventListener('input', (e) => {
    currentSearch = e.target.value;
    renderApps();
});

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Press / to focus search
    if (e.key === '/' && document.activeElement !== searchInput) {
        e.preventDefault();
        searchInput.focus();
    }

    // Press Esc to clear search
    if (e.key === 'Escape') {
        searchInput.value = '';
        currentSearch = '';
        searchInput.blur();
        renderApps();
    }
});

// Initialize
loadApps();
