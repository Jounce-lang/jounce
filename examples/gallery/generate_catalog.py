#!/usr/bin/env python3
"""
Generate catalog of Jounce example apps
Scans examples/apps/ and creates data/apps.json
"""

import json
import os
import re
from pathlib import Path
from collections import defaultdict

def extract_metadata(app_path):
    """Extract metadata from an app's main.jnc file"""
    main_file = app_path / "main.jnc"
    
    if not main_file.exists():
        return None
    
    content = main_file.read_text()
    lines = content.split('\n')
    
    # Extract metadata from comments at top of file
    metadata = {
        'id': app_path.name,
        'name': format_name(app_path.name),
        'description': '',
        'category': 'Uncategorized',
        'tags': [],
        'difficulty': 'intermediate',
        'features': [],
        'lines': len(lines),
    }
    
    # Parse comments for metadata
    for line in lines[:20]:  # Only check first 20 lines
        line = line.strip()
        if line.startswith('//'):
            comment = line[2:].strip()
            
            # Try to extract description (first comment line)
            if not metadata['description'] and comment and not any(x in comment.lower() for x in ['todo', 'fixme', '@']):
                metadata['description'] = comment
    
    # Auto-categorize based on app name/content
    name_lower = app_path.name.lower()
    content_lower = content.lower()
    
    if any(x in name_lower for x in ['counter', 'hello', 'button']):
        metadata['category'] = 'Basics'
        metadata['difficulty'] = 'beginner'
    elif any(x in name_lower for x in ['form', 'input', 'validation']):
        metadata['category'] = 'Forms'
    elif any(x in name_lower for x in ['todo', 'list', 'crud']):
        metadata['category'] = 'Real-World'
    elif any(x in name_lower for x in ['game', 'quiz', 'timer', 'stopwatch']):
        metadata['category'] = 'Interactivity'
    elif any(x in name_lower for x in ['server', 'database', 'websocket']):
        metadata['category'] = 'Advanced'
        metadata['difficulty'] = 'advanced'
    elif any(x in name_lower for x in ['calculator', 'converter', 'bmi']):
        metadata['category'] = 'Utilities'
    elif any(x in name_lower for x in ['style', 'css', 'theme']):
        metadata['category'] = 'Styling'
    
    # Extract features from code
    if 'signal(' in content:
        metadata['features'].append('Reactive Signals')
        metadata['tags'].append('signals')
    if 'computed(' in content:
        metadata['features'].append('Computed Values')
        metadata['tags'].append('computed')
    if 'effect(' in content:
        metadata['features'].append('Effects')
        metadata['tags'].append('effects')
    if '@server' in content:
        metadata['features'].append('Server Functions')
        metadata['tags'].append('server')
    if 'style {' in content or 'theme {' in content:
        metadata['features'].append('Styled Components')
        metadata['tags'].append('styling')
    if 'onclick=' in content_lower or 'oninput=' in content_lower:
        metadata['features'].append('Event Handling')
        metadata['tags'].append('events')
    if '.map(' in content or '.filter(' in content:
        metadata['features'].append('Array Operations')
        metadata['tags'].append('arrays')
    if 'component ' in content_lower:
        metadata['tags'].append('components')
    
    # Add category as tag
    metadata['tags'].append(metadata['category'].lower())
    
    return metadata

def format_name(dir_name):
    """Convert directory name to human-readable title"""
    # Remove number prefix
    name = re.sub(r'^\d+-', '', dir_name)
    # Replace hyphens with spaces
    name = name.replace('-', ' ')
    # Capitalize words
    name = ' '.join(word.capitalize() for word in name.split())
    return name

def main():
    apps_dir = Path(__file__).parent.parent / 'apps'
    
    if not apps_dir.exists():
        print(f"Error: {apps_dir} not found")
        return
    
    apps = []
    
    # Scan all app directories
    for app_dir in sorted(apps_dir.iterdir()):
        if not app_dir.is_dir():
            continue
        
        if app_dir.name.startswith('.') or app_dir.name == 'node_modules':
            continue
        
        metadata = extract_metadata(app_dir)
        if metadata:
            apps.append(metadata)
            print(f"✓ {metadata['id']:35} | {metadata['category']:15} | {metadata['name']}")
    
    # Generate catalog
    catalog = {
        'version': '1.0.0',
        'generated': '2025-10-31',
        'total': len(apps),
        'categories': list(set(app['category'] for app in apps)),
        'apps': apps
    }
    
    # Write to JSON file
    output_file = Path(__file__).parent / 'data' / 'apps.json'
    output_file.parent.mkdir(exist_ok=True)
    
    with open(output_file, 'w') as f:
        json.dump(catalog, f, indent=2)
    
    print(f"\n✨ Generated catalog: {output_file}")
    print(f"📊 Total apps: {len(apps)}")
    print(f"📂 Categories: {', '.join(sorted(catalog['categories']))}")
    
    # Print stats by category
    print("\n📈 Apps by category:")
    by_category = defaultdict(int)
    for app in apps:
        by_category[app['category']] += 1
    
    for category in sorted(by_category.keys()):
        print(f"   {category:15} {by_category[category]:2} apps")

if __name__ == '__main__':
    main()
