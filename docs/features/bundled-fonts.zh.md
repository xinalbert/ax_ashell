[English](bundled-fonts.md) · [文档导航](../README.zh.md)

# 内置字体

AxShell 内置一组精简的终端友好字体，让用户尚未配置系统字体时也有可用默认值。运行时字体文件和本地授权说明位于 [assets/fonts](../../assets/fonts/README.md)。

## 字体家族

| 字体 | 来源仓库 | 内置版本 | 内置样式 | 用途 |
| --- | --- | --- | --- | --- |
| Maple Mono NF CN | [subframe7536/maple-font](https://github.com/subframe7536/maple-font) | bundled release | Regular, Bold | 默认终端字体，覆盖 CJK 与 Nerd Font 字形 |
| Iosevka Term | [be5invis/Iosevka](https://github.com/be5invis/Iosevka) | 8.0.0 | Regular, Bold, Italic, Bold Italic | 紧凑终端文本和高密度英文代码 |
| JetBrains Mono | [JetBrains/JetBrainsMono](https://github.com/JetBrains/JetBrainsMono) | 2.304 | Regular, Bold, Italic, Bold Italic | 通用编程与终端文本 |
| Monaspace Neon Var | [githubnext/monaspace](https://github.com/githubnext/monaspace) | 1.400 | Variable font | 支持 200-800 weight、100-125 width 和 slant 的可变字体 |

AxShell 只内置渲染器实际选择的样式。Iosevka Extended、Oblique 与额外字重；JetBrains Mono NL、web 与额外字重；以及 Monaspace Argon、Krypton、Radon、Xenon 均有意排除。

## 授权

内置字体文件均按 SIL Open Font License 1.1 分发。本地授权说明随字体文件保存：

- [Maple Mono license](../../assets/fonts/LICENSE.txt)
- [Iosevka license](../../assets/fonts/LICENSE-Iosevka.txt)
- [JetBrains Mono license](../../assets/fonts/LICENSE-JetBrainsMono.txt)
- [Monaspace license](../../assets/fonts/LICENSE-Monaspace.txt)

调整内置字体时，需要同步更新本页、[assets/fonts/README.md](../../assets/fonts/README.md)、`src/app/theme.rs` 和 Settings 字体排序。
