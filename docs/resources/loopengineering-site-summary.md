# Loop Engineering 网站内容整理

来源：<https://loopengineering.app/>

## 1. 这个网站在讲什么

Loop Engineering 是一个面向 AI agent / 自动化工作循环（loop）的实践与工具站点。
它不是单纯介绍“如何写 Prompt”，而是系统地讲如何让 AI agent 以更安全、可验证、可控的方式持续执行任务。

它的核心主题可以概括为：

- 把 AI agent 的工作拆成一个可重复运行的“循环”
- 每次循环都要有目标、最小可验证输入、证据、边界和停止规则
- 通过模板、检查清单、评分器和生成器，降低 agent 乱跑、走捷径、超预算的风险

可以把它理解成“AI agent 安全运行手册 + 生成器 + 模板库”。

## 2. 网站的定位与价值主张

这个网站强调四个关键词：

- 安全（safe）
- 可验证（verifiable）
- 可控（controllable）
- 预算意识（token-aware）

它最关心的不是“让 AI 快速做完”，而是“让 AI 在可控范围内、用尽可能小的代价、在可被审查的条件下完成任务”。

网站的内容风格也很明确：

- 以“loop engineering”为主线
- 强调设计工作流，而不是只讲单次 Prompt
- 提醒用户在执行 agent loop 前，先明确假设、目标、验证标准和停止条件
- 很重视 maker/checker、独立审查、禁止操作和人工审批

## 3. 网站包含的主要栏目

### 3.1 首页

首页的核心作用是把整个网站的定位讲清楚：

- 这是一个给 AI agent loop 做设计、评估和执行辅助的工具站
- 重点不是“生成代码”，而是“给 agent 任务做更稳妥的执行框架”
- 首页内容中能看到一类典型的任务示例，例如“修复 PR 的 CI 失败”，并强调要：
  - 用最小改动修复
  - 只处理当前失败
  - 保留验证和人工 review
  - 不随意删测试或绕过检查

这说明网站的底层假设是：

“即便 agent 很强，也必须被约束在一个足够小、足够可验证的控制范围内运行。”

### 3.2 Loop Goal Generator

这个页面是一个目标生成器，用来帮助用户写出更适合 agent loop 执行的任务目标。

它重点解决的问题是：

- 目标写得太模糊，agent 不知道到底做什么
- 缺少验证标准，结果无法判断是否成功
- 缺少边界和停止规则，agent 容易扩大范围

生成的内容通常会包含：

- 假设（hypothesis）
- 最小可用运行（smallest useful run）
- 验证证据（validation evidence）
- 停止规则（stop rule）
- 反馈记录（feedback log）

它的价值在于把“任务说明”从自然语言 prompt，变成更像工程规格说明的 loop 任务定义。

### 3.3 Loop Budget Calculator

这是一个预算估算工具，用来量化 AI agent loop 的成本。

它主要帮助用户回答：

- 这次循环会花多少钱（按 token 或成本估算）
- 运行一次、一天、一个月大概要多少预算
- 风险等级如何
- 如何优化成本

它体现出网站的一种很重要的思路：

“AI agent 不是无限可用的资源，必须把成本也纳入 loop 设计中。”

### 3.4 Loop Readiness Score

这是站点中最有代表性的评估工具之一。

它会让用户回答 11 个问题，分别从以下维度评分：

- 假设是否清楚
- 第一次运行是否足够小
- 目标是否具体
- 是否能通过测试/命令/产物验证
- 是否有独立审查者
- 是否定义了禁止操作
- 是否有停止条件
- 是否有预算上限
- 是否有回退方案
- 是否能在隔离环境中运行
- 是否需要人工审批

它的意义不是给出一个“漂亮分数”，而是告诉你：

“这个任务是否已经准备好交给 agent 自动跑，或者是否还需要先补齐安全和验证条件。”

### 3.5 Loop Maturity Assessment

这个页面用于评估一个团队/个人的 AI 工作流成熟度。

它把 AI 使用阶段分层，从更基础的手工提示，到更高级的调度循环、自动化及多 agent 系统，帮助用户判断自己处在什么阶段。

它的目的不是“炫技”，而是帮助团队理解：

- 现在的 AI 工作流属于哪一层
- 还缺哪些治理、规范和基础设施
- 哪些步骤适合开始自动化，哪些步骤还不适合

### 3.6 Methodology Skill Generator

这个页面提供“方法论技能生成器”，帮助用户把常见分析方法转化成 agent 可使用的技能说明。

它支持的思维框架包括：

- Pyramid Principle
- 5 Whys
- SMART
- PDCA
- SWOT
- AIDA
- OKR
- WBS

这说明网站不止是在教“怎么让 agent 写代码”，还在教“怎么让 agent 使用更严谨的分析方法”。

### 3.7 AGENTS.md Generator

这个页面用于生成 AGENTS.md 文件。

AGENTS.md 是很多 AI coding agent（如 Codex、Claude Code、Cursor 等）会参考的说明文件。

它的作用是让团队把工作规范、约束、代码风格、流程要求固化下来，减少 agent 在不同仓库里行为不一致的问题。

### 3.8 SKILL.md Generator

这个页面用于生成可复用的 SKILL.md 文件。

SKILL.md 的用途是为 agent 提供“可调用的任务技能说明”，例如某种特定 workflow、某类分析方法或某种执行模板。

与 AGENTS.md 不同的是：

- AGENTS.md 更偏向整体团队/仓库规则
- SKILL.md 更偏向“某个能力/方法的可复用说明”

### 3.9 SKILL.md Templates

这个栏目提供一组可直接复制的 SKILL.md 模板，分别面向不同工具/标准：

- 通用模板（vendor-neutral）
- Codex 模板
- Claude Code 模板

它的价值在于降低“从零开始写技能文件”的门槛。

## 4. 模板库：这个网站最实用的部分

Templates 是整个网站最像“实战手册”的部分。它提供了大量可直接使用的 loop 模板。

这些模板不是泛泛而谈的流程说明，而是面向特定任务场景的可复制 prompt / workflow。

### 4.1 典型模板类别

网站提供的模板包括但不限于：

- CI failure fix loop：修复 PR 中的 CI 问题
- Bug fixing loop：复现 bug 并做最小修复
- Code review loop：对代码变更进行审查
- PR babysitter loop：盯着 PR 直到 CI 和评论都处理完
- Quality checker loop：用独立 checker 验证 agent 输出
- Methodology skill loop：把某个方法变成可复用 skill
- Feedback improvement loop：把失败经验转化成改进模板
- Documentation loop：保持文档与代码同步
- Refactor loop：小步重构并验证行为不变
- Dependency update loop：安全升级依赖
- Migration loop：迁移到新框架/版本
- Release notes loop：生成发布说明
- SEO content refresh loop：更新 SEO 内容
- Research summary loop：收集资料并形成结论摘要
- Data cleaning loop：清洗与验证数据
- Keyword monitoring loop：观察关键词变化并输出机会

这个模板库说明网站不是“概念型内容”，而是已经把很多真实工作流变成了可复制的 agent loop 方案。

## 5. 工具与资源目录

Tools 页面把网站中涉及的工具、运行时、编排器、记忆层和社区实践聚合起来。

它的定位是：

- 介绍可用的工具与运行环境
- 提醒用户在生产环境中要验证成熟度、许可、权限和价格
- 让用户了解哪些工具适合做 loop engineering

这说明网站的边界并不只局限在“模板”，而是把整个 agent 生态也纳入了参考范围。

## 6. Guides 和 Checklists：把理念变成操作说明

### 6.1 Guides

Guides 页面提供更深入的实践文章，比如：

- 什么是 loop engineering
- 如何建立第一个 loop
- 如何做 validation
- 如何使用 checkers
- 如何处理 token cost
- 如何使用 memory
- 如何比较不同 loop 模式

### 6.2 Checklists

Checklists 页面提供更短、更偏执行的核对清单，适合在实际执行前快速检查：

- 第一个 loop 是否准备好了
- 目标是否足够清晰
- 验证和证据是否足够完整
- 工作区是否隔离
- 是否需要独立 reviewer
- 是否设置了停止规则

这类内容体现出网站强烈的“工程化”气质：它希望把 agent loop 从“经验”提升到“流程”。

## 7. Failures：它非常重视“失败案例”

Failures 页面不是在讲“怎么做得好”，而是在讲“哪些事情容易出错”。

其中列出的典型失败场景包括：

- agent 使用了过期的项目记忆
- agent 合并了有缺陷的 PR
- agent 重复执行同一条失败命令
- agent 消耗大量 token 但没有进展
- agent 改了无关文件
- agent 删除测试以骗过 CI

这很重要，因为它表明网站不是只讲理想流程，而是把“真实风险”也纳入了设计中。

## 8. Sources：它的内容不是凭空发明的

Sources 页面把网站引用的官方文档、开源项目和研究资料汇总起来。

这说明它的内容体系是建立在现有实践基础之上的，而不是纯粹的营销内容。

## 9. 网站的共同设计原则

从所有页面看，站点背后有一套非常清晰的设计原则：

### 9.1 先定义目标，再定义 loop

任何 loop 都不是“让 agent 随便跑”，而是先回答：

- 这次循环要证明什么
- 成功标准是什么
- 失败时应该如何处理

### 9.2 用最小可用运行验证假设

它强调一开始不要做大改动，而是先用很小的实验去验证假设。

### 9.3 验证要独立、要可观测

它非常重视“独立 checker”与“证据记录”。

### 9.4 明确禁止操作

它会要求用户声明哪些操作不能做，例如：

- 不要删除测试
- 不要绕过 lint/类型检查
- 不要修改无关文件
- 不要在无审批时做 merge / deploy / delete / 外部通信

### 9.5 必须有停止规则

任何 loop 都应该说明：

- 什么时候算成功
- 什么时候算失败
- 什么时候该停
- 超过几次失败就要停止
- 超过预算就要停止

### 9.6 需要人类审批

对于高风险动作，网站强烈主张人工审批，而不是完全放权。

## 10. 这个网站适合什么人使用

它最适合以下人群：

- 正在使用 Claude Code / Codex / Cursor 等 agent 的开发者
- 团队内部想把 AI 自动化流程规范化的人
- 需要给 agent 任务做结构化说明的技术负责人
- 想让 AI loop 更安全、更可控的工程团队
- 希望从“单次 prompt”升级到“工程化 workflow”的人

## 11. 一句话总结

Loop Engineering 这个网站的本质是：

“把 AI agent 的工作从‘随便试一下’升级为‘可验证、可控、可停止、可审查的工程循环’。”

它提供的不是单一工具，而是一整套方法论、模板、检查清单、生成器和治理思路，帮助用户把 AI agent 用在更稳妥的方式下。
