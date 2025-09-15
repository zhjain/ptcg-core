# 提交规范 (Commit Convention)

本项目遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范，并结合项目特点制定以下提交规范。

## 提交消息格式

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### 类型 (Type)

- **feat**: 新功能 (feature)
- **fix**: 修复错误 (bug fix)
- **docs**: 文档变更 (documentation)
- **style**: 代码格式变更 (formatting, missing semicolons, etc)
- **refactor**: 代码重构 (neither fixes a bug nor adds a feature)
- **perf**: 性能优化 (performance improvement)
- **test**: 添加或修改测试 (adding missing tests, refactoring tests)
- **chore**: 构建过程或辅助工具的变动 (maintain)
- **ci**: CI/CD 相关变更
- **build**: 构建系统或外部依赖变更

### 作用域 (Scope)

项目特定的作用域：

- **core**: 核心数据结构 (Card, Player, Game, Deck)
- **rules**: 规则引擎系统
- **events**: 事件系统
- **effects**: 效果系统
- **data**: 数据导入/导出
- **network**: 网络功能 (当实现时)
- **ui**: 用户界面 (当实现时)
- **deps**: 依赖管理
- **config**: 配置文件

### 描述 (Description)

- 使用动词原形开始
- 首字母小写
- 结尾不加句号
- 简洁明了地描述变更内容

### 示例

```bash
# 新功能
feat(core): add basic Pokemon card structure
feat(rules): implement turn order validation
feat(effects): add damage and healing effects

# 修复错误
fix(core): resolve deck shuffling algorithm
fix(rules): correct energy attachment validation

# 文档
docs: add API documentation for Game class
docs(readme): update installation instructions

# 重构
refactor(core): simplify player state management
refactor(effects): extract common effect patterns

# 测试
test(core): add unit tests for card creation
test(rules): add integration tests for rule engine

# 构建/依赖
build: add serde feature to uuid dependency
chore(deps): update rust edition to 2021
```

### Breaking Changes

如果是破坏性变更，在类型后添加 `!`：

```
feat(core)!: change Card API structure

BREAKING CHANGE: Card constructor now requires CardType parameter
```

### 多行提交示例

```
feat(core): implement comprehensive game state management

- Add game phases (BeginningOfTurn, Main, Attack, EndOfTurn)
- Implement turn management and player switching
- Add win condition checking
- Support for game history tracking

Closes #123
```

## 提交频率建议

- **小而频繁**: 每个逻辑单元一个提交
- **原子性**: 每个提交应该是一个完整的、可编译的变更
- **相关性**: 相关的变更应该在同一个提交中

## 分支命名规范

- **feature/**: 新功能分支 (`feature/card-effects`)
- **fix/**: 修复分支 (`fix/deck-validation`)
- **docs/**: 文档分支 (`docs/api-reference`)
- **refactor/**: 重构分支 (`refactor/player-state`)

## 提交前检查清单

- [ ] 代码能够编译通过 (`cargo check`)
- [ ] 测试通过 (`cargo test`)
- [ ] 代码格式正确 (`cargo fmt`)
- [ ] 没有明显的代码质量问题 (`cargo clippy`)
- [ ] 提交消息符合规范
- [ ] 相关文档已更新