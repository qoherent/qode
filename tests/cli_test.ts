import { assertEquals, assertStringIncludes } from '@std/assert'
import { runSigilCommand } from '../src/cli/mod.ts'

Deno.test('sigil --help returns usage text', () => {
  const result = runSigilCommand(['--help'])

  assertEquals(result.status, 0)
  assertStringIncludes(result.stdout, 'Usage: sigil')
  assertStringIncludes(result.stdout, 'sigil check')
  assertStringIncludes(result.stdout, 'sigil lsp')
  assertEquals(result.stderr, '')
})

Deno.test('sigil check with no files reports a clear error', () => {
  const result = runSigilCommand(['check'])

  assertEquals(result.status, 1)
  assertEquals(result.stdout, '')
  assertStringIncludes(result.stderr, 'sigil check')
  assertStringIncludes(result.stderr, 'expected at least one file')
})

Deno.test('sigil lsp --help documents language server startup', () => {
  const result = runSigilCommand(['lsp', '--help'])

  assertEquals(result.status, 0)
  assertStringIncludes(result.stdout, 'Usage: sigil lsp')
  assertStringIncludes(result.stdout, 'starts the Sigil language server')
  assertEquals(result.stderr, '')
})

Deno.test('invalid commands return non-zero status', () => {
  const result = runSigilCommand(['unknown'])

  assertEquals(result.status, 1)
  assertEquals(result.stdout, '')
  assertStringIncludes(result.stderr, 'Unknown command: unknown')
})
