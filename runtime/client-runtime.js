// Jounce Client Runtime
// Provides JSX rendering and client-side utilities

// Simple JSX createElement function (h function)
export function h(tag, props, ...children) {
    if (typeof tag === 'function') {
        return tag(props, children);
    }

    const element = document.createElement(tag);

    // Set properties
    if (props) {
        for (const [key, value] of Object.entries(props)) {
            if (key === 'className') {
                element.className = value;
            } else if (key === 'class') {
                element.className = value;
            } else if (key.startsWith('on')) {
                const eventName = key.substring(2).toLowerCase();
                element.addEventListener(eventName, value);
            } else if (key === 'style' && typeof value === 'object') {
                Object.assign(element.style, value);
            } else {
                element.setAttribute(key, value);
            }
        }
    }

    // Append children
    for (const child of children.flat()) {
        if (child === null || child === undefined) {
            continue;
        } else if (typeof child === 'string' || typeof child === 'number') {
            element.appendChild(document.createTextNode(String(child)));
        } else if (child instanceof Node) {
            element.appendChild(child);
        }
    }

    return element;
}

// Mount a component to the DOM
export function mountComponent(component, selector = '#app') {
    const container = document.querySelector(selector);
    if (!container) {
        console.error(`Mount target "${selector}" not found`);
        return;
    }

    // Clear existing content
    container.innerHTML = '';

    // Render component
    const rendered = typeof component === 'function' ? component() : component;

    if (rendered instanceof Node) {
        container.appendChild(rendered);
    } else {
        console.error('Component did not return a valid DOM node');
    }
}

// RPC Client for calling server functions
export class RPCClient {
    constructor(baseUrl = '') {
        this.baseUrl = baseUrl;
    }

    async call(functionName, params = {}) {
        const response = await fetch(`${this.baseUrl}/rpc/${functionName}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(params),
        });

        if (!response.ok) {
            throw new Error(`RPC call failed: ${response.statusText}`);
        }

        return await response.json();
    }
}

// Jounce Router - Client-side routing with browser history API
export class JounceRouter {
    constructor() {
        this.routes = new Map(); // path -> render function
        this.currentPath = window.location.pathname;
        this.params = {};

        // Listen to popstate (back/forward buttons)
        window.addEventListener('popstate', () => {
            this.handleRoute(window.location.pathname);
        });
    }

    // Register a route with a render function
    route(path, renderFn) {
        this.routes.set(path, renderFn);
    }

    // Navigate to a path (pushes to history)
    navigate(path) {
        if (path !== this.currentPath) {
            window.history.pushState({}, '', path);
            this.handleRoute(path);
        }
    }

    // Handle route change - find matching route and render
    handleRoute(path) {
        this.currentPath = path;

        // Try exact match first
        if (this.routes.has(path)) {
            const renderFn = this.routes.get(path);
            renderFn();
            return;
        }

        // Try pattern matching for dynamic routes like /user/:id
        for (const [pattern, renderFn] of this.routes) {
            const params = this.matchRoute(pattern, path);
            if (params) {
                this.params = params;
                renderFn();
                return;
            }
        }

        // No match - render 404
        this.render404();
    }

    // Match a route pattern against actual path
    // Pattern: /user/:id, Path: /user/123 -> { id: "123" }
    matchRoute(pattern, path) {
        const patternParts = pattern.split('/').filter(p => p);
        const pathParts = path.split('/').filter(p => p);

        if (patternParts.length !== pathParts.length) {
            return null;
        }

        const params = {};
        for (let i = 0; i < patternParts.length; i++) {
            if (patternParts[i].startsWith(':')) {
                // Dynamic segment
                const paramName = patternParts[i].slice(1);
                params[paramName] = pathParts[i];
            } else if (patternParts[i] !== pathParts[i]) {
                // Static segment doesn't match
                return null;
            }
        }

        return params;
    }

    // Get URL parameter by name
    getParam(name) {
        return this.params[name] || '';
    }

    // Get current path
    getCurrentPath() {
        return this.currentPath;
    }

    // 404 handler
    render404() {
        const appEl = document.getElementById('app');
        if (appEl) {
            appEl.innerHTML = '<h1>404 - Page Not Found</h1>';
        }
    }

    // Start the router - call this after routes are registered
    start() {
        this.handleRoute(this.currentPath);
    }
}

// Global router instance
let globalRouter = null;

// Get or create global router
export function getRouter() {
    if (!globalRouter) {
        globalRouter = new JounceRouter();
    }
    return globalRouter;
}

// Convenience function for navigation
export function navigate(path) {
    getRouter().navigate(path);
}

// Export for window.Jounce global
if (typeof window !== 'undefined') {
    window.Jounce = {
        h,
        mountComponent,
        RPCClient,
        JounceRouter,
        getRouter,
        navigate,
    };
}
