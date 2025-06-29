#!/usr/bin/env node
// Generate filesystem metadata from data directory

const fs = require('fs');
const path = require('path');

// Parse markdown frontmatter metadata
function parseMarkdownMetadata(content) {
  const metadata = {};
  
  // Check if content starts with frontmatter delimiter
  if (!content.startsWith('--\n') && !content.startsWith('---\n')) {
    return metadata;
  }
  
  const delimiter = content.startsWith('---\n') ? '---' : '--';
  const lines = content.split('\n');
  let inFrontmatter = false;
  let endIndex = -1;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    
    if (i === 0 && line === delimiter) {
      inFrontmatter = true;
      continue;
    }
    
    if (inFrontmatter && line === delimiter) {
      endIndex = i;
      break;
    }
    
    if (inFrontmatter && line.includes(':')) {
      const colonIndex = line.indexOf(':');
      const key = line.substring(0, colonIndex).trim();
      const value = line.substring(colonIndex + 1).trim();
      
      if (key && value) {
        // Handle special parsing for tags (comma-separated)
        if (key === 'tag' || key === 'tags') {
          metadata.tags = value.split(',').map(tag => tag.trim()).filter(tag => tag);
        } else {
          metadata[key] = value;
        }
      }
    }
  }
  
  return metadata;
}

// Read and parse markdown file metadata
function getMarkdownMetadata(filePath) {
  try {
    const content = fs.readFileSync(filePath, 'utf-8');
    return parseMarkdownMetadata(content);
  } catch (error) {
    console.warn(`Error reading markdown file ${filePath}:`, error.message);
    return {};
  }
}

function generateMetadata(dirPath, relativePath = '') {
  const metadata = {
    type: 'directory',
    name: path.basename(dirPath) || 'root',
    path: relativePath,
    children: {}
  };

  try {
    const items = fs.readdirSync(dirPath);
    
    for (const item of items) {
      const itemPath = path.join(dirPath, item);
      const itemRelativePath = relativePath ? `${relativePath}/${item}` : item;
      const stats = fs.statSync(itemPath);
      
      if (stats.isDirectory()) {
        metadata.children[item] = generateMetadata(itemPath, itemRelativePath);
      } else {
        const fileMetadata = {
          type: 'file',
          name: item,
          path: itemRelativePath,
          size: stats.size,
          modified: stats.mtime.toISOString(),
          extension: path.extname(item).toLowerCase().slice(1) || null
        };
        
        // If it's a markdown file, extract its metadata
        if (path.extname(item).toLowerCase() === '.md') {
          const markdownMetadata = getMarkdownMetadata(itemPath);
          if (markdownMetadata.title) fileMetadata.title = markdownMetadata.title;
          if (markdownMetadata.description) fileMetadata.description = markdownMetadata.description;
          if (markdownMetadata.tags) fileMetadata.tags = markdownMetadata.tags;
        }
        
        metadata.children[item] = fileMetadata;
      }
    }
  } catch (error) {
    console.error(`Error reading directory ${dirPath}:`, error.message);
  }
  
  return metadata;
}

function main() {
  const dataDir = path.join(__dirname, 'data');
  const outputFile = path.join(__dirname, 'src', 'filesystem_metadata.json');
  
  if (!fs.existsSync(dataDir)) {
    console.error('Data directory does not exist!');
    process.exit(1);
  }
  
  console.log('Generating filesystem metadata...');
  const metadata = generateMetadata(dataDir);
  
  // Ensure src directory exists
  const srcDir = path.dirname(outputFile);
  if (!fs.existsSync(srcDir)) {
    fs.mkdirSync(srcDir, { recursive: true });
  }
  
  // Write metadata to JSON file
  fs.writeFileSync(outputFile, JSON.stringify(metadata, null, 2));
  
  console.log(`Metadata generated successfully: ${outputFile}`);
  console.log(`Found ${Object.keys(metadata.children).length} items in root directory`);
}

if (require.main === module) {
  main();
}