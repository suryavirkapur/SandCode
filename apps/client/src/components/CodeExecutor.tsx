"use client";
import React, { useState } from "react";

interface ExecutionResult {
  output: string;
  error: string;
}

const CodeExecutor: React.FC = () => {
  const [language, setLanguage] = useState<string>("python");
  const [code, setCode] = useState<string>("");
  const [input, setInput] = useState<string>("");
  const [result, setResult] = useState<ExecutionResult | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const executeCode = async () => {
    setIsLoading(true);
    try {
      const response = await fetch("http://localhost:8080/execute", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ language, code, input }),
      });

      if (!response.ok) {
        throw new Error("Network response was not ok");
      }

      const data: ExecutionResult = await response.json();
      setResult(data);
    } catch (error) {
      console.error("There was a problem with the fetch operation:", error);
      setResult({
        output: "",
        error: "Failed to execute code. Please try again.",
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-3xl font-bold mb-4">Code Executor</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {/* First row: Code and Language selection */}
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Language
            </label>
            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
            >
              <option value="python">Python</option>
              <option value="rust">Rust</option>
              <option value="cpp">C++</option>
              <option value="typescript">TypeScript</option>
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Code
            </label>
            <textarea
              value={code}
              onChange={(e) => setCode(e.target.value)}
              rows={10}
              className="mt-1 block w-full sm:text-sm border border-gray-300 rounded-md p-2 focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
        </div>

        {/* Second row: Input, Output, and Error */}
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Input
            </label>
            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              rows={3}
              className="mt-1 block w-full sm:text-sm border border-gray-300 rounded-md p-2 focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <button
            onClick={executeCode}
            disabled={isLoading}
            className="w-full inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
          >
            {isLoading ? "Executing..." : "Execute Code"}
          </button>
          {result && (
            <div>
              <h2 className="text-lg font-medium text-gray-900">Result:</h2>
              <pre className="mt-1 block w-full sm:text-sm border border-gray-300 rounded-md p-2 bg-gray-50 overflow-auto">
                {result.output}
              </pre>
              {result.error && (
                <pre className="mt-1 block w-full sm:text-sm border border-red-300 rounded-md p-2 bg-red-50 text-red-600 overflow-auto">
                  {result.error}
                </pre>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default CodeExecutor;
