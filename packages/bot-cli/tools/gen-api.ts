import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  type Symbol as MorphSymbol,
  Project,
  SyntaxKind,
  type Type,
} from "ts-morph";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "../../..");
const botjsRoot = resolve(repoRoot, "packages/botjs");
const entryFile = resolve(botjsRoot, "src/index.ts");
const outputFile = resolve(botjsRoot, "generated/api.json");

const project = new Project({
  tsConfigFilePath: resolve(repoRoot, "tsconfig.base.json"),
});

project.addSourceFilesAtPaths(resolve(botjsRoot, "src/**/*.ts"));

const source = project.getSourceFileOrThrow(entryFile);

function getJsDocText(declaration: {
  getJsDocs?: () => { getDescription: () => string }[];
}) {
  if (!declaration.getJsDocs) return undefined;
  const docs = declaration.getJsDocs();
  const description = docs
    .map((doc) => doc.getDescription().trim())
    .filter(Boolean)
    .join("\n");
  return description.length > 0 ? description : undefined;
}

function parseDefaultFromText(text?: string): string | undefined {
  if (!text) return undefined;
  const match =
    text.match(/default:\s*([^\s,)\]]+)/i) ??
    text.match(/default\s*=\s*([^\s,)\]]+)/i);
  if (!match) return undefined;
  return match[1]?.replace(/^['"]|['"]$/g, "");
}

function parseEnumFromText(text?: string): string[] | undefined {
  if (!text) return undefined;
  const values = new Set<string>();
  for (const match of text.matchAll(/'([^']+)'/g)) {
    values.add(match[1]);
  }
  for (const match of text.matchAll(/"([^"]+)"/g)) {
    values.add(match[1]);
  }
  return values.size > 0 ? Array.from(values) : undefined;
}

function getLiteralUnionValues(
  type: Type,
): Array<string | number | boolean> | undefined {
  const unionTypes = type.getUnionTypes();
  if (unionTypes.length === 0) return undefined;
  const values: Array<string | number | boolean> = [];
  for (const unionType of unionTypes) {
    if (unionType.isStringLiteral()) {
      values.push(unionType.getLiteralValue() as string);
    } else if (unionType.isNumberLiteral()) {
      values.push(unionType.getLiteralValue() as number);
    } else if (unionType.isBooleanLiteral()) {
      values.push(unionType.getLiteralValue() as boolean);
    }
  }
  return values.length > 0 ? values : undefined;
}

function parseParamTags(
  declarations: MorphSymbol["getDeclarations"],
): Map<string, string> {
  const map = new Map<string, string>();
  const decls = declarations();
  for (const decl of decls) {
    if (decl.getKind() !== SyntaxKind.FunctionDeclaration) continue;
    const fnDecl = decl.asKindOrThrow(SyntaxKind.FunctionDeclaration);
    for (const doc of fnDecl.getJsDocs()) {
      for (const tag of doc.getTags()) {
        if (tag.getTagName() !== "param") continue;
        const rawText = tag.getText();
        const cleaned = rawText.replace(/^\s*\*?\s*@param\s+/, "").trim();
        const match = cleaned.match(/^(\[?\w+\]?)(?:\s+-\s+)?([\s\S]*)$/);
        if (!match) continue;
        const name = match[1].replace(/^\[/, "").replace(/\]$/, "");
        const description = match[2]?.trim();
        if (description) {
          map.set(name, description);
        }
      }
    }
  }
  return map;
}

function getReturnDocText(
  declarations: MorphSymbol["getDeclarations"],
): string | undefined {
  const decls = declarations();
  for (const decl of decls) {
    if (decl.getKind() !== SyntaxKind.FunctionDeclaration) continue;
    const fnDecl = decl.asKindOrThrow(SyntaxKind.FunctionDeclaration);
    for (const doc of fnDecl.getJsDocs()) {
      for (const tag of doc.getTags()) {
        if (tag.getTagName() !== "returns" && tag.getTagName() !== "return")
          continue;
        const text = tag
          .getText()
          .replace(/^\s*\*?\s*@returns?\s+/, "")
          .trim();
        if (text.length > 0) {
          return text;
        }
      }
    }
  }
  return undefined;
}

function getSignatureParams(symbol: MorphSymbol) {
  const declarations = symbol.getDeclarations();
  const paramDocs = parseParamTags(symbol.getDeclarations.bind(symbol));
  for (const decl of declarations) {
    if (decl.getKind() === SyntaxKind.FunctionDeclaration) {
      const fnDecl = decl.asKindOrThrow(SyntaxKind.FunctionDeclaration);
      return fnDecl.getParameters().map((param) => {
        const docText = paramDocs.get(param.getName());
        const unionValues = getLiteralUnionValues(param.getType());
        const docEnum = parseEnumFromText(docText);
        const combinedEnum = unionValues ?? docEnum;
        return {
          name: param.getName(),
          type: param.getType().getText(param),
          optional: param.isOptional(),
          default:
            param.getInitializer()?.getText() ?? parseDefaultFromText(docText),
          jsdoc: docText ?? getJsDocText(param),
          enum: combinedEnum,
        };
      });
    }
  }
  return [] as Array<{ name: string; type: string }>;
}

function getReturnType(symbol: MorphSymbol): string | null {
  const declarations = symbol.getDeclarations();
  for (const decl of declarations) {
    if (decl.getKind() === SyntaxKind.FunctionDeclaration) {
      const fnDecl = decl.asKindOrThrow(SyntaxKind.FunctionDeclaration);
      return fnDecl.getReturnType().getText(fnDecl);
    }
  }
  return null;
}

function isAsyncReturn(returnType: string | null): boolean {
  if (!returnType) return false;
  return returnType.startsWith("Promise<");
}

const exportSymbols = source.getExportSymbols();

const exports = exportSymbols.map((symbol) => {
  const name = symbol.getName();
  const resolved = symbol.isAlias()
    ? (symbol.getAliasedSymbol() ?? symbol)
    : symbol;
  const declarations = resolved.getDeclarations();
  const isClass = declarations.some(
    (decl) => decl.getKind() === SyntaxKind.ClassDeclaration,
  );
  const isFunction = declarations.some(
    (decl) => decl.getKind() === SyntaxKind.FunctionDeclaration,
  );
  const isInterface = declarations.some(
    (decl) => decl.getKind() === SyntaxKind.InterfaceDeclaration,
  );
  const isTypeAlias = declarations.some(
    (decl) => decl.getKind() === SyntaxKind.TypeAliasDeclaration,
  );
  const isVariable = declarations.some(
    (decl) => decl.getKind() === SyntaxKind.VariableDeclaration,
  );
  const sourceFile = declarations[0]?.getSourceFile().getBaseName();

  if (isClass) {
    return {
      name,
      kind: "class",
      jsdoc:
        declarations.length > 0 ? getJsDocText(declarations[0]) : undefined,
      sourceFile,
    };
  }

  if (isFunction) {
    const params = getSignatureParams(resolved);
    const returnType = getReturnType(resolved);
    const jsdoc =
      declarations.length > 0 ? getJsDocText(declarations[0]) : undefined;
    const returnsDoc = getReturnDocText(
      resolved.getDeclarations.bind(resolved),
    );
    return {
      name,
      kind: "function",
      params,
      returns: returnType ?? "unknown",
      async: isAsyncReturn(returnType),
      jsdoc,
      returnsDoc,
      sourceFile,
    };
  }

  if (isInterface) {
    return {
      name,
      kind: "interface",
      sourceFile,
    };
  }

  if (isTypeAlias) {
    return {
      name,
      kind: "type",
      sourceFile,
    };
  }

  if (isVariable) {
    const type = declarations[0]?.getType().getText(declarations[0]);
    return {
      name,
      kind: "value",
      type: type ?? "unknown",
      jsdoc:
        declarations.length > 0 ? getJsDocText(declarations[0]) : undefined,
      sourceFile,
    };
  }

  return {
    name,
    kind: "type",
    sourceFile,
  };
});

const payload = {
  source: {
    package: "@tego/botjs",
    entrypoint: "src/index.ts",
  },
  exports,
};

await mkdir(dirname(outputFile), { recursive: true });
await writeFile(outputFile, `${JSON.stringify(payload, null, 2)}\n`, "utf8");
