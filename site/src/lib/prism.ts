import { Prism } from 'prism-react-renderer';

// Prism's language files register themselves on a global Prism, and
// prism-react-renderer bundles neither Rust nor Bash.
(globalThis as { Prism?: typeof Prism }).Prism = Prism;
await import('prismjs/components/prism-rust');
await import('prismjs/components/prism-bash');

export { Prism };
