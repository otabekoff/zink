// https://vitepress.dev/guide/custom-theme
import { h, onMounted, watch, nextTick } from 'vue'
import type { Theme } from 'vitepress'
import { useRoute } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import './style.css'

const ANIMATED_SELECTORS = '.zen-section, .code-showcase, .stats-bar, .motto-banner, .home-cta'

function initScrollAnimations(): void {
  setTimeout(() => {
    const elements = document.querySelectorAll<HTMLElement>(ANIMATED_SELECTORS)
    if (!elements.length) return

    const observer = new IntersectionObserver(
      (entries: IntersectionObserverEntry[]) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            entry.target.classList.add('fade-in', 'visible')
            observer.unobserve(entry.target)
          }
        }
      },
      { threshold: 0.15, rootMargin: '0px 0px -40px 0px' }
    )

    elements.forEach((el: HTMLElement) => {
      el.classList.add('fade-in')
      observer.observe(el)
    })
  }, 100)
}

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
    })
  },
  enhanceApp({ router }) {
    if (typeof window !== 'undefined') {
      router.onAfterRouteChange = () => {
        initScrollAnimations()
      }
    }
  },
  setup() {
    const route = useRoute()

    onMounted(() => {
      initScrollAnimations()
    })

    watch(
      () => route.path,
      () => {
        nextTick(() => initScrollAnimations())
      }
    )
  }
} satisfies Theme
