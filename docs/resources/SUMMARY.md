# Resources 目录内容总结

这个目录目前主要围绕 **Loop Engineering** 展开，包含一篇英文长文、一篇对应中文译文，以及一篇对 `loopengineering.app` 网站的中文梳理。整体主题高度一致：**不是教人如何写一次 prompt，而是教人如何把 AI agent 放进一个可持续运行、可验证、可审查、可停止的工作循环里。**

## 文件级摘要

| 文件 | 内容定位 | 主要内容 |
| --- | --- | --- |
| `Loop-Engineering-IEEE.md` | 英文原文 / 长篇论文式综述 | 系统定义了 Loop Engineering：它位于 prompt、context、harness 之上的第四层，核心是让系统自己提示 agent、自己循环执行。文中提出 loop 的 **五个动作**（discovery、handoff、verification、persistence、scheduling）与 **六个组成部分**（automations、worktrees、skills、connectors、sub-agents、memory），强调 **generator / evaluator 分离** 是可靠性的关键；同时给出五种常见失败模式、三个真实案例、四类隐性成本，以及一个“如何搭第一个 loop”的最小可行方案。 |
| `Loop-Engineering-IEEE_zh_CN.md` | 上述论文的中文全译 | 基本完整保留了英文原文结构、表格、图片与代码块，便于中文阅读与内部传播。内容重点与英文版一致，但更适合作为团队共识材料使用。 |
| `loopengineering-site-summary.md` | 对 `https://loopengineering.app/` 的中文站点导读 | 从站点结构和方法论角度做了归纳，介绍了首页、Goal Generator、Budget Calculator、Readiness Score、Maturity Assessment、AGENTS.md / SKILL.md 生成器、模板库、Guides、Checklists、Failures、Sources 等栏目。核心结论是：这个站点本质上是一个 **AI agent loop 的工程化实践手册与模板库**，强调安全、验证、预算、人工审批与停止规则。 |

## 这几份材料共同在讲什么

可以把整个目录压缩成 4 个共识：

1. **Loop Engineering 是“系统设计”而不是“提示技巧”**  
   重点不再是人逐轮驱动 agent，而是把任务发现、执行、验证、记录、再调度做成自动循环。

2. **验证比生成更重要**  
   材料反复强调，agent 很容易为自己的输出“点头通过”，所以必须引入独立 checker / evaluator，并优先验证真实行为，而不是只看表面代码。

3. **边界、证据、预算和停止规则是 loop 的基本安全装置**  
   没有这些约束，loop 很容易变成：范围失控、花费失控、结果不可审查、错误持续累积。

4. **人工判断不会消失，只会变得更重要**  
   这些文档的共同结论不是“让 AI 完全替代工程师”，而是“让工程师从机械执行者变成规则设计者、结果审查者与最终判断者”。

## 目录内最重要的概念

- **四层栈**：Prompt -> Context -> Harness -> Loop
- **五个动作**：发现、交接、验证、持久化、调度
- **六个组成部分**：自动化、worktree、skill、connector、sub-agent、memory
- **关键模式**：generator / evaluator 分离
- **常见风险**：验证债务、理解腐化、认知让渡、token 爆炸
- **实践原则**：小步开始、独立验证、保留人工 review、设置预算上限、明确禁止操作

## 推荐阅读顺序

如果是第一次看这个目录，建议按下面顺序读：

1. `loopengineering-site-summary.md`  
   先快速建立“Loop Engineering 是什么、网站提供哪些工具和模板”的整体印象。

2. `Loop-Engineering-IEEE_zh_CN.md`  
   再读中文版论文，建立完整概念框架与方法论细节。

3. `Loop-Engineering-IEEE.md`  
   最后回看英文原文，用于对照术语、引用原始表达，适合后续引用或继续延展研究。

## 一句话总览

这个 `resources` 目录本质上是一组围绕 **“如何把 AI agent 从一次性对话，升级成可验证、可控、可停止、可持续运行的工程循环”** 的资料集合：论文负责给出完整理论框架，中文译文负责降低阅读门槛，网站总结负责把方法论映射到实际工具、模板与执行场景。
