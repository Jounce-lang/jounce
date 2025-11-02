# Jounce Tutorial Certificate Template

**Implementation Guide for Tutorial Platform**

---

## Certificate Design

### Certificate SVG Template

Save as `certificate-template.svg`:

```svg
<svg width="1200" height="800" xmlns="http://www.w3.org/2000/svg">
  <!-- Background -->
  <rect width="1200" height="800" fill="#f8fafc"/>

  <!-- Border -->
  <rect x="40" y="40" width="1120" height="720" fill="none" stroke="#3B82F6" stroke-width="8" rx="12"/>
  <rect x="60" y="60" width="1080" height="680" fill="none" stroke="#60A5FA" stroke-width="2" rx="8"/>

  <!-- Header -->
  <text x="600" y="140" text-anchor="middle" font-family="Arial, sans-serif" font-size="48" font-weight="bold" fill="#1e293b">
    Certificate of Completion
  </text>

  <!-- Subheading -->
  <text x="600" y="190" text-anchor="middle" font-family="Arial, sans-serif" font-size="24" fill="#64748b">
    Jounce Interactive Tutorial
  </text>

  <!-- Divider -->
  <line x1="300" y1="220" x2="900" y2="220" stroke="#cbd5e1" stroke-width="2"/>

  <!-- "This certifies that" -->
  <text x="600" y="280" text-anchor="middle" font-family="Arial, sans-serif" font-size="20" fill="#64748b">
    This certifies that
  </text>

  <!-- Student Name (Dynamic) -->
  <text id="studentName" x="600" y="350" text-anchor="middle" font-family="Georgia, serif" font-size="52" font-weight="bold" fill="#0f172a">
    [STUDENT_NAME]
  </text>

  <!-- Achievement Text -->
  <text x="600" y="420" text-anchor="middle" font-family="Arial, sans-serif" font-size="20" fill="#64748b">
    has successfully completed all 10 lessons of the
  </text>

  <text x="600" y="455" text-anchor="middle" font-family="Arial, sans-serif" font-size="24" font-weight="bold" fill="#3B82F6">
    Jounce Interactive Tutorial
  </text>

  <!-- Skills Learned -->
  <text x="600" y="510" text-anchor="middle" font-family="Arial, sans-serif" font-size="16" fill="#64748b">
    Demonstrating proficiency in:
  </text>

  <text x="600" y="540" text-anchor="middle" font-family="Arial, sans-serif" font-size="16" fill="#475569">
    Reactive Programming • JSX • Components • State Management • Styling
  </text>

  <!-- Date (Dynamic) -->
  <text x="300" y="640" text-anchor="middle" font-family="Arial, sans-serif" font-size="18" fill="#64748b">
    Date Completed:
  </text>
  <text id="completionDate" x="300" y="670" text-anchor="middle" font-family="Arial, sans-serif" font-size="20" font-weight="bold" fill="#0f172a">
    [COMPLETION_DATE]
  </text>

  <!-- Certificate ID (Dynamic) -->
  <text x="900" y="640" text-anchor="middle" font-family="Arial, sans-serif" font-size="18" fill="#64748b">
    Certificate ID:
  </text>
  <text id="certificateId" x="900" y="670" text-anchor="middle" font-family="Arial, sans-serif" font-size="20" font-weight="bold" fill="#0f172a">
    [CERTIFICATE_ID]
  </text>

  <!-- Signature Line -->
  <line x1="420" y1="710" x2="780" y2="710" stroke="#cbd5e1" stroke-width="2"/>
  <text x="600" y="735" text-anchor="middle" font-family="Arial, sans-serif" font-size="16" font-style="italic" fill="#64748b">
    The Jounce Team
  </text>

  <!-- Logo/Badge (Optional) -->
  <circle cx="100" cy="100" r="50" fill="#3B82F6" opacity="0.1"/>
  <text x="100" y="115" text-anchor="middle" font-family="Arial, sans-serif" font-size="36" font-weight="bold" fill="#3B82F6">
    J
  </text>
</svg>
```

---

## Certificate Generation Logic

### JavaScript Implementation

```javascript
/**
 * Generate a certificate for a student who completed the tutorial
 * @param {string} studentName - Student's name
 * @param {Date} completionDate - Date tutorial was completed
 * @returns {object} Certificate data
 */
function generateCertificate(studentName, completionDate) {
  // Generate unique certificate ID
  const certificateId = generateCertificateId(studentName, completionDate);

  // Format date
  const formattedDate = formatDate(completionDate);

  // Create certificate data
  const certificate = {
    studentName: studentName,
    completionDate: formattedDate,
    certificateId: certificateId,
    skills: [
      "Reactive Programming",
      "JSX Syntax",
      "Component Architecture",
      "State Management",
      "Styling & UI Design"
    ],
    lessonsCompleted: 10,
    verificationUrl: `https://verify.jounce.dev/certificate/${certificateId}`
  };

  return certificate;
}

/**
 * Generate unique certificate ID
 * @param {string} name - Student name
 * @param {Date} date - Completion date
 * @returns {string} Certificate ID
 */
function generateCertificateId(name, date) {
  const timestamp = date.getTime();
  const hash = simpleHash(name + timestamp);
  const year = date.getFullYear();
  return `JOUNCE-${year}-${hash.toUpperCase()}`;
}

/**
 * Simple hash function for certificate ID
 * @param {string} str - String to hash
 * @returns {string} Hash (8 characters)
 */
function simpleHash(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash).toString(36).substring(0, 8);
}

/**
 * Format date for certificate
 * @param {Date} date - Date to format
 * @returns {string} Formatted date
 */
function formatDate(date) {
  const options = { year: 'numeric', month: 'long', day: 'numeric' };
  return date.toLocaleDateString('en-US', options);
}

/**
 * Render certificate SVG with student data
 * @param {object} certificateData - Certificate data from generateCertificate()
 * @returns {string} SVG string with data filled in
 */
function renderCertificateSVG(certificateData) {
  // Load template SVG (from file or embedded)
  let svg = loadCertificateTemplate();

  // Replace placeholders
  svg = svg.replace('[STUDENT_NAME]', certificateData.studentName);
  svg = svg.replace('[COMPLETION_DATE]', certificateData.completionDate);
  svg = svg.replace('[CERTIFICATE_ID]', certificateData.certificateId);

  return svg;
}

// Example Usage:
const studentName = "Jane Doe";
const completionDate = new Date();
const certificate = generateCertificate(studentName, completionDate);
const svg = renderCertificateSVG(certificate);

// Save or display the certificate
console.log(certificate);
// Output:
// {
//   studentName: "Jane Doe",
//   completionDate: "November 1, 2025",
//   certificateId: "JOUNCE-2025-A3F9B2E1",
//   skills: [...],
//   lessonsCompleted: 10,
//   verificationUrl: "https://verify.jounce.dev/certificate/JOUNCE-2025-A3F9B2E1"
// }
```

---

## Download & Share Features

### Download as PNG

```javascript
/**
 * Convert SVG to PNG for download
 * @param {string} svgString - SVG markup
 * @param {function} callback - Callback with PNG data URL
 */
function svgToPng(svgString, callback) {
  const canvas = document.createElement('canvas');
  canvas.width = 1200;
  canvas.height = 800;
  const ctx = canvas.getContext('2d');

  const img = new Image();
  img.onload = function() {
    ctx.drawImage(img, 0, 0);
    const pngDataUrl = canvas.toDataURL('image/png');
    callback(pngDataUrl);
  };

  const svgBlob = new Blob([svgString], { type: 'image/svg+xml;charset=utf-8' });
  const url = URL.createObjectURL(svgBlob);
  img.src = url;
}

// Usage:
svgToPng(svg, (pngDataUrl) => {
  const link = document.createElement('a');
  link.download = 'jounce-certificate.png';
  link.href = pngDataUrl;
  link.click();
});
```

### Share on Social Media

```javascript
/**
 * Generate social share links
 * @param {object} certificate - Certificate data
 * @returns {object} Share URLs
 */
function generateShareLinks(certificate) {
  const message = `I just completed the Jounce Interactive Tutorial! 🎉 #JounceJS #WebDev`;
  const url = certificate.verificationUrl;

  return {
    twitter: `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}&url=${encodeURIComponent(url)}`,
    linkedin: `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`,
    facebook: `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(url)}`
  };
}
```

---

## Certificate Verification System

### Verification Page

```markdown
# Certificate Verification

Enter a certificate ID to verify its authenticity.

**Format**: JOUNCE-YYYY-XXXXXXXX

[Input Field]
[Verify Button]

---

**Valid Certificate Example**:
- ✅ Certificate ID: JOUNCE-2025-A3F9B2E1
- ✅ Student: Jane Doe
- ✅ Completed: November 1, 2025
- ✅ Skills: Reactive Programming, JSX, Components, State Management, Styling
```

### Verification API

```javascript
/**
 * Verify certificate by ID
 * @param {string} certificateId - Certificate ID to verify
 * @returns {object|null} Certificate data if valid, null if invalid
 */
async function verifyCertificate(certificateId) {
  // Check format
  if (!/^JOUNCE-\d{4}-[A-Z0-9]{8}$/.test(certificateId)) {
    return null;
  }

  // Query database (pseudo-code)
  const certificate = await db.certificates.findOne({ id: certificateId });

  if (!certificate) {
    return null;
  }

  return {
    valid: true,
    studentName: certificate.studentName,
    completionDate: certificate.completionDate,
    skills: certificate.skills
  };
}
```

---

## UI/UX for Certificate Page

### Certificate Display Page

```html
<!-- After completing all 10 lessons -->
<div class="certificate-celebration">
  <h1>🎉 Congratulations!</h1>
  <p>You've completed all 10 lessons!</p>

  <!-- Certificate Preview -->
  <div class="certificate-preview">
    <svg>...</svg>
  </div>

  <!-- Actions -->
  <div class="certificate-actions">
    <button id="download-png">Download PNG</button>
    <button id="download-svg">Download SVG</button>
    <button id="download-pdf">Download PDF</button>
  </div>

  <!-- Share -->
  <div class="share-section">
    <h3>Share Your Achievement</h3>
    <button id="share-twitter">Share on Twitter</button>
    <button id="share-linkedin">Share on LinkedIn</button>
    <button id="share-facebook">Share on Facebook</button>
  </div>

  <!-- Verification -->
  <div class="verification-section">
    <p>Certificate ID: <code>JOUNCE-2025-A3F9B2E1</code></p>
    <p>Verify at: <a href="https://verify.jounce.dev">verify.jounce.dev</a></p>
  </div>

  <!-- Next Steps -->
  <div class="next-steps">
    <h3>What's Next?</h3>
    <ul>
      <li>Build your own app with starter templates</li>
      <li>Join the Discord community</li>
      <li>Read the advanced docs</li>
      <li>Deploy to production</li>
    </ul>
  </div>
</div>
```

---

## Database Schema

### Certificate Storage

```javascript
// MongoDB schema example
{
  _id: ObjectId("..."),
  certificateId: "JOUNCE-2025-A3F9B2E1",
  studentName: "Jane Doe",
  studentEmail: "jane@example.com", // Optional
  completionDate: ISODate("2025-11-01T14:30:00Z"),
  lessonsCompleted: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  timeSpent: 2847, // seconds
  createdAt: ISODate("2025-11-01T14:30:00Z"),
  verified: true
}
```

---

## Example Certificate Output

```
┌────────────────────────────────────────────────────────┐
│                                                        │
│           CERTIFICATE OF COMPLETION                    │
│             Jounce Interactive Tutorial                │
│                                                        │
│  ──────────────────────────────────────────────────   │
│                                                        │
│              This certifies that                       │
│                                                        │
│                  Jane Doe                              │
│                                                        │
│  has successfully completed all 10 lessons of the      │
│          Jounce Interactive Tutorial                   │
│                                                        │
│          Demonstrating proficiency in:                 │
│  Reactive Programming • JSX • Components •             │
│       State Management • Styling                       │
│                                                        │
│                                                        │
│  Date Completed:         Certificate ID:               │
│  November 1, 2025        JOUNCE-2025-A3F9B2E1          │
│                                                        │
│  ─────────────────────                                 │
│    The Jounce Team                                     │
│                                                        │
└────────────────────────────────────────────────────────┘
```

---

## Implementation Checklist

- [ ] Create SVG template
- [ ] Implement certificate generation logic
- [ ] Add download functionality (PNG, SVG, PDF)
- [ ] Build social sharing features
- [ ] Create verification page
- [ ] Set up database for certificate storage
- [ ] Add certificate display page
- [ ] Test on mobile devices
- [ ] Add print-friendly CSS
- [ ] Create email template for certificate delivery

---

**Last Updated**: November 1, 2025
**Version**: v1.0.0
