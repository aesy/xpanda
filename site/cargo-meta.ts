import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

// The page shows the crate's version and MSRV, so read them from the
// workspace manifest instead of keeping a second copy.
const manifest = readFileSync(fileURLToPath(new URL('../Cargo.toml', import.meta.url)), 'utf8');

function field(name: string): string {
  const match = new RegExp(`^${name}\\s*=\\s*"([^"]+)"`, 'm').exec(manifest);

  if (!match) {
    throw new Error(`Cargo.toml has no ${name}`);
  }

  return match[1];
}

export const cargoDefines = {
  __XPANDA_VERSION__: JSON.stringify(field('version')),
  __XPANDA_MSRV__: JSON.stringify(field('rust-version')),
};
