import { handleCheckCommand } from './check.ts'
import { handleLspCommand } from './lsp.ts'

export interface CommandResult {
  status: number
  stdout: string
  stderr: string
}

export const MAIN_HELP = `Usage: sigil <command> [options]

Commands:
  sigil check <files...>  Validate .sigil component files.
  sigil lsp              Start the Sigil language server.

Options:
  -h, --help            Show help.
`

export function runSigilCommand(args: readonly string[]): CommandResult {
  const [command, ...commandArgs] = args

  if (command === undefined || isHelpFlag(command)) {
    return ok(MAIN_HELP)
  }

  switch (command) {
    case 'check':
      return handleCheckCommand(commandArgs)
    case 'lsp':
      return handleLspCommand(commandArgs)
    default:
      return fail(`Unknown command: ${command}\n\n${MAIN_HELP}`)
  }
}

export function hasHelpFlag(args: readonly string[]): boolean {
  return args.some(isHelpFlag)
}

export function ok(stdout: string): CommandResult {
  return {
    status: 0,
    stdout,
    stderr: '',
  }
}

export function fail(stderr: string): CommandResult {
  return {
    status: 1,
    stdout: '',
    stderr,
  }
}

function isHelpFlag(arg: string): boolean {
  return arg === '--help' || arg === '-h'
}
