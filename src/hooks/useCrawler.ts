import { useState } from 'react';

export function useCrawler() {
    const [isCrawling, setIsCrawling] = useState(false);
    
    const startCrawl = async (url: string) => {
        setIsCrawling(true);
        console.log("Starting crawl for:", url);
        // Invoke Rust command here
        setIsCrawling(false);
    };

    return {
        isCrawling,
        startCrawl
    };
}
