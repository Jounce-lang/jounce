// Jounce Server Runtime
// Provides HTTP server and RPC infrastructure for server-side code

// Load environment variables from .env file (Session 17)
require('dotenv').config();

const http = require('http');
const url = require('url');
const fs = require('fs');
const path = require('path');
const Database = require('better-sqlite3');

class HttpServer {
    constructor(port = 3000) {
        this.port = port;
        this.rpcHandlers = new Map();
        this.server = null;
    }

    // Register an RPC handler
    rpc(name, handler) {
        this.rpcHandlers.set(name, handler);
    }

    // Start the HTTP server
    start() {
        this.server = http.createServer(async (req, res) => {
            const parsedUrl = url.parse(req.url, true);
            const pathname = parsedUrl.pathname;

            // Serve static files
            if (pathname === '/' || pathname === '/index.html') {
                this.serveFile(res, 'index.html', 'text/html');
            } else if (pathname === '/client.js') {
                this.serveFile(res, 'client.js', 'application/javascript');
            } else if (pathname === '/client-runtime.js') {
                this.serveFile(res, 'client-runtime.js', 'application/javascript');
            } else if (pathname === '/reactivity.js') {
                this.serveFile(res, 'reactivity.js', 'application/javascript');
            } else if (pathname === '/styles.css') {
                this.serveFile(res, 'styles.css', 'text/css');
            } else if (pathname === '/app.wasm') {
                this.serveFile(res, 'app.wasm', 'application/wasm');
            } else if (pathname.startsWith('/rpc/')) {
                // Handle RPC calls
                const rpcName = pathname.slice(5); // Remove '/rpc/' prefix
                await this.handleRPC(rpcName, req, res);
            } else {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('Not Found');
            }
        });

        this.server.listen(this.port, () => {
            console.log(`Server running at http://localhost:${this.port}`);
        });
    }

    // Serve a static file
    serveFile(res, filename, contentType) {
        const filePath = path.join(__dirname, filename);
        fs.readFile(filePath, (err, data) => {
            if (err) {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('File not found');
            } else {
                res.writeHead(200, { 'Content-Type': contentType });
                res.end(data);
            }
        });
    }

    // Handle RPC call
    async handleRPC(name, req, res) {
        const handler = this.rpcHandlers.get(name);
        if (!handler) {
            res.writeHead(404, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'RPC handler not found' }));
            return;
        }

        // Read request body
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });

        req.on('end', async () => {
            try {
                const params = JSON.parse(body || '{}');
                const result = await handler(params);
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(result));
            } catch (error) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: error.message }));
            }
        });
    }
}

// WebAssembly loading utility
function loadWasm(wasmPath) {
    const wasmBytes = fs.readFileSync(wasmPath);
    const wasmModule = new WebAssembly.Module(wasmBytes);
    return new WebAssembly.Instance(wasmModule, {});
}

// ============================================================================
// Database Utilities
// ============================================================================

class DB {
    constructor(filename = 'app.db') {
        // Create database file in dist directory
        const dbPath = path.join(__dirname, filename);
        this.db = new Database(dbPath);
        this.db.pragma('journal_mode = WAL'); // Better concurrency
    }

    // Execute a SQL query (SELECT)
    query(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.all(...params);
        } catch (error) {
            console.error('Query error:', error.message);
            throw error;
        }
    }

    // Execute a SQL command (INSERT, UPDATE, DELETE)
    execute(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.run(...params);
        } catch (error) {
            console.error('Execute error:', error.message);
            throw error;
        }
    }

    // Get a single row
    queryOne(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.get(...params);
        } catch (error) {
            console.error('QueryOne error:', error.message);
            throw error;
        }
    }

    // Close database connection
    close() {
        this.db.close();
    }

    // Begin transaction
    transaction(fn) {
        const txn = this.db.transaction(fn);
        return txn();
    }
}

// Global database instance (lazy initialization)
let globalDB = null;

// Get or create global database instance
function getDB(filename = 'app.db') {
    if (!globalDB) {
        globalDB = new DB(filename);
    }
    return globalDB;
}

// Helper functions for common operations
const dbHelpers = {
    // Create a table
    createTable(tableName, columns) {
        const db = getDB();
        const columnDefs = Object.entries(columns)
            .map(([name, type]) => `${name} ${type}`)
            .join(', ');
        const sql = `CREATE TABLE IF NOT EXISTS ${tableName} (${columnDefs})`;
        db.execute(sql);
    },

    // Insert a record
    insert(tableName, data) {
        const db = getDB();
        const keys = Object.keys(data);
        const values = Object.values(data);
        const placeholders = keys.map(() => '?').join(', ');
        const sql = `INSERT INTO ${tableName} (${keys.join(', ')}) VALUES (${placeholders})`;
        const result = db.execute(sql, values);
        return result.lastInsertRowid;
    },

    // Update records
    update(tableName, data, where, whereParams = []) {
        const db = getDB();
        const sets = Object.keys(data).map(key => `${key} = ?`).join(', ');
        const values = [...Object.values(data), ...whereParams];
        const sql = `UPDATE ${tableName} SET ${sets} WHERE ${where}`;
        const result = db.execute(sql, values);
        return result.changes;
    },

    // Delete records
    delete(tableName, where, whereParams = []) {
        const db = getDB();
        const sql = `DELETE FROM ${tableName} WHERE ${where}`;
        const result = db.execute(sql, whereParams);
        return result.changes;
    },

    // Select records
    select(tableName, where = null, whereParams = []) {
        const db = getDB();
        let sql = `SELECT * FROM ${tableName}`;
        if (where) {
            sql += ` WHERE ${where}`;
        }
        return db.query(sql, whereParams);
    },

    // Select one record
    selectOne(tableName, where, whereParams = []) {
        const db = getDB();
        const sql = `SELECT * FROM ${tableName} WHERE ${where}`;
        return db.queryOne(sql, whereParams);
    },

    // Count records
    count(tableName, where = null, whereParams = []) {
        const db = getDB();
        let sql = `SELECT COUNT(*) as count FROM ${tableName}`;
        if (where) {
            sql += ` WHERE ${where}`;
        }
        const result = db.queryOne(sql, whereParams);
        return result.count;
    }
};

// ==================== WebSocket Server ====================

// WebSocket server with rooms and broadcasting
class WebSocketServer {
    constructor(server, options = {}) {
        this.wss = null;
        this.clients = new Map();  // ws -> { id, rooms: Set }
        this.rooms = new Map();    // room -> Set of ws clients
        this.messageHandlers = [];
        this.connectionHandlers = [];
        this.disconnectionHandlers = [];

        try {
            const WebSocket = require('ws');
            this.wss = new WebSocket.Server({ server, ...options });

            this.wss.on('connection', (ws, req) => {
                const clientId = this.generateClientId();
                this.clients.set(ws, {
                    id: clientId,
                    rooms: new Set(),
                    metadata: {}
                });

                console.log(`[WebSocket] Client ${clientId} connected`);

                // Notify connection handlers
                this.connectionHandlers.forEach(handler => {
                    try {
                        handler(clientId, ws);
                    } catch (error) {
                        console.error('[WebSocket] Error in connection handler:', error);
                    }
                });

                ws.on('message', (data) => {
                    try {
                        const message = JSON.parse(data);
                        this.handleMessage(ws, message);
                    } catch (error) {
                        console.error('[WebSocket] Error parsing message:', error);
                    }
                });

                ws.on('close', () => {
                    const client = this.clients.get(ws);
                    if (client) {
                        console.log(`[WebSocket] Client ${client.id} disconnected`);

                        // Remove from all rooms
                        client.rooms.forEach(room => {
                            this.leaveRoom(ws, room);
                        });

                        // Notify disconnection handlers
                        this.disconnectionHandlers.forEach(handler => {
                            try {
                                handler(client.id, ws);
                            } catch (error) {
                                console.error('[WebSocket] Error in disconnection handler:', error);
                            }
                        });

                        this.clients.delete(ws);
                    }
                });

                ws.on('error', (error) => {
                    console.error('[WebSocket] Client error:', error);
                });
            });

            console.log('[WebSocket] Server initialized');
        } catch (error) {
            console.warn('[WebSocket] ws package not installed, WebSocket support disabled');
            console.warn('[WebSocket] Install with: npm install ws');
        }
    }

    // Generate unique client ID
    generateClientId() {
        return `client_${Math.random().toString(36).substring(7)}_${Date.now()}`;
    }

    // Handle incoming message
    handleMessage(ws, message) {
        const client = this.clients.get(ws);
        if (!client) return;

        // Handle built-in message types
        if (message.type === 'join_room') {
            this.joinRoom(ws, message.data.room);
            return;
        }

        if (message.type === 'leave_room') {
            this.leaveRoom(ws, message.data.room);
            return;
        }

        if (message.type === 'broadcast') {
            this.broadcastToRoom(message.data.room, message.data.type, message.data.data, ws);
            return;
        }

        // Notify custom message handlers
        this.messageHandlers.forEach(handler => {
            try {
                handler(client.id, message, ws);
            } catch (error) {
                console.error('[WebSocket] Error in message handler:', error);
            }
        });
    }

    // Join a room
    joinRoom(ws, roomName) {
        const client = this.clients.get(ws);
        if (!client) return;

        // Add client to room
        if (!this.rooms.has(roomName)) {
            this.rooms.set(roomName, new Set());
        }
        this.rooms.get(roomName).add(ws);
        client.rooms.add(roomName);

        console.log(`[WebSocket] Client ${client.id} joined room ${roomName}`);

        // Notify client
        this.send(ws, 'room_joined', { room: roomName });
    }

    // Leave a room
    leaveRoom(ws, roomName) {
        const client = this.clients.get(ws);
        if (!client) return;

        // Remove client from room
        if (this.rooms.has(roomName)) {
            this.rooms.get(roomName).delete(ws);
            if (this.rooms.get(roomName).size === 0) {
                this.rooms.delete(roomName);
            }
        }
        client.rooms.delete(roomName);

        console.log(`[WebSocket] Client ${client.id} left room ${roomName}`);

        // Notify client
        this.send(ws, 'room_left', { room: roomName });
    }

    // Send message to client
    send(ws, type, data) {
        if (ws.readyState === 1) { // OPEN
            const message = {
                type,
                data,
                timestamp: Date.now()
            };
            ws.send(JSON.stringify(message));
        }
    }

    // Broadcast to all clients
    broadcast(type, data, excludeWs = null) {
        const message = JSON.stringify({
            type,
            data,
            timestamp: Date.now()
        });

        this.clients.forEach((client, ws) => {
            if (ws !== excludeWs && ws.readyState === 1) {
                ws.send(message);
            }
        });
    }

    // Broadcast to room
    broadcastToRoom(roomName, type, data, excludeWs = null) {
        const room = this.rooms.get(roomName);
        if (!room) return;

        const message = JSON.stringify({
            type,
            data,
            timestamp: Date.now()
        });

        room.forEach(ws => {
            if (ws !== excludeWs && ws.readyState === 1) {
                ws.send(message);
            }
        });
    }

    // Register message handler
    onMessage(handler) {
        this.messageHandlers.push(handler);
    }

    // Register connection handler
    onConnection(handler) {
        this.connectionHandlers.push(handler);
    }

    // Register disconnection handler
    onDisconnection(handler) {
        this.disconnectionHandlers.push(handler);
    }

    // Get client count
    getClientCount() {
        return this.clients.size;
    }

    // Get room count
    getRoomCount() {
        return this.rooms.size;
    }

    // Get clients in room
    getClientsInRoom(roomName) {
        const room = this.rooms.get(roomName);
        return room ? Array.from(room).map(ws => this.clients.get(ws).id) : [];
    }
}

// ============================================================================
// Global Database Initialization
// ============================================================================

// Initialize global.db for server functions to use
// Server functions expect global.db to be available
if (!global.db) {
    const dbInstance = getDB(); // Create DB instance
    global.db = dbInstance.db;  // Assign raw better-sqlite3 Database object to global
}

module.exports = {
    HttpServer,
    loadWasm,
    DB,
    getDB,
    dbHelpers,
    WebSocketServer
};
