--
title: Markdown 元数据功能演示
description: 展示如何在 markdown 文件中使用 frontmatter 元数据
tags: metadata, frontmatter, markdown, demo
--

# 📝 Markdown 元数据功能演示

这个文件演示了新的 markdown 元数据功能！

## 🎯 元数据格式

在 markdown 文件的开头，你可以添加元数据块：

```markdown
--
title: 文件标题
description: 文件描述
tags: tag1, tag2, tag3
--
```

## 📁 功能特点

### 1. **文件元数据**
- 每个 `.md` 文件可以有自己的 `title`、`description` 和 `tags`
- 支持 `--` 和 `---` 两种分隔符格式
- 标签支持逗号分隔的多个值

### 2. **文件夹元数据**
- 文件夹的元数据来自其目录下的 `index.md` 文件
- 当你访问文件夹时，会自动读取 `index.md` 的元数据
- 为文件夹提供标题、描述和标签信息

### 3. **自动生成**
- 元数据在构建时自动提取并保存到 `filesystem_metadata.json`
- 使用 `node generate_metadata.js` 命令重新生成
- Rust 代码可以直接访问这些元数据

## 🛠️ 技术实现

### JavaScript 解析器
```javascript
// 解析 frontmatter 元数据
function parseMarkdownMetadata(content) {
  // 检查是否以 -- 或 --- 开头
  // 解析 key: value 格式
  // 特殊处理 tags 字段（逗号分隔）
}
```

### Rust 数据结构
```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileSystemNode {
    // 基础字段...
    
    // 元数据字段
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
```

## 📊 使用示例

当前文件的元数据：
- **标题**: Markdown 元数据功能演示
- **描述**: 展示如何在 markdown 文件中使用 frontmatter 元数据
- **标签**: metadata, frontmatter, markdown, demo

## 🎨 样式支持

元数据不会影响文件的渲染显示，但会：
- 提供更好的文件组织
- 支持未来的搜索功能
- 增强文件系统的语义化

## 🚀 下一步

这个元数据系统为以下功能奠定了基础：
- 文件搜索和过滤
- 标签导航
- 智能文件推荐
- 内容分类展示

---

*这个演示文件本身就包含了完整的元数据示例！*