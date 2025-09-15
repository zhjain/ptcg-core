# Git Hooks 和配置

## Pre-commit Hook (可选)

创建 `.git/hooks/pre-commit` 文件来自动检查代码质量：

```bash
#!/bin/sh
# Pre-commit hook for PTCG Core

echo "🔍 运行代码检查..."

# 检查代码编译
if ! cargo check --quiet; then
    echo "❌ 编译失败，请修复后再提交"
    exit 1
fi

# 检查代码格式
if ! cargo fmt --check; then
    echo "❌ 代码格式不符合规范，运行 'cargo fmt' 修复"
    exit 1
fi

# 检查代码质量
if ! cargo clippy --quiet -- -D warnings; then
    echo "❌ Clippy 检查发现问题，请修复后再提交"
    exit 1
fi

echo "✅ 所有检查通过！"
```

## 提交模板

创建 `.gitmessage` 提交消息模板：

```
<type>(<scope>): <描述>

# 详细说明变更内容

# 相关问题: #issue
```

使用模板：
```bash
git config commit.template .gitmessage
```

## 常用 Git 别名

```bash
# 设置有用的别名
git config alias.st status
git config alias.co checkout
git config alias.br branch
git config alias.ci commit
git config alias.lg "log --oneline --graph --decorate"
```