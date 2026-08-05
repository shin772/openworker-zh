// 运行时 DOM 翻译引擎 – OpenWorker v0.1.7 中文汉化
// 使用 MutationObserver + 词表精确匹配，无需修改源码

import zhCN from './zh-CN.json';

const DICT: Record<string, string> = zhCN;

// 规范化：去首尾空格、多个空格合并为一个
function normalize(s: string): string {
  return s.replace(/\s+/g, ' ').trim();
}

function translateNode(node: Text) {
  const raw = node.textContent ?? '';
  const key = normalize(raw);
  if (DICT[key] && DICT[key] !== key) {
    node.textContent = raw.replace(key, DICT[key]);
  }
}

// 处理 input/textarea placeholder
function translatePlaceholder(el: HTMLElement) {
  const ph = el.getAttribute('placeholder');
  if (ph) {
    const key = normalize(ph);
    if (DICT[key]) {
      el.setAttribute('placeholder', DICT[key]);
    }
  }
}

// 处理 aria-label
function translateAriaLabel(el: HTMLElement) {
  const al = el.getAttribute('aria-label');
  if (al) {
    const key = normalize(al);
    if (DICT[key]) {
      el.setAttribute('aria-label', DICT[key]);
    }
  }
}

// 处理 title 属性
function translateTitle(el: HTMLElement) {
  const t = el.getAttribute('title');
  if (t) {
    const key = normalize(t);
    if (DICT[key]) {
      el.setAttribute('title', DICT[key]);
    }
  }
}

function walkAndTranslate(root: Node) {
  if (root.nodeType === Node.TEXT_NODE) {
    translateNode(root as Text);
  } else if (root.nodeType === Node.ELEMENT_NODE) {
    const el = root as HTMLElement;
    translatePlaceholder(el);
    translateAriaLabel(el);
    translateTitle(el);
    root.childNodes.forEach(walkAndTranslate);
  }
}

// 初始翻译
let initTimer: ReturnType<typeof setTimeout>;
function scheduleInit() {
  clearTimeout(initTimer);
  initTimer = setTimeout(() => {
    walkAndTranslate(document.body);
    initTimer = setTimeout(() => walkAndTranslate(document.body), 2000);
  }, 500);
}

if (typeof document !== 'undefined') {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', scheduleInit);
  } else {
    scheduleInit();
  }

  // 监听后续 DOM 变化（含 characterData 以捕获 React 重渲染）
  const observer = new MutationObserver((mutations) => {
    for (const m of mutations) {
      if (m.type === 'characterData') {
        // React 异步重渲染后文本节点内容被直接替换
        if (m.target.nodeType === Node.TEXT_NODE) {
          translateNode(m.target as Text);
        }
        continue;
      }
      m.addedNodes.forEach((node) => {
        if (node.nodeType === Node.ELEMENT_NODE) {
          walkAndTranslate(node);
        }
      });
      if (m.type === 'attributes') {
        const el = m.target as HTMLElement;
        if (m.attributeName === 'placeholder') translatePlaceholder(el);
        if (m.attributeName === 'aria-label') translateAriaLabel(el);
        if (m.attributeName === 'title') translateTitle(el);
      }
    }
  });
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    characterData: true,
    attributeFilter: ['placeholder', 'aria-label', 'title'],
  });

  // 兜底轮询：处理 observer 启动前的短暂窗口
  let polls = 0;
  const MAX_POLLS = 40;
  const pollTimer = setInterval(() => {
    walkAndTranslate(document.body);
    polls += 1;
    if (polls >= MAX_POLLS) clearInterval(pollTimer);
  }, 3000);
}
