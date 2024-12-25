import type { RenderOptions } from '@testing-library/vue'
import type { ComponentMountingOptions } from '@vue/test-utils'
import type { Component } from 'vue'
import { render } from '@testing-library/vue'
import { mount } from '@vue/test-utils'

export function mountComponent<T extends Component>(
  component: T,
  options?: ComponentMountingOptions<any>,
) {
  return mount(component as any, options)
}

export function renderComponent<T extends Component>(
  component: T,
  options?: RenderOptions,
) {
  return render(component as any, options)
}
