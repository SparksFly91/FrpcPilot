import MarkdownIt from "markdown-it"

/**
 * 更新日志（Release Notes）Markdown 渲染器。
 *
 * 安全约束：
 * - `html: false`：禁用原始 HTML，杜绝注入；
 * - markdown-it 默认的 `validateLink` 会拦截 `javascript:` 等危险链接。
 */
const md = new MarkdownIt({
  html: false,
  linkify: true,
  breaks: true,
})

// 外链在新窗口打开（链接点击在组件中已被接管，统一交由系统浏览器处理）
const defaultLinkOpen = md.renderer.rules.link_open
md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
  const token = tokens[idx]
  token.attrSet("target", "_blank")
  token.attrSet("rel", "noopener noreferrer")
  return defaultLinkOpen
    ? defaultLinkOpen(tokens, idx, options, env, self)
    : self.renderToken(tokens, idx, options)
}

/** 将 Markdown 文本渲染为 HTML 字符串（空内容返回空串） */
export function renderMarkdown(source?: string | null): string {
  if (!source || !source.trim()) return ""
  return md.render(source)
}
