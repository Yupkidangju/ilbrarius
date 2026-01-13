import { useState } from "react";
import { useCrawler } from "./hooks/useCrawler";
import { Sidebar } from "./components/Sidebar";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const { trackUrl, finalUrl, isCrawling, error } = useCrawler();

  const handleTrack = async (e: React.FormEvent) => {
    e.preventDefault();
    if (url) {
      await trackUrl(url);
    }
  };

  return (
    <div className="flex h-screen w-full bg-[#1a1a1a] text-white">
      <Sidebar />
      
      <main className="flex-1 flex flex-col p-8">
        <h1 className="text-3xl font-bold mb-8">Librarius Explorer</h1>
        
        <div className="bg-gray-900 p-6 rounded-lg shadow-xl max-w-2xl">
          <form onSubmit={handleTrack} className="flex flex-col gap-4">
            <div className="flex flex-col gap-2">
              <label htmlFor="url-input" className="text-sm font-medium text-gray-400">
                Target URL (Supports 301/302 Tracking)
              </label>
              <input
                id="url-input"
                className="bg-gray-800 border border-gray-700 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                value={url}
                onChange={(e) => setUrl(e.currentTarget.value)}
                placeholder="https://example.com"
              />
            </div>
            
            <button 
              type="submit" 
              disabled={isCrawling}
              className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-bold py-2 px-4 rounded transition-colors"
            >
              {isCrawling ? "Tracking..." : "Analyze URL"}
            </button>
          </form>

          {error && (
            <div className="mt-4 p-3 bg-red-900/30 border border-red-500/50 rounded text-red-400 text-sm">
              Error: {error}
            </div>
          )}

          {finalUrl && (
            <div className="mt-6 p-4 bg-blue-900/20 border border-blue-500/30 rounded">
              <h3 className="text-sm font-semibold text-blue-400 mb-2">Final Destination:</h3>
              <p className="font-mono text-sm break-all">{finalUrl}</p>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
