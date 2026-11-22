// Reads `NAME=value` lines the way the demo's vars.env is meant to be read;
// blank lines and `#` comments are skipped.
export function parseEnvFile(text: string): Map<string, string> {
  const vars = new Map<string, string>();

  for (const line of text.split('\n')) {
    const trimmed = line.trim();
    const separator = trimmed.indexOf('=');

    if (trimmed === '' || trimmed.startsWith('#') || separator <= 0) {
      continue;
    }

    vars.set(trimmed.slice(0, separator).trim(), trimmed.slice(separator + 1));
  }

  return vars;
}
