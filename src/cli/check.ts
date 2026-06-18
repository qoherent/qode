import { type CommandResult, fail, hasHelpFlag, ok } from './commands.ts'

const CHECK_HELP = `Usage: sigil check <files...>

Validate .sigil component files.
`

export function handleCheckCommand(args: readonly string[]): CommandResult {
  if (hasHelpFlag(args)) {
    return ok(CHECK_HELP)
  }

  const unknownOption = args.find((arg) => arg.startsWith('-'))
  if (unknownOption !== undefined) {
    return fail(`sigil check: unknown option: ${unknownOption}\n`)
  }

  if (args.length === 0) {
    return fail('sigil check: expected at least one file\n')
  }

  return ok('sigil check: validation is not implemented yet\n')
}
