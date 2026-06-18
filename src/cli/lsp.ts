import { type CommandResult, fail, hasHelpFlag, ok } from './commands.ts'

const LSP_HELP = `Usage: sigil lsp [--help]

starts the Sigil language server over stdio.
`

export function handleLspCommand(args: readonly string[]): CommandResult {
  if (hasHelpFlag(args)) {
    return ok(LSP_HELP)
  }

  const unknownOption = args.find((arg) => arg.startsWith('-'))
  if (unknownOption !== undefined) {
    return fail(`sigil lsp: unknown option: ${unknownOption}\n`)
  }

  return fail('sigil lsp: language server is not implemented yet\n')
}
