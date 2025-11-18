# PTCG Core 项目结构说明

## 项目概述

PTCG Core 是一个使用 Rust 编写的宝可梦集换式卡牌游戏(Pokemon Trading Card Game)核心引擎。该项目采用模块化设计，提供了卡牌数据结构、游戏规则引擎、事件系统、效果系统和数据导入导出功能。当前测试覆盖率为11.79%。

## 目录结构

```
ptcg-core/
├── examples/                 # 使用示例
├── src/                      # 源代码目录
│   ├── core/                 # 核心数据结构
│   │   ├── card/             # 卡牌相关模块
│   │   │   ├── abilities.rs  # 卡牌能力
│   │   │   ├── attacks.rs    # 卡牌攻击
│   │   │   ├── energy.rs     # 能量卡牌
│   │   │   ├── mod.rs        # 卡牌模块声明
│   │   │   ├── pokemon.rs    # 宝可梦卡牌
│   │   │   ├── trainer.rs    # 训练家卡牌
│   │   │   └── types.rs      # 卡牌类型定义
│   │   ├── deck/             # 牌组相关模块
│   │   │   ├── manager.rs    # 牌组管理
│   │   │   ├── mod.rs        # 牌组模块声明
│   │   │   └── validation.rs # 牌组验证
│   │   ├── game/             # 游戏相关模块
│   │   │   ├── actions/      # 游戏动作
│   │   │   │   ├── attack_actions.rs  # 攻击动作
│   │   │   │   ├── card_actions.rs    # 卡牌动作
│   │   │   │   ├── energy_actions.rs  # 能量动作
│   │   │   │   ├── execution.rs       # 动作执行
│   │   │   │   └── mod.rs             # 动作模块声明
│   │   │   ├── setup/        # 游戏设置
│   │   │   │   ├── deck_setup.rs      # 牌组设置
│   │   │   │   ├── mod.rs             # 设置模块声明
│   │   │   │   ├── mulligan_setup.rs  # 穆勒规则设置
│   │   │   │   ├── player_setup.rs    # 玩家设置
│   │   │   │   ├── pokemon_setup.rs   # 宝可梦设置
│   │   │   │   └── turn_setup.rs      # 回合设置
│   │   │   ├── events.rs     # 游戏事件
│   │   │   ├── mod.rs        # 游戏模块声明
│   │   │   ├── state.rs      # 游戏状态管理
│   │   │   └── turn.rs       # 回合处理逻辑
│   │   ├── player/           # 玩家相关模块
│   │   │   ├── actions.rs    # 玩家动作
│   │   │   ├── conditions.rs # 玩家状态条件
│   │   │   ├── mod.rs        # 玩家模块声明
│   │   │   └── state.rs      # 玩家状态管理
│   │   └── mod.rs            # 核心模块声明
│   ├── data/                 # 数据导入/导出模块
│   │   ├── csv.rs            # CSV数据导入
│   │   ├── database.rs       # 数据库导入
│   │   ├── export.rs         # 数据导出
│   │   ├── import.rs         # 数据导入
│   │   ├── json.rs           # JSON数据导入/导出
│   │   └── mod.rs            # 数据模块声明
│   ├── effects/              # 卡牌效果系统
│   │   ├── manager.rs        # 效果管理器
│   │   ├── mod.rs            # 效果模块声明
│   │   ├── outcomes.rs       # 效果结果
│   │   ├── targets.rs        # 效果目标
│   │   └── types.rs          # 效果类型
│   ├── events/               # 事件系统
│   │   ├── bus.rs            # 事件总线
│   │   ├── handlers.rs       # 事件处理器
│   │   ├── mod.rs            # 事件模块声明
│   │   └── types.rs          # 事件类型
│   ├── lib.rs                # 库入口和重新导出
│   ├── network/              # 网络功能(可选)
│   │   ├── client.rs         # 网络客户端
│   │   ├── mod.rs            # 网络模块声明
│   │   └── server.rs         # 网络服务器
│   └── rules/                # 规则引擎系统
│       ├── effects.rs        # 规则效果
│       ├── engine.rs         # 规则引擎
│       ├── mod.rs            # 规则模块声明
│       ├── standard.rs       # 标准规则
│       └── validation.rs     # 规则验证
├── Cargo.toml                # 项目配置文件
├── README.md                 # 项目说明文档
└── TODO.md                   # 开发待办事项
```

## 各模块详细说明

### 1. 核心模块 (src/core/)

核心模块包含了游戏的基础数据结构和主要实体。

#### 1.1 卡牌模块 (src/core/card/)

包含所有与卡牌相关的数据结构和功能：
- `abilities.rs`: 卡牌能力定义
- `attacks.rs`: 攻击信息结构体和相关方法
- `energy.rs`: 能量卡牌相关功能（待实现）
- `pokemon.rs`: 宝可梦卡牌结构体和方法
- `trainer.rs`: 训练家卡牌相关功能（待实现）
- `types.rs`: 卡牌类型枚举（CardType, EnergyType, EvolutionStage, TrainerType, CardRarity）

#### 1.2 玩家模块 (src/core/player/)

管理玩家状态和操作：
- `actions.rs`: 玩家可执行的动作
- `conditions.rs`: 玩家特殊状态条件（中毒、麻痹等）
- `state.rs`: 玩家结构体，包含玩家的所有状态信息和操作方法

#### 1.3 牌组模块 (src/core/deck/)

处理牌组构建和验证：
- `manager.rs`: 牌组结构体和基本操作方法
- `validation.rs`: 牌组验证规则和错误类型

#### 1.4 游戏模块 (src/core/game/)

管理游戏状态和流程：

##### 1.4.1 游戏状态 (src/core/game/state.rs)
- `Game`: 主游戏结构体
- `GamePhase`: 游戏阶段枚举
- `GameState`: 游戏状态枚举
- `GameRules`: 游戏规则配置
- `GameEvent`: 游戏事件枚举

##### 1.4.2 游戏设置 (src/core/game/setup/)
- `deck_setup.rs`: 牌组设置相关功能
- `mulligan_setup.rs`: 穆勒规则处理（优化后的重抽与基础宝可梦检查流程）
- `player_setup.rs`: 玩家加入游戏和设置
- `pokemon_setup.rs`: 宝可梦相关设置（活跃宝可梦选择、备战区设置）
- `turn_setup.rs`: 先后手决定

##### 1.4.3 回合处理 (src/core/game/turn.rs)
- 游戏开始
- 回合开始/结束
- 阶段推进
- 胜负条件检查

##### 1.4.4 游戏动作 (src/core/game/actions/)
- `attack_actions.rs`: 攻击相关动作
- `card_actions.rs`: 卡牌相关动作
- `energy_actions.rs`: 能量相关动作
- `execution.rs`: 游戏动作执行功能（抽卡、附加能量、攻击、结束回合等）

### 2. 规则引擎模块 (src/rules/)

提供灵活的规则验证和执行系统：
- `engine.rs`: 规则引擎核心功能（Rule特质、RuleEngine、GameAction枚举、RuleViolation等）
- `standard.rs`: 标准规则实现（回合顺序、手牌限制、能量附加等）
- `effects.rs`: 规则效果相关（待实现）
- `validation.rs`: 规则验证相关（待实现）

### 3. 事件系统模块 (src/events/)

追踪游戏状态变化和通知：
- `bus.rs`: 事件总线，管理事件分发
- `handlers.rs`: 事件处理器实现（ConsoleEventHandler等）
- `types.rs`: 事件类型定义（GameEvent枚举等）

### 4. 效果系统模块 (src/effects/)

实现卡牌效果和触发器：
- `manager.rs`: 效果管理器（EffectManager）
- `types.rs`: 效果类型定义（Effect特质、EffectTrigger枚举、EffectTarget枚举等）
- `targets.rs`: 效果目标相关实现
- `outcomes.rs`: 效果结果和错误类型（待实现）

### 5. 数据模块 (src/data/)

处理不同格式的数据导入/导出：

#### 5.1 JSON支持 (src/data/json.rs)
- `JsonImporter`: JSON数据导入器
- `JsonExporter`: JSON数据导出器（部分实现）

#### 5.2 CSV支持 (src/data/csv.rs)
- `CsvImporter`: CSV数据导入器（占位符）

#### 5.3 数据库支持 (src/data/database.rs)
- `DatabaseImporter`: 数据库导入器（占位符）

#### 5.4 数据导入/导出 (src/data/import.rs, src/data/export.rs)
- 通用导入/导出特质
- 错误类型定义
- 批量导入器框架

### 6. 网络模块 (src/network/)

提供网络功能支持（需要启用async特性）：
- `client.rs`: 网络客户端（待实现）
- `server.rs`: 网络服务器（待实现）
- `mod.rs`: 网络模块声明

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
每个模块都应包含相应的测试用例，放在文件末尾的tests模块中。最近添加了针对洗牌功能、游戏动作执行和穆勒规则重抽流程的测试。使用cargo-tarpaulin检查测试覆盖率为11.79%。

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