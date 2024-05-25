'use client';

import { Editor } from '@/components/editor';
import { Terminal } from '@/components/terminal';
import { Binary, PlayIcon } from 'lucide-react';
import type { editor } from 'monaco-editor';
import init, { compile, decompile } from 'lina-wasm';
import { useRef, useState } from 'react';

export default function Home() {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

  const handleCompile = () => {
    const editor = editorRef.current;
    if (editor === null) return;
    const value = editor.getModel()?.getValue();
    if (value) init().then(() => compile(value));
  };

  const handleDecompile = () => {
    const editor = editorRef.current;
    if (editor === null) return;
    const value = editor.getModel()?.getValue();
    if (value) init().then(() => decompile(value));
  };

  const [terminal, setTerminal] = useState<string>('');

  const clearTerminal = () => {
    setTerminal('');
  };

  return (
    <main className="min-h-dvh mx-4 flex flex-col">
      <div className="flex py-4 gap-6">
        <button
          onClick={handleCompile}
          className="h-10 rounded-sm bg-indigo-500 px-2 text-sm hover:bg-indigo-600 whitespace-nowrap"
        >
          <PlayIcon className="inline mr-2 align-middle" />
          <span className="inline align-middle">EXECUTAR</span>
        </button>
        <button
          onClick={handleDecompile}
          className="h-10 rounded-sm bg-indigo-500 px-2 text-sm hover:bg-indigo-600 whitespace-nowrap"
        >
          <Binary className="inline mr-2 align-middle" />
          <span className="inline align-middle">DESCOMPILAR</span>
        </button>
        <button
          onClick={clearTerminal}
          className="h-10 rounded-sm bg-indigo-500 px-2 text-sm justify-end hover:bg-indigo-600 whitespace-nowrap"
        >
          <Binary className="inline mr-2 align-middle" />
          <span className="inline align-middle">Limpar</span>
        </button>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Editor
          editorRef={editorRef}
          className="min-h-[500px] h-[calc(100dvh-6.5rem)] border-4 border-indigo-500 rounded-sm"
        />
        <Terminal value={terminal} onChange={setTerminal} />
      </div>
    </main>
  );
}
