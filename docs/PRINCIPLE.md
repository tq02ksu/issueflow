

## LOOP 定义

- 固定的状态
- SOUL（方向层）
- PRINCIPLE（约束层）
- EXECUTION（执行层：包含 DESIGN + KNOWLEDGE + SHORT-TERM GOALS + RULE）

🧠 各层含义
1) SOUL（不变）

回答：为什么做

任务使命
长期目标
价值优先级

👉 特点：稳定、不随任务变

2) PRINCIPLE（不变）

回答：什么是对的做法

不编造
不确定要说明
优先准确性 vs 速度
是否允许主动澄清

👉 特点：行为宪法

补充原则：
- 对象优先于对话
- 写操作默认进入待确认状态
- memory 接口稳定, 不绑定具体实现
- 高配置信息后置, 高频观察信息前置

3) EXECUTION（变化层 ⭐）

回答：现在怎么做

这是你原来 3 个东西的合并：

包含内容：
A. SHORT-TERM GOALS（短期目标）✔
当前要完成什么
子目标拆解
任务阶段
B. DESIGN（执行流程）✔
plan → act → observe
是否多步
是否需要重规划
C. KNOWLEDGE（信息）✔
用户输入
工具结果
记忆上下文
D. RULE（输出约束）✔（轻量保留）
输出格式
长度限制
JSON / Markdown 等

## 对象原则

系统的主对象不是对话, 而是：

- LOOP
- RUN
- MEMORY
- APPROVAL / PENDING ACTION
- SKILL

聊天和消息流只是交互方式, 不是系统主模型。

## 写操作原则

默认读多写少。

任何外部写操作默认都不直接执行, 而是先形成待确认动作：

- GitLab comment
- GitLab issue / MR 更新
- 环境写入
- 外部系统写入

确认主体可以是：

- 人工确认
- 更高层 LOOP 确认
- 明确授权的系统级策略确认

对于 LOOP 内的执行结果, 更合理的分工是：

- executor 负责执行
- evaluator 负责确认
- evaluator 产出结论性消息, 再提交给上层 LOOP 使用

## Memory 原则

memory 是系统能力, 不是某个具体产品能力。

- 当前采用 mem0 作为通用记忆系统
- 后端通过稳定接口与 memory 系统交互
- 未来如替换 memory 实现, 只要接口不变, 后端逻辑不需要跟着改

重点不是 memory 内部怎么做, 而是：

- 什么时候写
- 写什么结构
- 怎么被 LOOP 读取和使用

## SKILL 原则

SKILL 是 LOOP 的能力引用对象, 也是 Agent Runtime 的执行对象。

也就是说：

- LOOP 决定使用什么 SKILL
- Agent Runtime 决定怎样执行这个 SKILL
- SKILL 本身应支持版本管理和升级

因此 SKILL 既不应完全写死在 LOOP 里, 也不应完全散落在 Runtime 实现里。

SKILL 升级本身应作为人工确认动作处理。

升级内容不仅可能是 SKILL 版本本身, 也可能是对 LOOP 定义中以下层的修改：

- SOUL
- PRINCIPLE
- DESIGN

## 交互优先级原则

系统应该优先暴露高频观察信息, 再暴露低频配置能力。

高频信息：

- LOOP 当前状态
- RUN 当前结果
- 待确认动作
- 当前 memory 摘要

低频信息：

- provider 配置
- secret 管理
- skill 版本管理
- 环境配置

这条原则不仅影响 UI, 也影响模块入口组织方式。

## 人工介入原则

人工可以随时介入各 agent 的执行。

系统设计时必须原生考虑不同强度的介入方式, 不能只支持最终审批：

- 停止一个工具调用
- 发送 steering 消息
- 停止本次 run
- 停止整个 loop

因此运行时架构、消息队列、状态机和 agent 工作流都必须兼容这种中途介入能力。

## 进化
与其评估提示词, 评估SKILL, 评估LOOP, 系统复杂度越来越高, 个性化太强, 评估成本越来越高, 不如让LOOP自我进化。

## 决策: LOOP指导LOOP进化
采用系统级LOOP指导LOOP 进化，形成自我进化的闭环。
系统预置LOOP进化原则，LOOP在执行过程中自我评估，发现问题后自我修正，形成自我进化的闭环。

**依赖子项**
- **记忆系统**: 采用mem0做为通用记忆系统, 记录LOOP执行过程中的问题和改进建议, 形成自我进化的闭环。
- **可观测平台**: otel 兼容的可观测平台, 统一的元数据定义方便 LOOP评估系统反思. ~~langfuse~~

## Agent Runtime
不同类型LOOP的执行环境需要采用不同的Agent Runtime, 这个是Harness, Context工程领域的工作和差异性, 本系统采用支持插件的方式来集成而不是自己开发。

## LOOP 控制Agent职责

LOOP 控制Agent 的职责不是直接完成所有重型执行, 而是负责：

- 根据预算决定执行范围和强度
- 规划任务
- 调起 Agent
- 跟进 Agent 执行过程中遇到的问题
- 整理执行结果
- 做评估
- 做优化
- 实时更新任务状态
- 实时更新总结信息

因此 LOOP 控制Agent 更接近 orchestrator / manager, 而不是单一执行器。

不同类型任务
- **编程任务**: OpenCode, codex, copilot cli
- **通用任务**: Hermes, OpenClaw

## 还需要继续讨论的重要点

1. evaluator 的结论性消息以什么标准结构提交给上层 LOOP
2. 人工介入、evaluator 结论、系统策略限制三者在运行时的优先级和覆盖关系
3. SKILL / LOOP 定义变更的人工确认流程, 是统一审批模型还是分别处理
