# PTCG Core 项目结构说明

## 项目概述

PTCG Core 是一个使用 Rust 编写的宝可梦集换式卡牌游戏(Pokemon Trading Card Game)核心引擎。该项目采用模块化设计，提供了卡牌数据结构、游戏规则引擎、事件系统、效果系统和数据导入导出功能。

## 目录结构

```
ptcg-core/
├── examples/              # 使用示例
├── src/                   # 源代码目录
│   ├── core/              # 核心数据结构
│   │   ├── game/          # 游戏相关模块
│   │   │   ├── actions.rs # 游戏动作实现
│   │   │   ├── mod.rs     # 游戏模块声明
│   │   │   ├── setup.rs   # 游戏设置逻辑
│   │   │   ├── state.rs   # 游戏状态管理
│   │   │   └── turn.rs    # 回合处理逻辑
│   │   ├── card.rs        # 卡牌数据结构
│   │   ├── deck.rs        # 牌组管理
│   │   ├── mod.rs         # 核心模块声明
│   │   └── player.rs      # 玩家数据结构
│   ├── data/              # 数据导入/导出模块
│   │   ├── csv.rs         # CSV数据导入
│   │   ├── database.rs    # 数据库导入
│   │   ├── json.rs        # JSON数据导入/导出
│   │   └── mod.rs         # 数据模块声明
│   ├── effects.rs         # 卡牌效果系统
│   ├── events.rs          # 事件系统
│   ├── lib.rs             # 库入口和重新导出
│   ├── network.rs         # 网络功能(可选)
│   └── rules.rs           # 规则引擎系统
├── Cargo.toml             # 项目配置文件
├── README.md              # 项目说明文档
└── TODO.md                # 开发待办事项
```

## 各模块详细说明

### 1. 核心模块 (src/core/)

核心模块包含了游戏的基础数据结构和主要实体。

#### 1.1 卡牌模块 (src/core/card.rs)

包含所有与卡牌相关的数据结构和功能：
- `Card`: 主要的卡牌结构体，包含卡牌的所有属性
- `CardType`: 卡牌类型枚举（宝可梦、能量、训练家）
- `EnergyType`: 能量类型枚举
- `EvolutionStage`: 进化阶段枚举
- `TrainerType`: 训练家卡类型枚举
- `CardRarity`: 卡牌稀有度枚举
- `Attack`: 攻击信息结构体
- `Ability`: 能力信息结构体
- 相关的辅助方法和构造函数
- `get_usable_attacks`: 获取满足能量需求的攻击数组方法

#### 1.2 玩家模块 (src/core/player.rs)

管理玩家状态和操作：
- `Player`: 玩家结构体，包含玩家的所有状态信息
- `SpecialCondition`: 特殊状态条件枚举（中毒、麻痹等）
- `SpecialConditionInstance`: 特殊状态实例
- `CardLocation`: 卡牌位置枚举
- 玩家操作方法（抽卡、放置宝可梦、附加能量等）
- `shuffle_deck`: 洗牌方法
- `get_attached_energy_types`: 获取指定宝可梦的附加能量类型列表方法

#### 1.3 牌组模块 (src/core/deck.rs)

处理牌组构建和验证：
- `Deck`: 牌组结构体
- `DeckValidationError`: 牌组验证错误枚举
- 牌组操作方法（添加/移除卡牌、验证等）
- 牌组统计和导出功能

#### 1.4 游戏模块 (src/core/game/)

管理游戏状态和流程：

##### 1.4.1 游戏状态 (src/core/game/state.rs)
- `Game`: 主游戏结构体
- `GamePhase`: 游戏阶段枚举
- `GameState`: 游戏状态枚举
- `GameRules`: 游戏规则配置
- `GameEvent`: 游戏事件枚举

##### 1.4.2 游戏设置 (src/core/game/setup.rs)
- 玩家加入游戏
- 牌组设置
- 先后手决定
- 初始手牌分发
- 基础宝可梦检查
- 穆勒规则处理（优化后的重抽与基础宝可梦检查流程）
- `MulliganResult`: 穆勒规则重抽结果枚举
- `perform_mulligan_for_both_and_check_basic_pokemon`: 对双方玩家执行重抽并检查基础宝可梦状态
- `perform_mulligan_and_check_basic_pokemon`: 对指定玩家执行重抽并检查是否包含基础宝可梦
- `print_player_hand`: 打印玩家手牌，用于穆勒规则重抽时让对手查看
- `declare_and_perform_mulligan`: 宣告没有基础宝可梦并执行穆勒规则重抽流程
- 活跃宝可梦选择
- 备战区设置
- 奖赏卡放置

##### 1.4.3 回合处理 (src/core/game/turn.rs)
- 游戏开始
- 回合开始/结束
- 阶段推进
- 胜负条件检查

##### 1.4.4 游戏动作 (src/core/game/actions.rs)
- `shuffle_deck`: 洗牌功能
- `shuffle_both_decks`: 双人洗牌支持
- `execute_action`: 游戏动作执行功能（抽卡、附加能量、攻击、结束回合等）

### 2. 规则引擎模块 (src/rules.rs)

提供灵活的规则验证和执行系统：
- `Rule`: 规则特质，所有规则都需要实现此特质
- `RuleEngine`: 规则引擎，管理所有规则
- `GameAction`: 游戏动作枚举（更新后的动作类型）
- `RuleViolation`: 规则违反信息
- 标准规则实现（回合顺序、手牌限制、能量附加等）

### 3. 事件系统模块 (src/events.rs)

追踪游戏状态变化和通知：
- `Event`: 事件特质
- `EventHandler`: 事件处理器特质
- `EventBus`: 事件总线，管理事件分发
- `GameEvent`: 游戏事件枚举（新增洗牌事件等）
- `ConsoleEventHandler`: 控制台事件处理器（示例）

### 4. 效果系统模块 (src/effects.rs)

实现卡牌效果和触发器：
- `Effect`: 效果特质，所有效果都需要实现此特质
- `EffectManager`: 效果管理器
- `EffectContext`: 效果上下文信息
- `EffectTrigger`: 效果触发器枚举
- `EffectTarget`: 效果目标枚举
- 各种效果结果和错误类型

### 5. 数据模块 (src/data/)

处理不同格式的数据导入/导出：

#### 5.1 JSON支持 (src/data/json.rs)
- `JsonImporter`: JSON数据导入器
- `JsonExporter`: JSON数据导出器

#### 5.2 CSV支持 (src/data/csv.rs)
- `CsvImporter`: CSV数据导入器

#### 5.3 数据库支持 (src/data/database.rs)
- `DatabaseImporter`: 数据库导入器

#### 5.4 数据模块声明 (src/data/mod.rs)
- 通用导入/导出特质
- 错误类型定义
- 批量导入器

### 6. 网络模块 (src/network.rs)

提供网络功能支持（需要启用async特性）：
- 网络对战相关功能

### 7. 库入口 (src/lib.rs)

项目的主入口文件：
- 模块声明和重新导出
- 公共API接口
- 错误类型定义
- 库信息和版本管理

## 功能特性

### 默认特性
- `json`: JSON数据导入/导出支持

### 可选特性
- `csv_import`: CSV数据导入支持
- `database`: 数据库支持
- `async`: 异步网络支持
- `full`: 启用所有特性

## 开发指南

### 代码组织原则
1. 按功能模块化组织代码
2. 每个模块应有明确的职责
3. 公共接口通过lib.rs重新导出
4. 使用Rust的模块系统管理可见性

### 添加新功能的建议
1. 核心数据结构应放在src/core/相应模块中
2. 新的游戏规则应实现Rule特质并添加到规则引擎
3. 新的卡牌效果应实现Effect特质并注册到效果管理器
4. 数据导入功能应实现DataImporter特质
5. 游戏事件应添加到GameEvent枚举并在适当时候触发
6. 游戏动作应在actions.rs中实现，并在Game结构体中提供execute_action方法执行

### 测试
每个模块都应包含相应的测试用例，放在文件末尾的tests模块中。最近添加了针对洗牌功能、游戏动作执行和穆勒规则重抽流程的测试。

## 构建和运行

```bash
# 标准构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 启用所有功能
cargo build --features full
```