import { describe, expect, it } from 'vitest'
import { defineComponent, h } from 'vue'
import { mountComponent, renderComponent } from '../helpers'

describe('helpers', () => {
  const TestComponent = defineComponent({
    render() {
      return h('div', 'test')
    },
  })

  it('mountComponent should work', () => {
    const wrapper = mountComponent(TestComponent)
    expect(wrapper.text()).toBe('test')
  })

  it('renderComponent should work', () => {
    const { getByText } = renderComponent(TestComponent)
    expect(getByText('test')).toBeTruthy()
  })
})
