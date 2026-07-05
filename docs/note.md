# NOTE

## 2026-7-4

基于这些文档及我个的想法重新写设计文档 DESIGN.md
1. 这个系统不做: opencode, codex, some like, openclaw, hermes 等. 但是需要的时候会调用.
2. 以schedule(chat方式设计定时任务), state machine(系统级与用户级), skills 的方式做Loop定义.
3. 需要一个记忆层, 来保证loop执行是持久化的.
4. 会自研一个轻量级的Agent来执行Loop的核心运转, 需要通过A2A协议与opencode, openclaw等即时通信.
5. 消息通知系统及人工确认 统一返馈机制. 系统级验证债务, 认知退化等交互平台, 被动人工介入.
6. Ag-ui a2ui 及时显示 agent运行情况, 随时可以break. steering.
7. 中优,AI网关及预算控制. 在不同阶段自动切换模型, 控制token预算, 集中管理权限.
8. 每个人可以设定人格, 说话方式, loop定义, 目标定义(中长期), 最期指示(近期的目标与规则).
9. agent session/instance管理. 除loop agent是系统级agent 其它agent 的生命周期需要及时管理相关的资源占用.
10. 环境密钥管理(与环境相关skill配合使用). -- 这个能实现AI搭环境, AI做浏览器检查.
11. skill注册中心. 版本管理.
12. 进化机制: SKILL 版本升级. 系统设计的 LOOP 负责提供SKILL的升级建议来进化系统, 也提供建议式报告.