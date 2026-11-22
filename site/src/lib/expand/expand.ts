import { ExpandError } from '@/lib/expand/error';
import { Evaluator, type Vars } from '@/lib/expand/evaluate';
import { parse } from '@/lib/expand/parse';

export type ExpandResult =
  { ok: true; text: string } | { ok: false; message: string; line: number; column: number };

function position(source: string, index: number) {
  const before = source.slice(0, index);
  const lines = before.split('\n');

  return { line: lines.length, column: [...lines[lines.length - 1]].length + 1 };
}

export function expand(source: string, vars: Vars): ExpandResult {
  try {
    return { ok: true, text: new Evaluator(vars).nodes(parse(source)) };
  } catch (error) {
    if (!(error instanceof ExpandError)) {
      throw error;
    }

    return { ok: false, message: error.message, ...position(source, error.index ?? 0) };
  }
}
