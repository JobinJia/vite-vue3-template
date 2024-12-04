#!/usr/bin/env node
import { mkdirSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'
import process from 'node:process'
import { Command } from 'commander'
import inquirer from 'inquirer'

const program = new Command()

async function userInput() {
  const answers = await inquirer.prompt([
    {
      type: 'input',
      name: 'moduleName',
      message: '请输入模块名称:',
      validate: (input: string) => {
        if (!input.trim())
          return '模块名称不能为空'
        return true
      },
    },
  ])
  return answers
}

program
  .name('g-api')
  .description('生成API模块文件')
  .argument('[apiModuleName]', '模块名称')
  .action(async (apiModuleName?: string) => {
    const answers = !apiModuleName ? await userInput() : { moduleName: apiModuleName }
    const { moduleName } = answers
    const basePath = resolve(process.cwd(), 'packages/request/src/api/modules', moduleName)

    // 创建目录
    mkdirSync(basePath, { recursive: true })

    // 生成类型定义文件
    const typesContent = `export interface ${moduleName}Request {
  // TODO: 添加请求参数类型
}

export interface ${moduleName}Response {
  // TODO: 添加响应数据类型
}
`
    writeFileSync(resolve(basePath, `${moduleName}.d.ts`), typesContent)

    // 生成API文件
    const apiContent = `import type { BaseResponse } from 'src/types'
import type { ${moduleName}Request, ${moduleName}Response } from './${moduleName}.d'
import { http } from 'src/fetch'

export const ${moduleName}Api = {
  // TODO: 添加API方法
  example: (data: ${moduleName}Request) =>
    http.Post<BaseResponse<${moduleName}Response>>('/${moduleName}/example', data),
}
`
    writeFileSync(resolve(basePath, `${moduleName}.api.ts`), apiContent)

    // 生成Mock文件
    const mockContent = `import type { MockServerRequest } from '@alova/mock'
import type { ${moduleName}Response } from './${moduleName}.d'
import { defineMock } from '@alova/mock'

export default defineMock({
  // Example Mock Api TODO
  '[POST]/${moduleName}/example': (params: MockServerRequest): ${moduleName}Response => {
    return {

    } as ${moduleName}Response
  },
})
`
    writeFileSync(resolve(basePath, `${moduleName}.mock.ts`), mockContent)

    console.log(`✨ 成功生成 ${moduleName} 模块文件`)
  })

program.parse()
