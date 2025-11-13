# PTCG Core Engine

一个灵活且可扩展的宝可梦集换式卡牌游戏 (Pokemon Trading Card Game) 核心引擎，使用 Rust 编写。

## 特性

- 🎯 **模块化设计**: 只使用您需要的功能
- 📊 **数据导入**: 支持多种数据格式 (JSON, CSV, 数据库)
- 🔧 **规则扩展**: 轻松添加新的卡牌效果和规则
- 🌐 **网络就绪**: 内置多人游戏支持
- ⚡ **高性能**: 零成本抽象和编译时优化
- 🛡️ **内存安全**: 利用 Rust 的所有权系统
- 🔄 **完整的游戏流程支持**: 包括洗牌、穆勒规则重抽、能量附加、攻击等核心游戏机制

## 快速开始

### 添加依赖

```toml
[dependencies]
ptcg-core = "0.1.0"
```

### 基础使用

```rust
use ptcg_core::{Game, Player, Deck};

fn main() {
    // 创建新游戏
    let mut game = Game::new();
    
    // 添加玩家
    let player1 = Player::new("玩家1".to_string());
    let player2 = Player::new("玩家2".to_string());
    
    game.add_player(player1).unwrap();
    game.add_player(player2).unwrap();
    
    // 开始游戏 (需要先设置牌组)
    // game.start().unwrap();
}
```

## 架构概览

### 核心模块

- **`core`**: 基础数据结构 (Card, Player, Game, Deck)
- **`rules`**: 规则引擎系统
- **`events`**: 事件系统用于游戏状态跟踪
- **`effects`**: 卡牌效果系统
- **`data`**: 数据导入/导出功能

### 功能特性

#### 默认特性
- `json`: JSON 数据导入/导出

#### 可选特性
- `csv_import`: CSV 数据导入
- `database`: 数据库支持 (.pdb 文件等)
- `async`: 异步网络支持
- `full`: 启用所有特性

## 使用示例

### 创建卡牌

```rust
use ptcg_core::{Card, CardType, EnergyType, EvolutionStage, CardRarity};

let pikachu = Card::new(
    "皮卡丘".to_string(),
    CardType::Pokemon {
        species: "皮卡丘".to_string(),
        hp: 60,
        retreat_cost: 1,
        weakness: Some(EnergyType::Fighting),
        resistance: None,
        stage: EvolutionStage::Basic,
        evolves_from: None,
    },
    "基础包".to_string(),
    "025".to_string(),
    CardRarity::Common,
);
```

### 构建牌组

```rust
use ptcg_core::{Deck, Card};

let mut deck = Deck::new("我的牌组".to_string(), "Standard".to_string());
deck.add_card(pikachu.id, 4); // 添加4张皮卡丘

// 验证牌组
let card_db = std::collections::HashMap::new();
match deck.validate(&card_db) {
    Ok(()) => println!("牌组合法！"),
    Err(errors) => println!("牌组错误: {:?}", errors),
}
```

### 使用规则引擎

```rust
use ptcg_core::{StandardRules, GameAction};

let engine = StandardRules::create_engine();
let action = GameAction::DrawCard { player_id: player1.id };

let violations = engine.validate_action(&game, &action);
if violations.is_empty() {
    println!("动作合法！");
}
```

### 执行游戏动作

```rust
use ptcg_core::{GameAction, RuleEngine};

// 创建规则引擎
let rule_engine = RuleEngine::new();

// 创建抽卡动作
let draw_action = GameAction::DrawCard { player_id: player1.id };

// 执行动作
match game.execute_action(&rule_engine, &draw_action) {
    Ok(()) => println!("抽卡成功！"),
    Err(violations) => println!("动作违反规则: {:?}", violations),
}
```

## 开发

### 环境要求

- Rust 1.70+ 
- Cargo

### 构建

```bash
# 标准构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 静态分析
cargo clippy
```

### 启用特定功能

```bash
# 启用所有功能
cargo build --features full

# 启用特定功能
cargo build --features "database,async"
```

## 贡献

欢迎贡献！请查看 [COMMIT_CONVENTION.md](COMMIT_CONVENTION.md) 了解提交规范。

### 开发流程

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交变更 (`git commit -m 'feat: add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目使用 MIT 或 Apache-2.0 双重许可证。

## 路线图

- [x] 基础数据结构
- [x] 规则引擎系统
- [x] 事件系统
- [x] 效果系统
- [x] 数据导入框架
- [x] 完整的游戏动作执行系统
- [x] 穆勒规则重抽流程优化
- [ ] 完整的标准规则实现
- [ ] 网络多人游戏支持
- [ ] AI 对手系统
- [ ] 图形用户界面
- [ ] 更多卡牌效果
- [ ] 性能优化

## 相关项目

这个核心引擎被设计为可以轻松集成到各种上层应用中：

- 桌面游戏客户端
- 网页游戏平台
- 移动应用
- 服务器端游戏逻辑
- 卡牌模拟器工具

## 社区

- 📧 问题反馈: [GitHub Issues](https://github.com/your-org/ptcg-core/issues)
- 💬 讨论: [GitHub Discussions](https://github.com/your-org/ptcg-core/discussions)