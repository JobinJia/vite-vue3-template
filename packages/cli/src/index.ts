#!/usr/bin/env node
import { mkdirSync, readdirSync, rmSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'
import process from 'node:process'
import { Command } from 'commander'
import inquirer from 'inquirer'
import { capitalize } from './util'

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
  .name('api')
  .description('API 模块管理工具')

// 生成命令
program
  .command('gen')
  .description('生成新的 API 模块')
  .argument('[apiModuleName]', '模块名称')
  .action(async (apiModuleName?: string) => {
    const answers = !apiModuleName ? await userInput() : { moduleName: apiModuleName }
    const { moduleName } = answers

    const capitalizedName = capitalize(moduleName)

    const basePath = resolve(process.cwd(), 'packages/request/src/api/modules', moduleName)

    // 创建目录
    mkdirSync(basePath, { recursive: true })

    // 生成类型定义文件
    const typesContent = `export interface ${capitalizedName}Request {
  // TODO: 添加请求参数类型
}

export interface ${capitalizedName}Response {
  // TODO: 添加响应数据类型
}
`
    writeFileSync(resolve(basePath, `${moduleName}.d.ts`), typesContent)

    // 生成API文件
    const apiContent = `import type { BaseResponse } from '../../../types'
import type { ${capitalizedName}Request, ${capitalizedName}Response } from './${moduleName}'
import { http } from '../../../fetch'

export const ${capitalizedName}Api = {
  // TODO: 添加API方法
  example: (data: ${capitalizedName}Request) =>
    http.Post<BaseResponse<${capitalizedName}Response>>('/${moduleName}/example', data),
}
`
    writeFileSync(resolve(basePath, `${moduleName}.api.ts`), apiContent)

    // 生成Mock文件
    const mockContent = `import type { MockServerRequest } from '@alova/mock'
import type { ${capitalizedName}Response } from './${moduleName}'
import { defineMock } from '@alova/mock'

export default defineMock({
  // Example Mock Api TODO
  // eslint-disable-next-line unused-imports/no-unused-vars
  '[POST]/${moduleName}/example': (params: MockServerRequest): ${capitalizedName}Response => {
    return {

    } as ${capitalizedName}Response
  },
})
`
    writeFileSync(resolve(basePath, `${moduleName}.mock.ts`), mockContent)

    console.log(`✨ 成功生成 ${moduleName} 模块文件`)
    // 重新生成 API 索引文件
    program.parse(['node', 'cli.js', 'export'])
  })

// 导出命令
program
  .command('export')
  .description('生成 API 索引文件')
  .action(() => {
    const modulesPath = resolve(process.cwd(), 'packages/request/src/api/modules')
    const modules = readdirSync(modulesPath, { withFileTypes: true })
      .filter(dirent => dirent.isDirectory())
      .map(dirent => dirent.name)

    const indexContent = `// This file is auto-generated, don't edit it manually.
${modules.map(module => `export * from './modules/${module}/${module}.api'\nexport * from './modules/${module}/${module}.d'`).join('\n')}
`
    const indexPath = resolve(process.cwd(), 'packages/request/src/api/index.ts')
    writeFileSync(indexPath, indexContent)

    console.log('✨ 成功生成 API 索引文件')
  })

program
  .command('delete')
  .description('删除 API 模块')
  .argument('[apiModuleName]', '模块名称')
  .action((apiModuleName?: string) => {
    const basePath = resolve(process.cwd(), 'packages/request/src/api/modules', apiModuleName)
    rmSync(basePath, { recursive: true })
    console.log(`✨ 成功删除 ${apiModuleName} 模块文件`)
    // 重新生成 API 索引文件
    program.parse(['node', 'cli.js', 'export'])
  })

program.parse()
