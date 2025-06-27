#!/usr/bin/env node
// Generate filesystem metadata from data directory

const fs = require('fs');
const path = require('path');

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
        metadata.children[item] = {
          type: 'file',
          name: item,
          path: itemRelativePath,
          size: stats.size,
          modified: stats.mtime.toISOString(),
          extension: path.extname(item).toLowerCase().slice(1) || null
        };
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