/**
 * Jounce Security Runtime
 *
 * Provides security middleware functions for authentication, validation,
 * rate limiting, and sanitization.
 *
 * This module is automatically imported when security annotations are used.
 */

// Global security context for the current request
let __jounce_security_context = {
  user: null,
  session: null,
  request: null
};

/**
 * Set the security context for the current request
 *
 * This should be called by your framework adapter (Express, Fastify, etc.)
 * at the start of each request to provide user and request information.
 *
 * @param {Object} context - Security context
 * @param {Object} context.user - Current user (with roles, permissions, etc.)
 * @param {Object} context.session - Current session
 * @param {Object} context.request - Current request (with ip, protocol, etc.)
 *
 * @example
 * app.use((req, res, next) => {
 *   __jounce_set_security_context({
 *     user: req.user,
 *     session: req.session,
 *     request: { ip: req.ip, protocol: req.protocol }
 *   });
 *   next();
 * });
 */
export function __jounce_set_security_context(context) {
  __jounce_security_context = {
    user: context.user || null,
    session: context.session || null,
    request: context.request || null
  };
}

/**
 * Get the current security context
 *
 * @returns {Object} Current security context
 */
export function __jounce_get_security_context() {
  return __jounce_security_context;
}

/**
 * Authentication check middleware
 *
 * Verifies that the current user meets the specified authentication requirements.
 *
 * @param {Object} requirements - Authentication requirements
 * @param {string} [requirements.role] - Required role (e.g., "admin", "user")
 * @param {string} [requirements.permission] - Required permission (e.g., "users:delete")
 * @param {string[]} [requirements.roles] - Any of these roles (OR logic)
 * @param {string[]} [requirements.permissions] - Any of these permissions (OR logic)
 *
 * @returns {boolean} True if authenticated and authorized
 * @throws {Error} If not authenticated or not authorized
 *
 * @example
 * __jounce_auth_check({ role: "admin" })
 * __jounce_auth_check({ permission: "users:delete" })
 * __jounce_auth_check({ roles: ["admin", "moderator"] })
 */
export function __jounce_auth_check(requirements) {
  const { user } = __jounce_security_context;

  // Check if user is authenticated
  if (!user) {
    throw new Error("Authentication required");
  }

  // If no specific requirements, just being authenticated is enough
  if (!requirements || Object.keys(requirements).length === 0) {
    return true;
  }

  // Check single role requirement
  if (requirements.role) {
    if (!user.roles || !Array.isArray(user.roles)) {
      throw new Error(`Unauthorized: missing role information`);
    }
    if (!user.roles.includes(requirements.role)) {
      throw new Error(`Unauthorized: requires role '${requirements.role}'`);
    }
  }

  // Check single permission requirement
  if (requirements.permission) {
    if (!user.permissions || !Array.isArray(user.permissions)) {
      throw new Error(`Unauthorized: missing permission information`);
    }
    if (!user.permissions.includes(requirements.permission)) {
      throw new Error(`Unauthorized: requires permission '${requirements.permission}'`);
    }
  }

  // Check multiple roles requirement (OR logic - any role matches)
  if (requirements.roles && Array.isArray(requirements.roles)) {
    if (!user.roles || !Array.isArray(user.roles)) {
      throw new Error(`Unauthorized: missing role information`);
    }
    const hasRole = requirements.roles.some(role => user.roles.includes(role));
    if (!hasRole) {
      throw new Error(`Unauthorized: requires one of roles: ${requirements.roles.join(', ')}`);
    }
  }

  // Check multiple permissions requirement (OR logic - any permission matches)
  if (requirements.permissions && Array.isArray(requirements.permissions)) {
    if (!user.permissions || !Array.isArray(user.permissions)) {
      throw new Error(`Unauthorized: missing permission information`);
    }
    const hasPermission = requirements.permissions.some(perm => user.permissions.includes(perm));
    if (!hasPermission) {
      throw new Error(`Unauthorized: requires one of permissions: ${requirements.permissions.join(', ')}`);
    }
  }

  return true;
}

/**
 * Input validation middleware
 *
 * Validates data against a schema. Throws an error if validation fails.
 *
 * Schema format:
 * {
 *   fieldName: {
 *     type: "string" | "number" | "boolean" | "array" | "object",
 *     required: true | false,
 *     minLength: number (for strings),
 *     maxLength: number (for strings),
 *     pattern: string (regex pattern for strings),
 *     min: number (for numbers),
 *     max: number (for numbers),
 *     enum: array (allowed values),
 *     fields: schema (nested schema for objects),
 *     items: schema (schema for array items)
 *   }
 * }
 *
 * @param {Object} schema - Validation schema
 * @param {Object} data - Data to validate
 * @returns {boolean} True if valid
 * @throws {Error} If validation fails
 *
 * @example
 * const UserSchema = {
 *   username: { type: "string", required: true, minLength: 3, maxLength: 20 },
 *   email: { type: "string", required: true, pattern: "^[^@]+@[^@]+\\.[^@]+$" },
 *   age: { type: "number", min: 0, max: 150 }
 * };
 * __jounce_validate(UserSchema, { username: "john", email: "john@example.com", age: 30 });
 */
export function __jounce_validate(schema, data) {
  if (typeof schema !== 'object' || schema === null) {
    throw new Error('Schema must be an object');
  }

  if (typeof data !== 'object' || data === null) {
    throw new Error('Data must be an object');
  }

  // Validate each field in the schema
  for (const [fieldName, rules] of Object.entries(schema)) {
    const value = data[fieldName];

    // Check required fields
    if (rules.required && (value === undefined || value === null)) {
      throw new Error(`Validation error: Missing required field '${fieldName}'`);
    }

    // Skip validation if field is optional and not provided
    if (value === undefined || value === null) {
      continue;
    }

    // Check type
    if (rules.type) {
      const actualType = Array.isArray(value) ? 'array' : typeof value;
      if (actualType !== rules.type) {
        throw new Error(`Validation error: Field '${fieldName}' must be of type ${rules.type}, got ${actualType}`);
      }
    }

    // String validations
    if (typeof value === 'string') {
      if (rules.minLength !== undefined && value.length < rules.minLength) {
        throw new Error(`Validation error: Field '${fieldName}' must be at least ${rules.minLength} characters`);
      }
      if (rules.maxLength !== undefined && value.length > rules.maxLength) {
        throw new Error(`Validation error: Field '${fieldName}' must be at most ${rules.maxLength} characters`);
      }
      if (rules.pattern) {
        const regex = new RegExp(rules.pattern);
        if (!regex.test(value)) {
          throw new Error(`Validation error: Field '${fieldName}' does not match required pattern`);
        }
      }
    }

    // Number validations
    if (typeof value === 'number') {
      if (rules.min !== undefined && value < rules.min) {
        throw new Error(`Validation error: Field '${fieldName}' must be at least ${rules.min}`);
      }
      if (rules.max !== undefined && value > rules.max) {
        throw new Error(`Validation error: Field '${fieldName}' must be at most ${rules.max}`);
      }
    }

    // Enum validation
    if (rules.enum && Array.isArray(rules.enum)) {
      if (!rules.enum.includes(value)) {
        throw new Error(`Validation error: Field '${fieldName}' must be one of: ${rules.enum.join(', ')}`);
      }
    }

    // Nested object validation
    if (rules.type === 'object' && rules.fields) {
      __jounce_validate(rules.fields, value);
    }

    // Array item validation
    if (rules.type === 'array' && rules.items) {
      if (!Array.isArray(value)) {
        throw new Error(`Validation error: Field '${fieldName}' must be an array`);
      }
      for (let i = 0; i < value.length; i++) {
        try {
          // For object items, validate against schema
          if (typeof rules.items === 'object' && !rules.items.type) {
            __jounce_validate(rules.items, value[i]);
          } else {
            // For primitive items, validate type
            const itemType = Array.isArray(value[i]) ? 'array' : typeof value[i];
            if (itemType !== rules.items.type) {
              throw new Error(`must be of type ${rules.items.type}`);
            }
          }
        } catch (err) {
          throw new Error(`Validation error: Field '${fieldName}[${i}]' ${err.message}`);
        }
      }
    }
  }

  return true;
}

/**
 * Rate limiting middleware
 *
 * Tracks request counts and enforces rate limits.
 * Uses in-memory storage (suitable for single-instance apps).
 * For distributed systems, use Redis or similar.
 *
 * @param {Object} limits - Rate limit configuration
 * @param {number} limits.max - Maximum requests allowed
 * @param {number} limits.window - Time window in milliseconds
 * @param {string} [limits.key] - Custom key for rate limiting (defaults to user.id or request.ip)
 *
 * @returns {boolean} True if within rate limit
 * @throws {Error} If rate limit exceeded
 *
 * @example
 * __jounce_ratelimit({ max: 10, window: 60000 }) // 10 requests per minute
 * __jounce_ratelimit({ max: 100, window: 3600000 }) // 100 requests per hour
 */

// In-memory rate limit store: Map<key, Array<timestamp>>
const __rateLimitStore = new Map();

export function __jounce_ratelimit(limits) {
  const { user, request } = __jounce_security_context;

  // Determine the key for rate limiting
  let key;
  if (limits.key) {
    key = limits.key;
  } else if (user && user.id) {
    key = `user:${user.id}`;
  } else if (request && request.ip) {
    key = `ip:${request.ip}`;
  } else {
    key = 'anonymous';
  }

  const now = Date.now();
  const windowMs = limits.window || 60000; // Default 1 minute
  const maxRequests = limits.max || 100; // Default 100 requests

  // Get or create request history for this key
  if (!__rateLimitStore.has(key)) {
    __rateLimitStore.set(key, []);
  }

  const requests = __rateLimitStore.get(key);

  // Remove requests outside the current window
  const recentRequests = requests.filter(timestamp => now - timestamp < windowMs);

  // Check if rate limit exceeded
  if (recentRequests.length >= maxRequests) {
    const oldestRequest = Math.min(...recentRequests);
    const retryAfter = Math.ceil((oldestRequest + windowMs - now) / 1000);
    throw new Error(`Rate limit exceeded: max ${maxRequests} requests per ${windowMs}ms. Retry after ${retryAfter} seconds`);
  }

  // Add current request timestamp
  recentRequests.push(now);
  __rateLimitStore.set(key, recentRequests);

  return true;
}

/**
 * Clear rate limit data for a specific key or all keys
 * Useful for testing or manual resets
 *
 * @param {string} [key] - Specific key to clear, or undefined to clear all
 */
export function __jounce_ratelimit_clear(key) {
  if (key) {
    __rateLimitStore.delete(key);
  } else {
    __rateLimitStore.clear();
  }
}

/**
 * Input sanitization middleware
 *
 * Sanitizes data to prevent XSS attacks by escaping HTML special characters.
 * Also handles nested objects and arrays.
 *
 * @param {*} data - Data to sanitize (string, object, array)
 * @returns {*} Sanitized data
 *
 * @example
 * __jounce_sanitize("<script>alert('xss')</script>")
 * // Returns: "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
 *
 * __jounce_sanitize({ name: "<b>John</b>", age: 30 })
 * // Returns: { name: "&lt;b&gt;John&lt;/b&gt;", age: 30 }
 */
export function __jounce_sanitize(data) {
  // Handle null/undefined
  if (data === null || data === undefined) {
    return data;
  }

  // Sanitize strings - escape HTML special characters
  if (typeof data === 'string') {
    return data
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#x27;')
      .replace(/\//g, '&#x2F;');
  }

  // Sanitize arrays - recursively sanitize each element
  if (Array.isArray(data)) {
    return data.map(item => __jounce_sanitize(item));
  }

  // Sanitize objects - recursively sanitize each value
  if (typeof data === 'object') {
    const sanitized = {};
    for (const [key, value] of Object.entries(data)) {
      sanitized[key] = __jounce_sanitize(value);
    }
    return sanitized;
  }

  // Return primitives (numbers, booleans) unchanged
  return data;
}

/**
 * HTTPS enforcement check
 *
 * Verifies that the current request is using HTTPS protocol.
 *
 * @returns {boolean} True if HTTPS
 * @throws {Error} If not using HTTPS
 *
 * @example
 * __jounce_require_https()
 */
export function __jounce_require_https() {
  const { request } = __jounce_security_context;

  if (!request) {
    throw new Error("Security error: No request context available");
  }

  if (request.protocol !== 'https') {
    throw new Error("Security error: This endpoint requires HTTPS");
  }

  return true;
}
