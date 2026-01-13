import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function useCrawler() {
    const [isCrawling, setIsCrawling] = useState(false);
    const [finalUrl, setFinalUrl] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    
    const trackUrl = async (url: string) => {
        setIsCrawling(true);
        setError(null);
        try {
            const result: string = await invoke('track_url', { url });
            setFinalUrl(result);
            return result;
        } catch (err) {
            setError(err as string);
            console.error("Redirection error:", err);
        } finally {
            setIsCrawling(false);
        }
    };

    return {
        isCrawling,
        finalUrl,
        error,
        trackUrl
    };
}
